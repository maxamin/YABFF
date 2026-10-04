//! WAF-ban detector — see `docs/waf_detection.uml`.
//!
//! This is the small classifier that sits on top of feroxbuster's existing
//! rate/error-reaction machinery. It does **not** re-count requests or
//! re-derive thresholds: it consumes the tallies [`FeroxScan`] already keeps
//! and the [`PolicyTrigger`] that [`should_enforce_policy`] already produces,
//! then adds the one judgement the existing code does not make on its own —
//! *is this a transient hiccup or a sustained ban?* — plus vendor attribution
//! and a single explainable [`BanVerdict`] that maps back onto the existing
//! `tune()` / `bail()` / `cool_down()` reactions.
//!
//! [`should_enforce_policy`]: super::requester
//! [`FeroxScan`]: crate::scan_manager::FeroxScan

use reqwest::header::HeaderMap;

use super::PolicyTrigger;
use crate::config::RequesterPolicy;
use crate::scan_manager::FeroxScan;
use crate::HIGH_ERROR_RATIO;

/// 429-ratio threshold, mirroring `requester.rs` (`HIGH_ERROR_RATIO / 3.0`).
const RATE_LIMIT_RATIO: f64 = HIGH_ERROR_RATIO / 3.0;

/// How many consecutive enforcement intervals the *same* trigger must survive
/// (across feroxbuster's cooldowns) before a transient throttle is reclassified
/// as a ban. A 403 wall or a mid-scan block page still bans immediately; this
/// only governs the slow escalation of a mild/transient trigger (429s, a
/// transport-error ratio), so a host has to misbehave for this many rounds
/// running before AutoBail abandons it.
const SUSTAINED_INTERVALS: u32 = 10;

/// Where a target sits on the transient→ban spectrum.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub enum BanState {
    /// nothing wrong; `should_enforce_policy()` returned `None`
    #[default]
    Clear,
    /// 429s above threshold — slow down and honor `Retry-After`
    RateLimited,
    /// a first/mild trigger (errors or a sub-wall 403 rate) — tune down
    Throttled,
    /// sustained trigger, a 403 wall, or a block page mid-scan — a real ban
    Banned,
    /// the scan was stopped for this target (AutoBail)
    Bailed,
}

/// Which WAF/CDN vendor the edge looks like, inferred from headers/cookies that
/// feroxbuster already fetched. Attribution only — it never gates the verdict.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum WafVendor {
    Cloudflare,
    Akamai,
    AwsWafShield,
    Imperva,
    Sucuri,
    Fastly,
    ModSecurity,
    /// a generic WAF marker was present but not attributable to a known vendor
    Generic,
}

impl WafVendor {
    /// short, stable label for logs/evidence
    pub fn name(self) -> &'static str {
        match self {
            WafVendor::Cloudflare => "cloudflare",
            WafVendor::Akamai => "akamai",
            WafVendor::AwsWafShield => "aws-waf/shield",
            WafVendor::Imperva => "imperva/incapsula",
            WafVendor::Sucuri => "sucuri",
            WafVendor::Fastly => "fastly",
            WafVendor::ModSecurity => "modsecurity",
            WafVendor::Generic => "generic-waf",
        }
    }
}

/// The recommended action. The caller maps this onto controls feroxbuster
/// already owns (`Tune -> Requester::tune`, `Bail -> Requester::bail`,
/// `Backoff -> Requester::cool_down` honoring `Retry-After`).
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum WafReaction {
    /// proceed unchanged
    Continue,
    /// reduce request rate (AutoTune path)
    Tune,
    /// pause; `retry_after` seconds if the server told us, else caller's backoff
    Backoff { retry_after: Option<u64> },
    /// stop scanning this target (AutoBail path); the sweep continues
    Bail,
}

