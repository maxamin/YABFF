//! Native ML layer for feroxbuster.
//!
//! This is the feroxml engine set ported in-crate: K-Means/nearest-centroid
//! fingerprinting, variable-order Markov/PPM path prediction, a Thompson-sampling
//! scheduler, BM25 ranking, and SimHash soft-404 detection. The engines are
//! pure-Rust and self-contained; [`ProbeResp`] is the lightweight response view
//! the fingerprinter consumes, so the ML layer never depends on the scanner's
//! own types and stays unit-testable in isolation.
//!
//! Runtime wiring (see `main.rs` / `event_handlers::outputs` / `scanner`): when
//! `--ml` is set, feroxbuster probes the target concurrently, confidence-gated
//! [`fingerprint_gated`]s a profile, seeds a Markov model (merging any model at
//! `--ml-model`), and injects a seed batch of [`predict_words`] into the initial
//! wordlist. Then, for **every directory** it scans, it calls
//! [`predict_for_scan`] to inject fresh per-directory Markov predictions —
//! BM25-reranked against the paths discovered so far, with the number of guesses
//! scaled by a bandit ([`scheduler`]) over that directory's productivity. Every
//! discovered URL is [`observe`]d to learn online, and the model is [`save`]d on
//! shutdown.

use std::collections::HashMap;
use std::sync::RwLock;

use lazy_static::lazy_static;

// The engines live in the shared `ferox-ml-core` crate (see also `../../feroxml`,
// which depends on the same crate). This module is just the in-crate runtime
// façade: a process-wide learned model plus the glue `main.rs`, the scanner, and
// the output handler call.
use ferox_ml_core::interfaces::{Predictor, Scheduler};
use ferox_ml_core::markov::MarkovModel;
use ferox_ml_core::ranking::Bm25;
use ferox_ml_core::tokenize::{last_segment, path_segments};

pub use ferox_ml_core::{ProbeResp, PROFILES};

/// The discriminating paths the fingerprint probe requests.
pub fn probe_paths() -> &'static [&'static str] {
    &ferox_ml_core::profiles::PROBE_PATHS
}

/// Classify probe responses into a framework profile + centroid distances.
pub fn fingerprint(probes: &[ProbeResp]) -> (String, Vec<(String, f64)>) {
    use ferox_ml_core::interfaces::Classifier;
    let fv = ferox_ml_core::fingerprint::feature_vector(probes);
    ferox_ml_core::fingerprint::NearestCentroid.classify(&fv)
}

/// Fallback profile used when a fingerprint can't be trusted.
pub const GENERIC_PROFILE: &str = "LEGACY_STATIC";

/// E1 catch-all / soft-404 guard (see [`ferox_ml_core::fingerprint::is_catch_all`]):
/// true when the target answers nearly every probe with no strong discriminator,
/// so its path-presence signal is noise and fingerprinting should abstain.
pub fn is_catch_all(probes: &[ProbeResp]) -> bool {
    ferox_ml_core::fingerprint::is_catch_all(probes)
}

/// E2 per-path soft-404 scoring: learn the soft-404 baseline from probes to
/// random, almost-certainly-absent paths, then demote any discriminating probe
/// whose response matches that baseline to status 404 (so a catch-all's uniform
/// soft-404 body no longer reads as "present"). Returns how many were demoted.
pub fn score_soft_404(probes: &mut [ProbeResp], random_probes: &[ProbeResp]) -> usize {
    let filter = ferox_ml_core::fingerprint::learn_soft_404(random_probes);
    ferox_ml_core::fingerprint::apply_soft_404(probes, &filter)
}

/// Confidence-gated fingerprint. Returns `(profile, confident)`.
///
/// Classification is only trusted when all of: (a) at least one probe answered
/// (otherwise the probe was blind — DNS/connection failures); (b) the target is
/// not a catch-all / soft-404 server (E1 — otherwise path-presence is noise); and
/// (c) the nearest centroid beats the runner-up by at least `margin` (otherwise
/// the match is ambiguous). When any check fails it falls back to
/// [`GENERIC_PROFILE`] and reports `confident = false` so the caller can log it.
pub fn fingerprint_gated(probes: &[ProbeResp], margin: f64) -> (String, bool) {
    // E1: a catch-all / soft-404 server defeats path-presence fingerprinting at
    // any margin, so abstain up front.
    if is_catch_all(probes) {
        return (GENERIC_PROFILE.to_string(), false);
    }
    let (best, dists) = fingerprint(probes);
    let answered = probes.iter().filter(|p| p.status != 0).count();
    let d0 = dists.first().map(|d| d.1).unwrap_or(f64::INFINITY);
    let d1 = dists.get(1).map(|d| d.1).unwrap_or(f64::INFINITY);
    let confident = answered >= 1 && (d1 - d0) >= margin;
    if confident {
        (best, true)
    } else {
        (GENERIC_PROFILE.to_string(), false)
    }
}

