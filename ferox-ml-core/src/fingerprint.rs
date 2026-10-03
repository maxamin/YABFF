//! Phase 1 — target fingerprinting.
//!
//! A bounded probe (run through feroxbuster over [`crate::profiles::PROBE_PATHS`])
//! yields responses; [`feature_vector`] turns them into a numeric vector, and a
//! [`Classifier`] assigns the target to the nearest profile centroid.
//!
//! Two classifiers, both satisfying "each seed centroid classifies to its own
//! profile": [`NearestCentroid`] (default) and [`KMeansClassifier`] (KMeans
//! seeded with the profile centroids — with a single target vector it performs
//! one assignment step, which is exactly nearest-centroid; it exists for parity
//! and to seed a real KMeans when batch-probing many hosts later).

use crate::ProbeResp;
use crate::interfaces::Classifier;
use std::collections::{HashMap, HashSet};

use crate::profiles::{centroids, seed_matrix, FEATURE_WEIGHTS, N_FEATURES, PROBE_PATHS, PROFILES};
use crate::tokenize::path_segments;

/// Normalize a URL to its slash-joined path segments, lowercased.
fn norm_path(url: &str) -> String {
    path_segments(url).join("/")
}

/// Build the [`N_FEATURES`]-dim feature vector from probe responses.
pub fn feature_vector(responses: &[ProbeResp]) -> [f64; N_FEATURES] {
    // "present" = a non-404 status we asked feroxbuster to report.
    let present: std::collections::HashSet<String> = responses
        .iter()
        .filter(|r| r.status != 0 && r.status != 404)
        .map(|r| norm_path(&r.url))
        .collect();
    let has = |p: &str| present.contains(p);

    // aggregate header signals across all probe responses
    let mut x_powered_by = false;
    let mut server_java = false;
    let mut cookie_jsession = false;
    let mut cookie_wp = false;
    let mut cookie_php = false;
    let mut php_sig = false;
    let mut root_json = false;
    let mut cookie_django = false;
    // richer technology signals (E-features from headers/cookies/body shape)
    let mut cookie_node = false; // connect.sid / next-auth / express session
    let mut cookie_php_fw = false; // laravel / codeigniter / symfony session
    let mut sec_headers = false; // CSP / HSTS / X-Frame-Options / X-Content-Type-Options
    for r in responses {
        if let Some(v) = r.header("x-powered-by") {
            x_powered_by = true;
            let lv = v.to_lowercase();
            if lv.contains("php") {
                php_sig = true;
            }
            if lv.contains("servlet") || lv.contains("jsp") {
                server_java = true;
            }
        }
        if let Some(v) = r.header("server") {
            let lv = v.to_lowercase();
            if lv.contains("tomcat") || lv.contains("jetty") || lv.contains("coyote") {
                server_java = true;
            }
        }
        if let Some(v) = r.header("set-cookie") {
            let lv = v.to_lowercase();
            if lv.contains("jsessionid") {
                cookie_jsession = true;
            }
            if lv.contains("wordpress_") || lv.contains("wp-") {
                cookie_wp = true;
            }
            if lv.contains("phpsessid") {
                cookie_php = true;
                php_sig = true;
            }
            // Django's CSRF cookie is distinctive (its "sessionid" cookie is not,
            // since "jsessionid" also contains it, so key only on csrftoken).
            if lv.contains("csrftoken") {
                cookie_django = true;
            }
            if lv.contains("connect.sid") || lv.contains("next-auth") || lv.contains("express:sess") {
                cookie_node = true;
            }
            if lv.contains("laravel_session") || lv.contains("ci_session") || lv.contains("symfony") {
                cookie_php_fw = true;
                php_sig = true;
            }
        }
        // security headers → a "modern app" signal (SPA/REST/Spring/Django lean on
        // them; classic static/PHP sites less so)
        for h in [
            "content-security-policy",
            "strict-transport-security",
            "x-frame-options",
            "x-content-type-options",
        ] {
            if r.header(h).is_some() {
                sec_headers = true;
            }
        }
        // root response content-type
        if norm_path(&r.url).is_empty() {
            if let Some(ct) = r.header("content-type") {
                if ct.to_lowercase().contains("application/json") {
                    root_json = true;
                }
            }
        }
    }

    let swagger = has("swagger-ui.html") || has("openapi.json") || has("v2/api-docs");
    // REST/GraphQL APIs often 500 or 200 on their base path (e.g. Juice Shop's
    // /rest, /graphql); fold those into the "api" signal so fingerprinting isn't
    // blind to them. The probe runs with all_codes, so a 500 is visible here.
    let rest_like = has("api") || has("api/v1") || has("api/v2") || has("rest") || has("graphql");
    let jsp_sig = server_java || has("index.jsp");
    if has("index.php") || has("xmlrpc.php") {
        php_sig = true;
    }
    let sig_spa = has("manifest.webmanifest") || has("_next") || has("assets");
    let sig_django = cookie_django || has("static/admin");
    // a real rendered HTML page was served (vs small JSON/404 bodies) — separates
    // CMS/SPA/static from JSON APIs. Uses body sizes captured on the probe.
    let heavy_html = responses
        .iter()
        .any(|r| r.status != 0 && r.status != 404 && r.word_count > 200);
    // A strong framework session cookie (JSESSIONID / WordPress / Node / PHP-fw;
    // Django's csrftoken is already folded into sig_django) means the app is
    // unambiguously dynamic, even when an auth wall 302s every probe path to a
    // login page and no path-presence signal survives soft-404 demotion — the
    // auth-walled Spring (WebGoat) case. Without this, such a target keeps
    // static_sig high and misclassifies as LEGACY_STATIC despite the cookie.
    let strong_cookie = cookie_jsession || cookie_wp || cookie_node || cookie_php_fw;
    let strong_dynamic = rest_like
        || has("actuator")
        || has("wp-json")
        || swagger
        || x_powered_by
        || sig_spa
        || sig_django
        || strong_cookie;
    let static_sig = if strong_dynamic { 0.1 } else { 0.9 };

    let b = |cond: bool| if cond { 1.0 } else { 0.0 };
    [
        b(has("wp-json")),        // 0
        b(has("wp-login.php")),   // 1
        b(has("actuator")),       // 2
        b(has("actuator/health")),// 3
        b(rest_like),             // 4  (api / api/v1|v2 / rest / graphql)
        b(has("api/v1") || has("api/v2")), // 5
        b(swagger),               // 6
        b(has(".git/head") || has(".git/HEAD")), // 7 (norm_path lowercases)
        b(has("robots.txt")),     // 8
        b(root_json),             // 9
        b(x_powered_by),          // 10
        b(server_java),           // 11
        b(cookie_jsession),       // 12
        b(cookie_wp),             // 13
        b(cookie_php),            // 14
        b(php_sig),               // 15
        b(jsp_sig),               // 16
        static_sig,               // 17
        b(sig_spa),               // 18
        b(sig_django),            // 19
        b(cookie_node),           // 20
        b(cookie_php_fw),         // 21
        b(sec_headers),           // 22
        b(heavy_html),            // 23
    ]
}

