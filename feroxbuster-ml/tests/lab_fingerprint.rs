//! Lab-grounded integration tests for the ML layer.
//!
//! The probe responses in `tests/fixtures/jsintel_labs.json` were captured by
//! replaying `feroxbuster::ml::probe_paths()` against the live jsintel lab estate
//! (OWASP Juice Shop, DVWA, WebGoat, WordPress, Django). Replaying the capture
//! keeps the test deterministic and offline while exercising the fingerprinter on
//! *real* server behavior — including the catch-all / soft-404 servers that return
//! 200 for almost everything, which is exactly the case the confidence gate exists
//! to handle.
//!
//! What the capture shows (and we assert below):
//! * the **WordPress** mock is identified correctly (`wordpress_test_cookie`,
//!   `xmlrpc.php` 405, and a PHP `x-powered-by` survive the catch-all noise);
//! * **WebGoat** and **Django** answer 404/sparsely, so with no dynamic signal
//!   they fall to `LEGACY_STATIC` by a wide margin;
//! * the **Juice Shop** and **DVWA** catch-alls return 200 for every probe, so
//!   path-presence fingerprinting is driven by a weak margin — raising
//!   `--ml-fp-margin` correctly demotes them to the generic fallback. This is the
//!   honest limitation the gate guards against.

use std::collections::HashMap;

use feroxbuster::ml::{self, ProbeResp, GENERIC_PROFILE};

#[derive(serde::Deserialize)]
struct RawResp {
    url: String,
    status: u16,
    #[serde(default)]
    headers: HashMap<String, String>,
}

fn load_labs() -> HashMap<String, Vec<ProbeResp>> {
    let text = include_str!("fixtures/jsintel_labs.json");
    let raw: HashMap<String, Vec<RawResp>> = serde_json::from_str(text).unwrap();
    raw.into_iter()
        .map(|(lab, resps)| {
            let probes = resps
                .into_iter()
                .map(|r| ProbeResp {
                    url: r.url,
                    status: r.status,
                    headers: r.headers,
                    ..Default::default()
                })
                .collect();
            (lab, probes)
        })
        .collect()
}

#[test]
fn every_lab_probe_answered_and_classifies() {
    let labs = load_labs();
    for lab in ["juice-shop", "dvwa", "webgoat", "wordpress", "django"] {
        let probes = labs.get(lab).unwrap_or_else(|| panic!("missing lab {lab}"));
        assert!(
            probes.iter().any(|p| p.status != 0),
            "{lab}: at least one probe should have answered"
        );
        let (profile, dists) = ml::fingerprint(probes);
        assert_eq!(dists.len(), 4, "{lab}: distance to all four profiles");
        assert!(
            ml::PROFILES.contains(&profile.as_str()),
            "{lab}: classified as a known profile, got {profile}"
        );
    }
}

#[test]
fn wordpress_lab_is_identified() {
    let labs = load_labs();
    // the real WP mock keeps enough signal (wp cookie + xmlrpc 405 + PHP) to be
    // classified correctly despite returning 200 for unrelated probe paths
    let (profile, confident) = ml::fingerprint_gated(&labs["wordpress"], 0.10);
    assert_eq!(profile, "WORDPRESS_CMS", "wordpress lab");
    assert!(confident);
}

#[test]
fn sparse_or_404_labs_fall_to_static_by_a_wide_margin() {
    let labs = load_labs();
    for lab in ["webgoat", "django"] {
        let (_, dists) = ml::fingerprint(&labs[lab]);
        let margin = dists[1].1 - dists[0].1;
        let (profile, confident) = ml::fingerprint_gated(&labs[lab], 0.10);
        assert_eq!(profile, "LEGACY_STATIC", "{lab} with no dynamic signal");
        assert!(confident);
        assert!(margin > 1.0, "{lab}: unambiguous static, margin={margin}");
    }
}

#[test]
fn catchall_labs_abstain_via_the_e1_guard() {
    let labs = load_labs();
    // Juice Shop and DVWA answer 200 to every probe with no strong discriminator,
    // so the E1 catch-all guard abstains them to the generic profile at any margin
    // — instead of the confident-but-wrong profile raw classification would pick.
    for lab in ["juice-shop", "dvwa"] {
        assert!(ml::is_catch_all(&labs[lab]), "{lab} is a catch-all server");

        let (profile, confident) = ml::fingerprint_gated(&labs[lab], 0.10);
        assert!(!confident, "{lab}: catch-all guard abstains even at a lenient margin");
        assert_eq!(profile, GENERIC_PROFILE, "{lab}: falls back to generic");

        // raw (unguarded) classification would have committed to a specific profile
        let (raw, _) = ml::fingerprint(&labs[lab]);
        assert_ne!(raw, GENERIC_PROFILE, "{lab}: raw classifier picks a specific profile");
    }
}

#[test]
fn learns_the_real_juice_shop_static_tree_online() {
    // Real same-origin paths jsintel crawled from the Juice Shop lab
    // (output_lab_run/assets/crawled_urls.txt). After observing them, the model
    // should predict the learned siblings for their directory contexts.
    let crawled = [
        "http://shop.vuln.lab/static/app.js",
        "http://shop.vuln.lab/static/config.json",
        "http://shop.vuln.lab/static/endpoints.js",
        "http://shop.vuln.lab/static/framework.js",
        "http://shop.vuln.lab/static/dom.js",
        "http://shop.vuln.lab/static/lazy.js",
        "http://shop.vuln.lab/static/jquery/jquery.min.js",
        "http://shop.vuln.lab/static/jquery/graphql.js",
    ];

    ml::init("LEGACY_STATIC", "", &ml::MlParams::default());
    for url in crawled {
        ml::observe(url);
    }

    // directory context /static -> learned children
    let under_static = ml::predict_words("http://shop.vuln.lab/static", 25);
    for want in ["app.js", "config.json", "endpoints.js", "framework.js"] {
        assert!(
            under_static.iter().any(|w| w == want),
            "/static should predict {want}, got {under_static:?}"
        );
    }

    // nested directory context /static/jquery -> its learned children
    let under_jquery = ml::predict_words("http://shop.vuln.lab/static/jquery", 25);
    assert!(
        under_jquery.iter().any(|w| w == "jquery.min.js"),
        "/static/jquery should predict jquery.min.js, got {under_jquery:?}"
    );
}