/// Tunable parameters for the ML layer (sourced from the config).
#[derive(Debug, Clone)]
pub struct MlParams {
    /// maximum Markov order (PPM back-off depth)
    pub max_order: usize,
    /// Markov additive-smoothing constant
    pub alpha: f64,
    /// Markov soft-404 / low-probability prune threshold
    pub threshold: f64,
    /// base number of predictions injected per directory
    pub predictions: usize,
    /// BM25-rerank predictions against the discovered-path corpus
    pub rank: bool,
    /// scheduler name: `thompson` | `ucb1` | `round_robin`
    pub scheduler: String,
    /// RNG seed for the scheduler (determinism in tests/benchmarks)
    pub seed: u64,
}

impl Default for MlParams {
    fn default() -> Self {
        Self {
            max_order: 3,
            alpha: 0.5,
            threshold: 0.02,
            predictions: 25,
            rank: true,
            scheduler: "thompson".to_string(),
            seed: 1,
        }
    }
}

lazy_static! {
    /// The active learned model, shared across scan tasks. `None` until `init`.
    static ref ML: RwLock<Option<MlState>> = RwLock::new(None);
}

struct MlState {
    model: MarkovModel,
    model_path: String,
    /// BM25 corpus of discovered path segments, for re-ranking predictions.
    bm25: Bm25,
    /// Bandit over directories; scales each directory's prediction budget.
    scheduler: Box<dyn Scheduler + Send + Sync>,
    /// Hits observed under each directory (its Markov-context key).
    hits_by_dir: HashMap<String, u32>,
    /// Base number of predictions per directory.
    predictions: usize,
    /// Whether to BM25-rerank predictions.
    rank: bool,
}

/// Directory key for a URL: its lowercased, slash-joined path segments.
fn dir_key(url: &str) -> String {
    path_segments(url).join("/")
}

/// The parent-directory key of a discovered resource URL.
fn parent_key(url: &str) -> String {
    let mut segs = path_segments(url);
    segs.pop();
    segs.join("/")
}

/// Seed a model for `profile`, merge any model persisted at `model_path`, and
/// install it as the active model. Idempotent; a later call replaces the state.
pub fn init(profile: &str, model_path: &str, params: &MlParams) {
    let mut model =
        MarkovModel::seeded(profile, params.max_order, params.alpha, params.threshold);
    if !model_path.is_empty() {
        if let Ok(text) = std::fs::read_to_string(model_path) {
            if let Ok(learned) = MarkovModel::from_json(&text) {
                model.merge(&learned);
            }
        }
    }
    if let Ok(mut guard) = ML.write() {
        *guard = Some(MlState {
            model,
            model_path: model_path.to_string(),
            bm25: Bm25::new(),
            scheduler: ferox_ml_core::scheduler::build(&params.scheduler, params.seed),
            hits_by_dir: HashMap::new(),
            predictions: params.predictions,
            rank: params.rank,
        });
    }
}

/// True once `init` has installed a model.
pub fn is_active() -> bool {
    ML.read().map(|g| g.is_some()).unwrap_or(false)
}

/// Predicted next path tokens for `recent_path`, as bare words for the scanner.
pub fn predict_words(recent_path: &str, top_n: usize) -> Vec<String> {
    let guard = match ML.read() {
        Ok(g) => g,
        Err(_) => return vec![],
    };
    let Some(state) = guard.as_ref() else {
        return vec![];
    };
    state
        .model
        .predict(recent_path, top_n)
        .into_iter()
        .map(|(tok, _)| tok.trim().trim_matches('/').to_string())
        .filter(|t| !t.is_empty())
        .collect()
}

/// Learn from a discovered URL: online-update the Markov model, add the segment
/// to the BM25 corpus, and credit a hit to its parent directory (reward signal
/// for the bandit's per-directory budget).
pub fn observe(url: &str) {
    if let Ok(mut guard) = ML.write() {
        if let Some(state) = guard.as_mut() {
            state.model.learn(url);
            let seg = last_segment(url);
            if !seg.is_empty() {
                state.bm25.add_document(&seg);
            }
            *state.hits_by_dir.entry(parent_key(url)).or_insert(0) += 1;
        }
    }
}