/// Weighted Euclidean distance: each squared feature difference is scaled by its
/// weight, so high-weight discriminators dominate the metric (E3).
fn weighted_euclidean(a: &[f64], b: &[f64], weights: &[f64]) -> f64 {
    a.iter()
        .zip(b)
        .zip(weights)
        .map(|((x, y), w)| w * (x - y) * (x - y))
        .sum::<f64>()
        .sqrt()
}

/// Nearest-centroid classification under an explicit weight vector. Exposed so an
/// evaluation can compare weighted (E3) vs unweighted metrics; the default
/// [`NearestCentroid`] uses [`FEATURE_WEIGHTS`].
pub fn classify_with_weights(features: &[f64], weights: &[f64]) -> (String, Vec<(String, f64)>) {
    let mut dists: Vec<(String, f64)> = centroids()
        .into_iter()
        .map(|(name, c)| (name.to_string(), weighted_euclidean(features, &c, weights)))
        .collect();
    dists.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    let best = dists[0].0.clone();
    (best, dists)
}

/// Default softmax temperature for [`softmax_confidence`] — sharp enough that a
/// clear match reads as high confidence, soft enough that a near-tie reads as low.
pub const DEFAULT_CONFIDENCE_BETA: f64 = 2.0;

/// E5 — calibrated confidence. Turn centroid *distances* into a probability
/// distribution over profiles via `softmax(-beta * distance)`, so callers get a
/// comparable `[0,1]` confidence rather than a raw distance or margin. The nearest
/// centroid gets the highest probability; a near-tie spreads mass across profiles.
/// Returns `(profile, probability)` pairs, highest probability first.
pub fn softmax_confidence(dists: &[(String, f64)], beta: f64) -> Vec<(String, f64)> {
    if dists.is_empty() {
        return vec![];
    }
    // subtract the min distance for numerical stability (shift-invariant)
    let min = dists.iter().map(|(_, d)| *d).fold(f64::INFINITY, f64::min);
    let exps: Vec<f64> = dists.iter().map(|(_, d)| (-beta * (d - min)).exp()).collect();
    let sum: f64 = exps.iter().sum();
    let mut out: Vec<(String, f64)> = dists
        .iter()
        .zip(exps)
        .map(|((n, _), e)| (n.clone(), e / sum))
        .collect();
    out.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    out
}