/// A read-only snapshot derived from existing tallies — this struct adds no new
/// counters, it just packages what [`FeroxScan`] and the response headers
/// already expose so [`WafBanDetector::classify`] can reason over it.
#[derive(Clone, Debug, Default)]
pub struct BanSignals {
    /// requests made so far for this scan
    pub requests: u64,
    /// fraction of requests answered 403 (`status_403s / requests`)
    pub ratio_403: f64,
    /// fraction of requests answered 429 (`status_429s / requests`)
    pub ratio_429: f64,
    /// general error count (timeouts/conn/TLS) for this scan
    pub error_count: usize,
    /// `Retry-After` seconds parsed from a 429/503 response, if present
    pub retry_after: Option<u64>,
    /// a new wildcard/soft-404 block page appeared *after* warm-up
    pub new_wildcard_midscan: bool,
    /// transport-level errors (resets/timeouts/TLS) are spiking
    pub transport_spike: bool,
}

impl BanSignals {
    /// Pure constructor — ratios computed here so the math is unit-testable
    /// without a live [`FeroxScan`]. A zero request count yields zero ratios
    /// (no division by zero, no false signal before warm-up).
    pub fn new(requests: u64, n403: usize, n429: usize, errors: usize) -> Self {
        let (ratio_403, ratio_429) = if requests == 0 {
            (0.0, 0.0)
        } else {
            let r = requests as f64;
            (n403 as f64 / r, n429 as f64 / r)
        };
        Self {
            requests,
            ratio_403,
            ratio_429,
            error_count: errors,
            retry_after: None,
            new_wildcard_midscan: false,
            transport_spike: false,
        }
    }

    /// Build from the scan's own tallies — the feroxbuster glue. Everything
    /// here is an accessor feroxbuster already exposes.
    pub fn from_scan(scan: &FeroxScan) -> Self {
        Self::new(
            scan.requests(),
            scan.num_errors(PolicyTrigger::Status403),
            scan.num_errors(PolicyTrigger::Status429),
            scan.num_errors(PolicyTrigger::Errors),
        )
    }

    /// builder: attach a parsed `Retry-After`
    pub fn with_retry_after(mut self, secs: Option<u64>) -> Self {
        self.retry_after = secs;
        self
    }

    /// builder: mark that a block page appeared mid-scan (from HeuristicTests)
    pub fn with_block_page(mut self, yes: bool) -> Self {
        self.new_wildcard_midscan = yes;
        self
    }

    /// builder: mark a transport-error spike (from Stats)
    pub fn with_transport_spike(mut self, yes: bool) -> Self {
        self.transport_spike = yes;
        self
    }
}

/// The verdict emitted per enforcement checkpoint.
#[derive(Clone, Debug)]
pub struct BanVerdict {
    pub state: BanState,
    /// the feroxbuster trigger that drove this (reused, not re-derived)
    pub trigger: Option<PolicyTrigger>,
    /// 0.0–1.0 aggregate confidence that we are being flagged
    pub confidence: f64,
    pub vendor: Option<WafVendor>,
    /// how many consecutive intervals the current trigger has held
    pub persisted_intervals: u32,
    /// human-readable reasons, for logs and `--verbosity`
    pub evidence: Vec<String>,
    pub recommendation: WafReaction,
}

/// Infers a WAF vendor from headers/cookies already present on a response.
/// Pure and data-driven; add a vendor by adding a match arm.
#[derive(Default)]
pub struct VendorAttributor;

impl VendorAttributor {
    pub fn new() -> Self {
        Self
    }