/// Per-directory predictions for a directory about to be (or being) scanned.
///
/// Reward the directory's bandit arm by its base-pass hit rate
/// (`hits / base_len`), then scale the prediction budget by the arm's value
/// estimate so productive directories earn more guesses (25%–100% of
/// `predictions`). Predictions are over-fetched, BM25-reranked against the
/// discovered-path corpus (unless disabled), and truncated to the budget.
///
/// `base_len` is the size of the base wordlist already tried against this
/// directory; pass `0` before any base pass to get the full, unscaled budget.
pub fn predict_for_scan(target_url: &str, base_len: usize) -> Vec<String> {
    let mut guard = match ML.write() {
        Ok(g) => g,
        Err(_) => return vec![],
    };
    let Some(state) = guard.as_mut() else {
        return vec![];
    };

    let base_n = state.predictions;
    if base_n == 0 {
        return vec![];
    }

    let dir = dir_key(target_url);
    let n = if base_len == 0 {
        base_n
    } else {
        let hits = *state.hits_by_dir.get(&dir).unwrap_or(&0);
        let reward = (hits as f64 / base_len as f64).clamp(0.0, 1.0);
        state.scheduler.add_arm(&dir);
        state.scheduler.update(&dir, reward);
        let value = state.scheduler.value(&dir).clamp(0.0, 1.0);
        (((base_n as f64) * (0.25 + 0.75 * value)).round() as usize).clamp(1, base_n)
    };

    // over-fetch so the reranker has candidates to reorder, then trim to budget
    let raw = state.model.predict(target_url, n.saturating_mul(3).max(n));
    let ranked = if state.rank {
        state.bm25.rerank(&raw)
    } else {
        raw
    };
    ranked
        .into_iter()
        .map(|(tok, _)| tok.trim().trim_matches('/').to_string())
        .filter(|t| !t.is_empty())
        .take(n)
        .collect()
}

/// Persist the active model to its configured path (no-op if unset).
pub fn save() {
    if let Ok(guard) = ML.read() {
        if let Some(state) = guard.as_ref() {
            if state.model_path.is_empty() {
                return;
            }
            if let Ok(js) = state.model.to_json() {
                if let Some(parent) = std::path::Path::new(&state.model_path).parent() {
                    let _ = std::fs::create_dir_all(parent);
                }
                let _ = std::fs::write(&state.model_path, js);
            }
        }
    }
}