/// E5 — classify and return the top profile with its calibrated probability.
pub fn classify_confidence(features: &[f64]) -> (String, f64) {
    let (_, dists) = classify_with_weights(features, &FEATURE_WEIGHTS);
    softmax_confidence(&dists, DEFAULT_CONFIDENCE_BETA)
        .into_iter()
        .next()
        .unwrap_or_else(|| ("LEGACY_STATIC".to_string(), 0.0))
}

/// E7 — fit `k = PROFILES.len()` centroids over many host feature vectors with
/// Lloyd's algorithm, seeded from the profile [`centroids`] (so cluster identity
/// stays aligned to the named profiles). Returns refined `(profile, centroid)`
/// pairs for batch analysis / data-driven centroid refresh across a set of
/// authorized hosts; the single-target classifier is unchanged. Empty clusters
/// keep their seed centroid.
pub fn kmeans_fit(
    vectors: &[[f64; N_FEATURES]],
    iters: usize,
    weights: &[f64],
) -> Vec<(&'static str, [f64; N_FEATURES])> {
    let seeds = centroids();
    let names: Vec<&'static str> = seeds.iter().map(|(n, _)| *n).collect();
    let mut cents: Vec<[f64; N_FEATURES]> = seeds.iter().map(|(_, c)| *c).collect();
    if vectors.is_empty() {
        return names.into_iter().zip(cents).collect();
    }
    for _ in 0..iters {
        let mut sums = vec![[0.0f64; N_FEATURES]; cents.len()];
        let mut counts = vec![0usize; cents.len()];
        for v in vectors {
            let ci = cents
                .iter()
                .enumerate()
                .map(|(i, c)| (i, weighted_euclidean(v, c, weights)))
                .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
                .map(|(i, _)| i)
                .unwrap();
            for d in 0..N_FEATURES {
                sums[ci][d] += v[d];
            }
            counts[ci] += 1;
        }
        for i in 0..cents.len() {
            if counts[i] > 0 {
                for d in 0..N_FEATURES {
                    cents[i][d] = sums[i][d] / counts[i] as f64;
                }
            }
        }
    }
    names.into_iter().zip(cents).collect()
}

/// A server is treated as a catch-all / soft-404 responder when it answers
/// (non-404) to more than this fraction of the discriminating probe paths.
pub const CATCH_ALL_PRESENT_FRACTION: f64 = 0.7;

/// Fraction of the discriminating probe paths that "answered" — a non-zero,
/// non-404 status. Measured against the known probe-path count
/// ([`PROBE_PATHS`]), not the length of `probes`, so it is correct whether the
/// caller includes 404 responses in the slice (the fork does) or omits them (a
/// feroxbuster NDJSON stream may). Capped at 1.0 for callers that add extra
/// probes (e.g. the root path).
pub fn present_fraction(probes: &[ProbeResp]) -> f64 {
    let total = PROBE_PATHS.len().max(1);
    let present = probes
        .iter()
        .filter(|p| p.status != 0 && p.status != 404)
        .count();
    (present as f64 / total as f64).min(1.0)
}

/// Whether any probe carries a strong, hard-to-fake framework discriminator that
/// survives a catch-all: a WordPress or servlet session cookie, or `xmlrpc.php`
/// answering 405 (WordPress's method-not-allowed, which a catch-all 200 won't
/// produce).
pub fn has_strong_discriminator(probes: &[ProbeResp]) -> bool {
    probes.iter().any(|p| {
        let cookie = p.header("set-cookie").unwrap_or_default().to_lowercase();
        let wp = cookie.contains("wordpress_") || cookie.contains("wp-");
        let jsession = cookie.contains("jsessionid");
        let xmlrpc_405 = p.url.to_lowercase().ends_with("xmlrpc.php") && p.status == 405;
        wp || jsession || xmlrpc_405
    })
}