    /// Case-insensitive header lookup → lowercased value.
    fn header<'a>(headers: &'a HeaderMap, name: &str) -> Option<String> {
        headers
            .get(name)
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_lowercase())
    }

    /// True if any `set-cookie` value contains `needle`.
    fn cookie_contains(headers: &HeaderMap, needle: &str) -> bool {
        headers
            .get_all("set-cookie")
            .iter()
            .filter_map(|v| v.to_str().ok())
            .any(|c| c.to_lowercase().contains(needle))
    }

    /// Best-effort vendor identification; `None` when no WAF markers are seen.
    pub fn identify(&self, headers: &HeaderMap) -> Option<WafVendor> {
        let server = Self::header(headers, "server").unwrap_or_default();

        // Cloudflare: cf-ray / server / __cf_bm | cf_clearance cookies
        if headers.contains_key("cf-ray")
            || headers.contains_key("cf-mitigated")
            || server.contains("cloudflare")
            || Self::cookie_contains(headers, "__cf_bm")
            || Self::cookie_contains(headers, "cf_clearance")
        {
            return Some(WafVendor::Cloudflare);
        }

        // Akamai: x-akamai-* headers / akamaighost server
        if server.contains("akamaighost")
            || server.contains("akamai")
            || headers.keys().any(|k| k.as_str().starts_with("x-akamai"))
        {
            return Some(WafVendor::Akamai);
        }

        // Imperva / Incapsula: x-iinfo / x-cdn: Incapsula / incap_ses cookie
        if headers.contains_key("x-iinfo")
            || Self::header(headers, "x-cdn").is_some_and(|v| v.contains("incapsula"))
            || Self::cookie_contains(headers, "incap_ses")
            || Self::cookie_contains(headers, "visid_incap")
        {
            return Some(WafVendor::Imperva);
        }

        // Sucuri: x-sucuri-* headers / sucuri server
        if headers.contains_key("x-sucuri-id")
            || headers.contains_key("x-sucuri-cache")
            || server.contains("sucuri")
        {
            return Some(WafVendor::Sucuri);
        }

        // Fastly: x-served-by/x-fastly / fastly server
        if server.contains("fastly")
            || headers.keys().any(|k| k.as_str().starts_with("x-fastly"))
            || Self::header(headers, "x-served-by").is_some_and(|v| v.contains("fastly"))
        {
            return Some(WafVendor::Fastly);
        }

        // AWS WAF / Shield / ALB: x-amzn-* request ids, awselb server
        if server.contains("awselb")
            || headers.contains_key("x-amzn-requestid")
            || headers.keys().any(|k| k.as_str().starts_with("x-amzn-waf"))
            || headers.contains_key("x-amz-cf-id")
        {
            return Some(WafVendor::AwsWafShield);
        }

        // ModSecurity: server banner is the usual (only) header tell
        if server.contains("mod_security") || server.contains("modsecurity") {
            return Some(WafVendor::ModSecurity);
        }

        // generic WAF markers
        if headers.contains_key("x-waf-event-info")
            || headers.keys().any(|k| {
                let k = k.as_str();
                k.contains("waf") || k.contains("firewall")
            })
        {
            return Some(WafVendor::Generic);
        }

        None
    }
}

/// Build a reqwest [`HeaderMap`] from a plain `String` map (lossy: entries whose
/// name or value isn't valid HTTP are skipped). Lets callers whose responses
/// carry headers as a `HashMap<String, String>` — e.g. the --ml-loop in-process
/// runner's `FeroxResponse` — reuse the same vendor attribution and parsing.
pub fn header_map_from(pairs: &std::collections::HashMap<String, String>) -> HeaderMap {
    use reqwest::header::{HeaderName, HeaderValue};
    let mut h = HeaderMap::new();
    for (k, v) in pairs {
        if let (Ok(name), Ok(val)) = (
            HeaderName::from_bytes(k.as_bytes()),
            HeaderValue::from_str(v),
        ) {
            h.append(name, val);
        }
    }
    h
}

/// Parse a `Retry-After` header as integer seconds (HTTP-date form is ignored;
/// the caller falls back to its own backoff when this is `None`).
pub fn parse_retry_after(headers: &HeaderMap) -> Option<u64> {
    headers
        .get("retry-after")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.trim().parse::<u64>().ok())
}

