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
use crate::profiles::{centroids, N_FEATURES, PROBE_PATHS};
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
    let strong_dynamic =
        rest_like || has("actuator") || has("wp-json") || swagger || x_powered_by;
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
    ]
}

fn euclidean(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b).map(|(x, y)| (x - y) * (x - y)).sum::<f64>().sqrt()
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
pub fn is_catch_all(probes: &[ProbeResp]) -> bool {
    present_fraction(probes) > CATCH_ALL_PRESENT_FRACTION && !has_strong_discriminator(probes)
}

/// Nearest-centroid classifier over the four profile centroids.
pub struct NearestCentroid;

impl Classifier for NearestCentroid {
    fn classify(&self, features: &[f64]) -> (String, Vec<(String, f64)>) {
        let mut dists: Vec<(String, f64)> = centroids()
            .into_iter()
            .map(|(name, c)| (name.to_string(), euclidean(features, &c)))
            .collect();
        dists.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        let best = dists[0].0.clone();
        (best, dists)
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