/// E1 — catch-all / soft-404 guard. True when the target answers almost every
/// probe path (so path-presence features are noise) **and** exposes no strong
/// discriminator to classify on. Callers should abstain — fall back to a generic
/// profile — rather than trust a confident-but-wrong fingerprint. Validated on
/// the jsintel labs: it demotes the Juice Shop / DVWA catch-alls while leaving
/// WordPress (whose cookie + `xmlrpc.php` 405 survive) classified.
///
/// Most precise when the probes have first been cleaned by [`apply_soft_404`]
/// (E2): then "present" already excludes catch-all soft-404 bodies and this is a
/// backstop for callers without body data.
pub fn is_catch_all(probes: &[ProbeResp]) -> bool {
    present_fraction(probes) > CATCH_ALL_PRESENT_FRACTION && !has_strong_discriminator(probes)
}

/// E2 — learn the server's soft-404 response signature(s) from probes to random,
/// almost-certainly-absent paths. Each answered random probe contributes its
/// [`ProbeResp::signature`] as a known-bogus fingerprint.
pub fn learn_soft_404(random_probes: &[ProbeResp]) -> crate::dedup::SoftNotFoundFilter {
    let mut filter = crate::dedup::SoftNotFoundFilter::new();
    for p in random_probes {
        if p.status != 0 {
            filter.learn_bogus(p.signature());
        }
    }
    filter
}

/// E2 — per-path soft-404 scoring. A catch-all server answers 200 to every path
/// with the *same* soft-404 body; comparing each discriminating probe's signature
/// to the learned baseline tells real content from that uniform noise. Probes
/// whose signature matches the soft-404 baseline are demoted to status 404 so
/// that [`feature_vector`] / [`present_fraction`] / [`is_catch_all`] treat them
/// as absent — turning "everything is present" back into the true, sparse signal.
/// Returns how many probes were demoted. A no-op when `filter` learned nothing or
/// the probes carry no body data (all sizes 0 collapse to one bucket, but with an
/// empty filter nothing matches).
pub fn apply_soft_404(probes: &mut [ProbeResp], filter: &crate::dedup::SoftNotFoundFilter) -> usize {
    let mut demoted = 0;
    for p in probes.iter_mut() {
        if p.status != 0 && p.status != 404 && filter.is_soft_not_found(&p.signature()) {
            p.status = 404;
            demoted += 1;
        }
    }
    demoted
}

/// Nearest-centroid classifier over the four profile centroids.
pub struct NearestCentroid;

impl Classifier for NearestCentroid {
    fn classify(&self, features: &[f64]) -> (String, Vec<(String, f64)>) {
        // E3: weighted metric so strong discriminators outweigh noisy presence bits
        classify_with_weights(features, &FEATURE_WEIGHTS)
    }
}

/// KMeans seeded with the profile centroids. For a single target vector this
/// reduces to one nearest-centroid assignment step (documented in the module).
pub struct KMeansClassifier {
    pub seed: u64,
}

impl Classifier for KMeansClassifier {
    fn classify(&self, features: &[f64]) -> (String, Vec<(String, f64)>) {
        // With one observation the Lloyd assignment step is nearest-centroid.
        let _ = self.seed;
        NearestCentroid.classify(features)
    }
}

/// The characteristic path vocabulary of a profile (its seed matrix's `from` and
/// `to` tokens).
fn profile_vocab(profile: &str) -> HashSet<String> {
    let mut v = HashSet::new();
    for (from, tos) in seed_matrix(profile) {
        if !from.is_empty() {
            v.insert(from.to_string());
        }
        for (to, _) in tos {
            v.insert(to.to_string());
        }
    }
    v
}

/// E6 — online re-fingerprinting. Starts from the probe's initial profile and
/// accumulates evidence from paths discovered during the scan: each discovered
/// segment that belongs to a profile's characteristic vocabulary votes for it,
/// weighted by specificity (a token shared by many profiles counts less). So a
/// scan can correct an ambiguous initial guess — e.g. discovering `/wp-content`,
/// `/wp-admin` flips an unsure target to `WORDPRESS_CMS`.
pub struct ProfileTracker {
    votes: HashMap<String, f64>,
    vocab: HashMap<&'static str, HashSet<String>>,
    /// how many profiles each token appears in (for specificity weighting)
    doc_freq: HashMap<String, usize>,
    initial: String,
}