/// The stateful detector. One per scan/target. Cheap, single-threaded; the
/// caller owns synchronization (feroxbuster already serializes enforcement via
/// `PolicyData.cooling_down`).
pub struct WafBanDetector {
    policy: RequesterPolicy,
    state: BanState,
    last_trigger: Option<PolicyTrigger>,
    consecutive_triggers: u32,
    vendor: Option<WafVendor>,
    last_retry_after: Option<u64>,
    attributor: VendorAttributor,
}

impl WafBanDetector {
    /// Create a detector honoring the run's `RequesterPolicy`.
    pub fn new(policy: RequesterPolicy) -> Self {
        Self {
            policy,
            state: BanState::Clear,
            last_trigger: None,
            consecutive_triggers: 0,
            vendor: None,
            last_retry_after: None,
            attributor: VendorAttributor::new(),
        }
    }

    /// the most recent `Retry-After` seen (seconds), if any
    pub fn last_retry_after(&self) -> Option<u64> {
        self.last_retry_after
    }

    /// current state (last classification)
    pub fn state(&self) -> BanState {
        self.state
    }

    /// attributed vendor, once seen
    pub fn vendor(&self) -> Option<WafVendor> {
        self.vendor
    }

    /// Feed response headers for attribution + `Retry-After`; call per response.
    /// Once a vendor is identified it sticks (edges are consistent per host).
    /// Returns any parsed `Retry-After` so the caller can fold it into signals.
    pub fn observe_headers(&mut self, headers: &HeaderMap) -> Option<u64> {
        if self.vendor.is_none() {
            self.vendor = self.attributor.identify(headers);
        }
        let retry = parse_retry_after(headers);
        if retry.is_some() {
            self.last_retry_after = retry;
        }
        retry
    }

    /// The decision. Takes the trigger feroxbuster's `should_enforce_policy()`
    /// already produced (or `None`) plus a signal snapshot, updates persistence,
    /// and returns the verdict + recommended reaction.
    pub fn classify(&mut self, sig: &BanSignals, trigger: Option<PolicyTrigger>) -> BanVerdict {
        self.update_persistence(trigger);

        let wall_403 = sig.ratio_403 >= HIGH_ERROR_RATIO;
        let heavy_429 = sig.ratio_429 >= RATE_LIMIT_RATIO;
        let sustained = self.consecutive_triggers >= SUSTAINED_INTERVALS;
        let block_page = sig.new_wildcard_midscan;

        let mut evidence = Vec::new();
        if wall_403 {
            // clamp for display: feroxbuster's error tally can briefly run ahead
            // of the progress-bar request count, pushing the raw ratio past 1.0.
            evidence.push(format!(
                "403 wall: {:.0}% of {} requests",
                (sig.ratio_403.min(1.0)) * 100.0,
                sig.requests
            ));
        }
        if heavy_429 {
            evidence.push(format!(
                "rate-limited: {:.0}% 429",
                (sig.ratio_429.min(1.0)) * 100.0
            ));
        }
        if sustained {
            evidence.push(format!(
                "trigger persisted {} intervals",
                self.consecutive_triggers
            ));
        }
        if block_page {
            evidence.push("block page appeared mid-scan".to_string());
        }
        if sig.transport_spike {
            evidence.push("transport-error spike".to_string());
        }
        if let Some(v) = self.vendor {
            evidence.push(format!("vendor: {}", v.name()));
        }

        // severity state from the (reused) trigger + ban heuristics
        let mut state = match trigger {
            None => BanState::Clear,
            Some(t) => {
                let ban = wall_403 || block_page || sustained;
                if ban {
                    BanState::Banned
                } else if t == PolicyTrigger::Status429 {
                    BanState::RateLimited
                } else {
                    BanState::Throttled
                }
            }
        };

        let confidence = self.confidence(sig, trigger.is_some());
        let recommendation = self.reaction_for(state, sig.retry_after);

        // a Bail reaction means we are abandoning the target
        if recommendation == WafReaction::Bail {
            state = BanState::Bailed;
        }
        self.state = state;

        BanVerdict {
            state,
            trigger,
            confidence,
            vendor: self.vendor,
            persisted_intervals: self.consecutive_triggers,
            evidence,
            recommendation,
        }
    }