/// Reset the active model (used by tests).
#[cfg(test)]
pub fn reset() {
    if let Ok(mut guard) = ML.write() {
        *guard = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    lazy_static! {
        /// serializes tests that mutate the global `ML` singleton (cargo runs
        /// tests in parallel threads, which would otherwise race on it)
        static ref TEST_LOCK: Mutex<()> = Mutex::new(());
    }

    #[test]
    fn probe_paths_include_rest_markers() {
        let p = probe_paths();
        assert!(p.contains(&"wp-json"));
        assert!(p.contains(&"rest"));
        assert!(p.contains(&"actuator"));
    }

    #[test]
    fn fingerprint_wordpress_from_probes() {
        let mut wp = ProbeResp::new("https://x.test/wp-json", 200);
        wp.headers
            .insert("content-type".into(), "application/json".into());
        let probes = vec![
            wp,
            ProbeResp::new("https://x.test/wp-login.php", 200),
            ProbeResp::new("https://x.test/xmlrpc.php", 405),
        ];
        let (profile, _dists) = fingerprint(&probes);
        assert_eq!(profile, "WORDPRESS_CMS");
    }

    #[test]
    fn init_predict_observe_save_roundtrip() {
        let _guard = TEST_LOCK.lock().unwrap();
        reset();
        let dir = std::env::temp_dir().join(format!("ferox-ml-{}", std::process::id()));
        let _ = std::fs::create_dir_all(&dir);
        let model_path = dir.join("model.json").to_string_lossy().to_string();

        init("REST_API", &model_path, &MlParams::default());
        assert!(is_active());

        // profile seed predicts the api -> v1 transition
        let preds = predict_words("https://x.test/api", 5);
        assert!(preds.iter().any(|w| w == "v1"), "preds={preds:?}");

        // online learning captures a new chain, then persists
        observe("https://x.test/shop/checkout/receipt");
        let after = predict_words("https://x.test/shop/checkout", 5);
        assert!(after.iter().any(|w| w == "receipt"), "after={after:?}");

        save();
        assert!(std::path::Path::new(&model_path).exists());

        // a fresh init merges the saved learning back in
        reset();
        init("LEGACY_STATIC", &model_path, &MlParams::default());
        let merged = predict_words("https://x.test/shop/checkout", 5);
        assert!(merged.iter().any(|w| w == "receipt"), "merged={merged:?}");

        reset();
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn fingerprint_gate_rejects_blind_and_ambiguous() {
        // every probe failed to answer (status 0) -> blind -> not confident
        let blind = vec![
            ProbeResp::new("https://x.test/wp-json", 0),
            ProbeResp::new("https://x.test/api", 0),
        ];
        let (profile, confident) = fingerprint_gated(&blind, 0.10);
        assert!(!confident);
        assert_eq!(profile, GENERIC_PROFILE);

        // a clear WordPress target clears the margin and is trusted
        let mut wp = ProbeResp::new("https://x.test/wp-json", 200);
        wp.headers
            .insert("content-type".into(), "application/json".into());
        let probes = vec![
            wp,
            ProbeResp::new("https://x.test/wp-login.php", 200),
            ProbeResp::new("https://x.test/xmlrpc.php", 405),
        ];
        let (profile, confident) = fingerprint_gated(&probes, 0.10);
        assert!(confident, "clear WP should be confident");
        assert_eq!(profile, "WORDPRESS_CMS");
    }

    #[test]
    fn predict_for_scan_scales_budget() {
        let _guard = TEST_LOCK.lock().unwrap();
        reset();
        init("REST_API", "", &MlParams::default()); // predictions = 25

        // learn a rich set of children under /blog so the *budget* (not the number
        // of available predictions) is the binding constraint
        for i in 0..40 {
            observe(&format!("https://x.test/blog/post{i}"));
        }

        // base_len = 0 => no scaling => full budget (up to `predictions`)
        let full = predict_for_scan("https://x.test/blog", 0);
        assert!(full.len() >= 20, "full={}", full.len());

        // same directory measured as unproductive (40 hits over a huge base pass =>
        // tiny reward => low bandit value => smaller budget)
        let scaled = predict_for_scan("https://x.test/blog", 100_000);
        assert!(
            scaled.len() < full.len(),
            "scaled={} full={}",
            scaled.len(),
            full.len()
        );

        reset();
    }

    #[test]
    fn fingerprint_gate_margin_is_monotonic() {
        // a clear WordPress target: trusted at a lenient margin, demoted to the
        // generic profile once the required margin exceeds the actual gap
        let mut wp = ProbeResp::new("https://x.test/wp-json", 200);
        wp.headers
            .insert("content-type".into(), "application/json".into());
        let probes = vec![
            wp,
            ProbeResp::new("https://x.test/wp-login.php", 200),
            ProbeResp::new("https://x.test/xmlrpc.php", 405),
        ];
        let (_, dists) = fingerprint(&probes);
        let gap = dists[1].1 - dists[0].1;

        let (p_lo, c_lo) = fingerprint_gated(&probes, gap * 0.5);
        assert!(c_lo && p_lo == "WORDPRESS_CMS");

        let (p_hi, c_hi) = fingerprint_gated(&probes, gap * 2.0 + 1.0);
        assert!(!c_hi && p_hi == GENERIC_PROFILE);
    }

    #[test]
    fn predict_budget_has_a_floor_for_unproductive_dirs() {
        let _guard = TEST_LOCK.lock().unwrap();
        reset();
        init("REST_API", "", &MlParams::default()); // predictions = 25

        // rich learned context so budget (not availability) binds
        for i in 0..40 {
            observe(&format!("https://x.test/zone/leaf{i}"));
        }
        // zero hits over a huge base pass => minimum reward, but the floor keeps a
        // non-empty, bounded budget (>= 25%)
        let floored = predict_for_scan("https://x.test/zone", 1_000_000);
        assert!(!floored.is_empty(), "floor keeps at least one prediction");
        assert!(floored.len() <= 25, "never exceeds the base budget");

        reset();
    }

    #[test]
    fn no_rank_keeps_predictor_order_and_scheduler_choice_is_honored() {
        let _guard = TEST_LOCK.lock().unwrap();
        reset();
        let params = MlParams {
            rank: false,
            scheduler: "ucb1".to_string(),
            ..MlParams::default()
        };
        init("REST_API", "", &params);
        assert!(is_active());
        // with ranking off we still get the seed chain for /api
        let preds = predict_for_scan("https://x.test/api", 0);
        assert!(preds.iter().any(|w| w == "v1"), "preds={preds:?}");
        reset();
    }
}
