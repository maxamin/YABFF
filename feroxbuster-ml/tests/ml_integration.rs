//! Integration tests for feroxbuster's native ML layer (`feroxbuster::ml`).
//!
//! These exercise the public ML API the scanner wires into: fingerprinting a
//! target from probe responses, seeding/predicting with the Markov model, online
//! learning, and model persistence. The stateful flow is kept in a single test
//! function because the active model is process-global shared state.

use feroxbuster::ml::{self, ProbeResp};

#[test]
fn probe_paths_are_exposed_and_discriminating() {
    let p = ml::probe_paths();
    assert!(!p.is_empty());
    for marker in ["wp-json", "actuator", "api", "rest", "robots.txt"] {
        assert!(p.contains(&marker), "probe paths should include {marker}");
    }
}

#[test]
fn fingerprint_classifies_rest_api() {
    // an API that 500s on its base path (Juice-Shop-shaped) is still REST
    let probes = vec![
        ProbeResp::new("https://x.test/api", 500),
        ProbeResp::new("https://x.test/rest", 500),
        ProbeResp::new("https://x.test/api/v1", 500),
        ProbeResp::new("https://x.test/robots.txt", 200),
        ProbeResp::new("https://x.test/", 200),
    ];
    let (profile, dists) = ml::fingerprint(&probes);
    assert_eq!(profile, "REST_API", "dists={dists:?}");
    assert_eq!(dists.len(), 4, "distances to all four profiles");
}

#[test]
fn fingerprint_classifies_wordpress() {
    let mut root = ProbeResp::new("https://x.test/wp-json", 200);
    root.headers
        .insert("content-type".into(), "application/json".into());
    let probes = vec![
        root,
        ProbeResp::new("https://x.test/wp-login.php", 200),
        ProbeResp::new("https://x.test/xmlrpc.php", 405),
    ];
    let (profile, _) = ml::fingerprint(&probes);
    assert_eq!(profile, "WORDPRESS_CMS");
}

#[test]
fn learn_predict_and_persist_end_to_end() {
    let dir = std::env::temp_dir().join(format!("ferox-ml-it-{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let model_path = dir.join("model.json").to_string_lossy().to_string();
    let _ = std::fs::remove_file(&model_path);

    // seed the REST profile; its seed predicts api -> v1
    ml::init("REST_API", &model_path, &ml::MlParams::default());
    assert!(ml::is_active());
    let preds = ml::predict_words("https://x.test/api", 6);
    assert!(preds.iter().any(|w| w == "v1"), "preds={preds:?}");

    // learn a non-standard chain online, then it becomes predictable
    ml::observe("https://x.test/shop/checkout/receipt");
    let chain = ml::predict_words("https://x.test/shop/checkout", 6);
    assert!(chain.iter().any(|w| w == "receipt"), "chain={chain:?}");

    // persist, then a fresh init (different profile) merges the learning back
    ml::save();
    assert!(std::path::Path::new(&model_path).exists());
    ml::init("LEGACY_STATIC", &model_path, &ml::MlParams::default());
    let merged = ml::predict_words("https://x.test/shop/checkout", 6);
    assert!(
        merged.iter().any(|w| w == "receipt"),
        "learned transition should survive persistence: {merged:?}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}