impl ProfileTracker {
    /// Start from the initial (probe) profile, which is given a prior vote.
    pub fn new(initial: &str) -> Self {
        let vocab: HashMap<&'static str, HashSet<String>> =
            PROFILES.iter().map(|p| (*p, profile_vocab(p))).collect();
        let mut doc_freq: HashMap<String, usize> = HashMap::new();
        for set in vocab.values() {
            for tok in set {
                *doc_freq.entry(tok.clone()).or_insert(0) += 1;
            }
        }
        let mut votes = HashMap::new();
        votes.insert(initial.to_string(), 1.0); // prior for the probe's guess
        Self {
            votes,
            vocab,
            doc_freq,
            initial: initial.to_string(),
        }
    }

    /// Fold a discovered URL's path segments into the per-profile evidence.
    pub fn observe_path(&mut self, path: &str) {
        for seg in path_segments(path) {
            let df = *self.doc_freq.get(&seg).unwrap_or(&0);
            if df == 0 {
                continue; // not characteristic of any profile
            }
            let weight = 1.0 / df as f64; // specific tokens count more
            for p in PROFILES {
                if self.vocab[p].contains(&seg) {
                    *self.votes.entry(p.to_string()).or_insert(0.0) += weight;
                }
            }
        }
    }

    /// The profile with the most accumulated evidence (ties keep the initial).
    pub fn current(&self) -> String {
        let initial_score = *self.votes.get(&self.initial).unwrap_or(&0.0);
        self.votes
            .iter()
            .fold((self.initial.clone(), initial_score), |(bn, bs), (n, s)| {
                if *s > bs {
                    (n.clone(), *s)
                } else {
                    (bn, bs)
                }
            })
            .0
    }

    /// `Some(profile)` if accumulated evidence now favors a different profile than
    /// the initial probe guess — the signal to re-seed mid-scan.
    pub fn corrected(&self) -> Option<String> {
        let now = self.current();
        (now != self.initial).then_some(now)
    }
}

/// Build the classifier named in the config.
pub fn build(name: &str, seed: u64) -> Box<dyn Classifier> {
    match name {
        "kmeans" => Box::new(KMeansClassifier { seed }),
        _ => Box::new(NearestCentroid),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_seed_centroid_classifies_to_itself() {
        let clf = NearestCentroid;
        for (name, c) in centroids() {
            let (got, _dists) = clf.classify(&c);
            assert_eq!(got, name, "centroid for {name} classified as {got}");
        }
    }

    #[test]
    fn kmeans_variant_matches_nearest_centroid() {
        for (name, c) in centroids() {
            let (got, _) = (KMeansClassifier { seed: 1 }).classify(&c);
            assert_eq!(got, name);
        }
    }

    #[test]
    fn api_that_500s_on_base_is_detected_as_rest() {
        // Juice-Shop-shaped: /rest, /api, /api/v1 all 500 (visible under all_codes),
        // HTML root, robots present. Must still fingerprint as REST, not LEGACY.
        let mk = |u: &str, s: u16| ProbeResp {
            url: u.into(),
            status: s,
            ..Default::default()
        };
        let resps = vec![
            mk("https://x.test/rest", 500),
            mk("https://x.test/api", 500),
            mk("https://x.test/api/v1", 500),
            mk("https://x.test/robots.txt", 200),
            mk("https://x.test/", 200),
        ];
        let fv = feature_vector(&resps);
        let (profile, dists) = NearestCentroid.classify(&fv);
        assert_eq!(profile, "REST_API", "features={fv:?} dists={dists:?}");
    }

    #[test]
    fn wordpress_probe_vector_classifies_as_wordpress() {
        // synthesize probe responses for a WordPress-looking target
        let resps = vec![
            ProbeResp {
                url: "https://x.test/wp-json".into(),
                status: 200,
                headers: [("content-type".to_string(), "application/json".to_string())]
                    .into_iter()
                    .collect(),
                ..Default::default()
            },
            ProbeResp {
                url: "https://x.test/wp-login.php".into(),
                status: 200,
                ..Default::default()
            },
            ProbeResp {
                url: "https://x.test/xmlrpc.php".into(),
                status: 405,
                headers: [("x-powered-by".to_string(), "PHP/8.1".to_string())]
                    .into_iter()
                    .collect(),
                ..Default::default()
            },
        ];
        let fv = feature_vector(&resps);
        let (profile, _) = NearestCentroid.classify(&fv);
        assert_eq!(profile, "WORDPRESS_CMS", "features={fv:?}");
    }
}