    /// Count how long the *same* trigger has held across checkpoints.
    fn update_persistence(&mut self, trigger: Option<PolicyTrigger>) {
        match trigger {
            Some(t) if self.last_trigger == Some(t) => self.consecutive_triggers += 1,
            Some(t) => {
                self.consecutive_triggers = 1;
                self.last_trigger = Some(t);
            }
            None => {
                self.consecutive_triggers = 0;
                self.last_trigger = None;
            }
        }
    }

    /// Aggregate several weak signals into one 0–1 score.
    fn confidence(&self, sig: &BanSignals, triggered: bool) -> f64 {
        if !triggered {
            return 0.0;
        }
        let mut c = sig.ratio_403;
        c = c.max((sig.ratio_429 / RATE_LIMIT_RATIO).min(1.0));
        c = c.max((self.consecutive_triggers as f64 / SUSTAINED_INTERVALS as f64).min(1.0));
        if sig.new_wildcard_midscan {
            c = c.max(0.9);
        }
        if sig.transport_spike {
            c = c.max(0.5);
        }
        if self.vendor.is_some() {
            c = (c + 0.1).min(1.0);
        }
        c.clamp(0.0, 1.0)
    }

    /// Map (state, policy) onto the existing feroxbuster reactions.
    fn reaction_for(&self, state: BanState, retry_after: Option<u64>) -> WafReaction {
        match state {
            BanState::Clear => WafReaction::Continue,
            BanState::RateLimited => WafReaction::Backoff { retry_after },
            BanState::Throttled => match self.policy {
                RequesterPolicy::AutoTune => WafReaction::Tune,
                // AutoBail abandons a target only on a confirmed ban, not a mild
                // or transient trigger — back off and keep scanning instead.
                RequesterPolicy::AutoBail => WafReaction::Backoff { retry_after },
                RequesterPolicy::Default => WafReaction::Continue,
            },
            BanState::Banned => match self.policy {
                RequesterPolicy::AutoBail => WafReaction::Bail,
                // AutoTune/Default can't un-ban by tuning; back off and let the
                // caller decide whether to keep going.
                _ => WafReaction::Backoff { retry_after },
            },
            BanState::Bailed => WafReaction::Bail,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

    fn headers(pairs: &[(&str, &str)]) -> HeaderMap {
        let mut h = HeaderMap::new();
        for (k, v) in pairs {
            h.append(
                HeaderName::from_bytes(k.as_bytes()).unwrap(),
                HeaderValue::from_str(v).unwrap(),
            );
        }
        h
    }

    // ---- BanSignals math ------------------------------------------------

    #[test]
    fn signals_ratios_are_computed() {
        let s = BanSignals::new(100, 90, 30, 10);
        assert!((s.ratio_403 - 0.90).abs() < 1e-9);
        assert!((s.ratio_429 - 0.30).abs() < 1e-9);
        assert_eq!(s.error_count, 10);
    }

    #[test]
    fn signals_zero_requests_no_divide_by_zero() {
        let s = BanSignals::new(0, 0, 0, 0);
        assert_eq!(s.ratio_403, 0.0);
        assert_eq!(s.ratio_429, 0.0);
    }

    // ---- vendor attribution --------------------------------------------

    #[test]
    fn attributes_cloudflare() {
        let a = VendorAttributor::new();
        assert_eq!(
            a.identify(&headers(&[("cf-ray", "7d2f-LHR"), ("server", "cloudflare")])),
            Some(WafVendor::Cloudflare)
        );
        assert_eq!(
            a.identify(&headers(&[("set-cookie", "__cf_bm=abc; path=/")])),
            Some(WafVendor::Cloudflare)
        );
    }

    #[test]
    fn attributes_each_known_vendor() {
        let a = VendorAttributor::new();
        let cases = [
            (vec![("server", "AkamaiGHost")], WafVendor::Akamai),
            (vec![("x-iinfo", "9-1-2")], WafVendor::Imperva),
            (vec![("x-sucuri-id", "12")], WafVendor::Sucuri),
            (vec![("x-served-by", "cache-fastly")], WafVendor::Fastly),
            (vec![("x-amzn-requestid", "r1")], WafVendor::AwsWafShield),
            (vec![("server", "Mod_Security")], WafVendor::ModSecurity),
            (vec![("x-waf-event-info", "blocked")], WafVendor::Generic),
        ];
        for (hdrs, want) in cases {
            assert_eq!(a.identify(&headers(&hdrs)), Some(want), "for {hdrs:?}");
        }
    }

    #[test]
    fn no_markers_no_vendor() {
        let a = VendorAttributor::new();
        assert_eq!(a.identify(&headers(&[("server", "nginx")])), None);
    }

    #[test]
    fn parses_retry_after_integer_only() {
        assert_eq!(parse_retry_after(&headers(&[("retry-after", "120")])), Some(120));
        assert_eq!(
            parse_retry_after(&headers(&[("retry-after", "Wed, 21 Oct 2025 07:28:00 GMT")])),
            None
        );
        assert_eq!(parse_retry_after(&headers(&[])), None);
    }

    // ---- classification -------------------------------------------------

    #[test]
    fn no_trigger_is_clear_and_continue() {
        let mut d = WafBanDetector::new(RequesterPolicy::AutoTune);
        let v = d.classify(&BanSignals::new(100, 0, 0, 0), None);
        assert_eq!(v.state, BanState::Clear);
        assert_eq!(v.recommendation, WafReaction::Continue);
        assert_eq!(v.confidence, 0.0);
    }

    #[test]
    fn four_oh_three_wall_is_a_ban() {
        let mut d = WafBanDetector::new(RequesterPolicy::AutoBail);
        let sig = BanSignals::new(100, 95, 0, 0); // 95% 403 >= HIGH_ERROR_RATIO
        let v = d.classify(&sig, Some(PolicyTrigger::Status403));
        assert_eq!(v.state, BanState::Bailed); // AutoBail promotes ban -> bailed
        assert_eq!(v.recommendation, WafReaction::Bail);
        assert!(v.confidence >= 0.9);
        assert!(v.evidence.iter().any(|e| e.contains("403 wall")));
    }

    #[test]
    fn four_oh_three_wall_without_autobail_backs_off() {
        let mut d = WafBanDetector::new(RequesterPolicy::AutoTune);
        let v = d.classify(&BanSignals::new(100, 95, 0, 0), Some(PolicyTrigger::Status403));
        assert_eq!(v.state, BanState::Banned);
        assert_eq!(v.recommendation, WafReaction::Backoff { retry_after: None });
    }

    #[test]
    fn rate_limited_backs_off_and_honors_retry_after() {
        let mut d = WafBanDetector::new(RequesterPolicy::AutoTune);
        let sig = BanSignals::new(100, 0, 40, 0).with_retry_after(Some(30));
        let v = d.classify(&sig, Some(PolicyTrigger::Status429));
        assert_eq!(v.state, BanState::RateLimited);
        assert_eq!(v.recommendation, WafReaction::Backoff { retry_after: Some(30) });
    }

    #[test]
    fn mild_error_trigger_throttles_under_autotune() {
        let mut d = WafBanDetector::new(RequesterPolicy::AutoTune);
        let v = d.classify(&BanSignals::new(100, 0, 0, 30), Some(PolicyTrigger::Errors));
        assert_eq!(v.state, BanState::Throttled);
        assert_eq!(v.recommendation, WafReaction::Tune);
    }

    #[test]
    fn autobail_backs_off_on_transient_but_bails_on_ban() {
        let mut d = WafBanDetector::new(RequesterPolicy::AutoBail);
        // transient 429 (not a ban): back off, don't abandon the target
        let v = d.classify(&BanSignals::new(100, 0, 40, 0), Some(PolicyTrigger::Status429));
        assert_eq!(v.state, BanState::RateLimited);
        assert_eq!(v.recommendation, WafReaction::Backoff { retry_after: None });
        // a mild 403 rate (sub-wall) under AutoBail also just backs off, not bail
        let v = d.classify(&BanSignals::new(100, 10, 0, 0), Some(PolicyTrigger::Status403));
        assert_eq!(v.state, BanState::Throttled);
        assert!(matches!(v.recommendation, WafReaction::Backoff { .. }));
        // but a 403 wall is a ban -> bail
        let v = d.classify(&BanSignals::new(100, 95, 0, 0), Some(PolicyTrigger::Status403));
        assert_eq!(v.state, BanState::Bailed);
        assert_eq!(v.recommendation, WafReaction::Bail);
    }

    #[test]
    fn mild_trigger_under_default_policy_does_nothing() {
        let mut d = WafBanDetector::new(RequesterPolicy::Default);
        let v = d.classify(&BanSignals::new(100, 0, 0, 30), Some(PolicyTrigger::Errors));
        assert_eq!(v.state, BanState::Throttled);
        assert_eq!(v.recommendation, WafReaction::Continue);
    }

    #[test]
    fn persistent_mild_trigger_escalates_to_ban() {
        let mut d = WafBanDetector::new(RequesterPolicy::AutoTune);
        let sig = BanSignals::new(100, 10, 0, 30); // sub-wall, mild
        // the first SUSTAINED_INTERVALS-1 intervals stay throttled...
        for i in 1..SUSTAINED_INTERVALS {
            let v = d.classify(&sig, Some(PolicyTrigger::Errors));
            assert_eq!(v.state, BanState::Throttled, "interval {i} should still throttle");
        }
        // ...and the SUSTAINED_INTERVALS-th consecutive identical trigger bans.
        let v = d.classify(&sig, Some(PolicyTrigger::Errors));
        assert_eq!(v.state, BanState::Banned);
        assert_eq!(v.persisted_intervals, SUSTAINED_INTERVALS);
        assert!(v.evidence.iter().any(|e| e.contains("persisted")));
    }

    #[test]
    fn a_clear_interval_resets_persistence() {
        let mut d = WafBanDetector::new(RequesterPolicy::AutoTune);
        let sig = BanSignals::new(100, 10, 0, 30);
        d.classify(&sig, Some(PolicyTrigger::Errors));
        d.classify(&sig, Some(PolicyTrigger::Errors));
        d.classify(&BanSignals::new(100, 0, 0, 0), None); // recovered
        let v = d.classify(&sig, Some(PolicyTrigger::Errors));
        assert_eq!(v.persisted_intervals, 1);
        assert_eq!(v.state, BanState::Throttled);
    }

    #[test]
    fn midscan_block_page_is_a_ban_immediately() {
        let mut d = WafBanDetector::new(RequesterPolicy::AutoTune);
        let sig = BanSignals::new(100, 20, 0, 0).with_block_page(true);
        let v = d.classify(&sig, Some(PolicyTrigger::Status403));
        assert_eq!(v.state, BanState::Banned);
        assert!(v.confidence >= 0.9);
        assert!(v.evidence.iter().any(|e| e.contains("block page")));
    }

    #[test]
    fn vendor_is_folded_into_verdict() {
        let mut d = WafBanDetector::new(RequesterPolicy::AutoTune);
        d.observe_headers(&headers(&[("cf-ray", "1-2")]));
        let v = d.classify(&BanSignals::new(100, 95, 0, 0), Some(PolicyTrigger::Status403));
        assert_eq!(v.vendor, Some(WafVendor::Cloudflare));
        assert!(v.evidence.iter().any(|e| e.contains("cloudflare")));
    }
}
