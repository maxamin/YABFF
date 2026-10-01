//! End-to-end exercise of **every** ML engine function against the real jsintel
//! lab captures (`tests/fixtures/jsintel_labs_full.json`, captured from the live
//! Juice Shop / DVWA / WebGoat / WordPress / Django estate with response sizes and
//! soft-404 baselines). Replaying the capture keeps it deterministic and offline.
//!
//! Run the full report with:
//!   cargo test -p feroxbuster --test lab_features -- --nocapture
//!
//! Features covered: feature_vector, weighted nearest-centroid classify (E3),
//! calibrated confidence (E5), catch-all guard (E1), per-path soft-404 scoring
//! (E2), the confidence gate, online re-fingerprinting (E6), K-Means refresh (E7),
//! Markov seed+predict+subword back-off (E8), BM25 ranking, and the bandit.

use std::collections::HashMap;

use feroxbuster::ml::{self, ProbeResp};
use ferox_ml_core::fingerprint::{classify_confidence, feature_vector, kmeans_fit, ProfileTracker};
use ferox_ml_core::interfaces::Predictor;
use ferox_ml_core::markov::MarkovModel;
use ferox_ml_core::profiles::{FEATURE_WEIGHTS, N_FEATURES};
use ferox_ml_core::ranking::Bm25;
use ferox_ml_core::{scheduler, tokenize};

#[derive(serde::Deserialize, Clone)]
struct Resp {
    url: String,
    status: u16,
    #[serde(default)]
    headers: HashMap<String, String>,
    #[serde(default)]
    content_length: u64,
    #[serde(default)]
    word_count: u64,
    #[serde(default)]
    line_count: u64,
}

#[derive(serde::Deserialize)]
struct LabFull {
    probes: Vec<Resp>,
    random: Vec<Resp>,
}

fn to_probe(r: &Resp) -> ProbeResp {
    ProbeResp {
        url: r.url.clone(),
        status: r.status,
        headers: r.headers.clone(),
        content_length: r.content_length,
        word_count: r.word_count,
        line_count: r.line_count,
    }
}

fn load() -> HashMap<String, LabFull> {
    serde_json::from_str(include_str!("fixtures/jsintel_labs_full.json")).unwrap()
}

const LABS: [&str; 5] = ["juice-shop", "dvwa", "webgoat", "wordpress", "django"];

#[test]
fn every_engine_function_against_the_labs() {
    let data = load();

    println!("\n==== per-lab feature sweep ====");
    for lab in LABS {
        let full = &data[lab];
        let probes: Vec<ProbeResp> = full.probes.iter().map(to_probe).collect();
        let randoms: Vec<ProbeResp> = full.random.iter().map(to_probe).collect();

        // --- feature_vector + weighted classify (E3) ---
        let fv = feature_vector(&probes);
        assert_eq!(fv.len(), N_FEATURES, "{lab}: feature width");
        let (raw, _dists) = ml::fingerprint(&probes);

        // --- E5 calibrated confidence ---
        let (cp, conf) = classify_confidence(&fv);
        assert!(conf > 0.0 && conf <= 1.0, "{lab}: confidence {conf}");

        // --- E1 catch-all guard ---
        let catchall = ml::is_catch_all(&probes);

        // --- E2 per-path soft-404 scoring ---
        let mut cleaned = probes.clone();
        let demoted = ml::score_soft_404(&mut cleaned, &randoms);

        // --- confidence gate (E1+E2+E3) ---
        let (gated, confident) = ml::fingerprint_gated(&probes, 0.10);

        println!(
            "{lab:11} raw={raw:22} conf={cp:14}({conf:.2}) catchall={catchall:5} \
             soft404_demoted={demoted:2} gated={gated:16} confident={confident}"
        );

        // --- E6 online re-fingerprinting: feed the answered probe paths ---
        let mut tracker = ProfileTracker::new(&gated);
        for r in &full.probes {
            if r.status != 0 && r.status != 404 {
                tracker.observe_path(&r.url);
            }
        }
        let tracked = tracker.current();

        // --- E8 Markov seed + predict for this profile ---
        let model = MarkovModel::seeded(&raw, 3, 0.5, 0.02);
        let preds = model.predict("", 5);

        // --- BM25 over discovered segments ---
        let mut bm = Bm25::new();
        for r in &full.probes {
            if r.status != 0 && r.status != 404 {
                bm.add_document(&tokenize::last_segment(&r.url));
            }
        }
        let ranked = bm.rerank(&preds);

        // --- bandit value ---
        let mut sched = scheduler::build("thompson", 1);
        sched.add_arm(lab);
        sched.update(lab, 1.0);
        let value = sched.value(lab);

        println!(
            "            tracker={tracked:16} seed_preds={:?} bm25_top={} bandit_value={value:.2}",
            preds.iter().map(|(t, _)| t.as_str()).take(3).collect::<Vec<_>>(),
            ranked.first().map(|(t, _)| t.as_str()).unwrap_or("-"),
        );
    }

    // --- E7 real K-Means over all five lab feature vectors ---
    println!("\n==== E7 K-Means over the 5 lab vectors ====");
    let vectors: Vec<[f64; N_FEATURES]> =
        LABS.iter().map(|l| feature_vector(&data[*l].probes.iter().map(to_probe).collect::<Vec<_>>())).collect();
    let refreshed = kmeans_fit(&vectors, 10, &FEATURE_WEIGHTS);
    assert_eq!(refreshed.len(), ml::PROFILES.len());
    println!("refreshed {} centroids (seeded from profiles)", refreshed.len());

    // ---- assertions: the documented, signal-bearing verdicts ----
    let g = |lab: &str| ml::fingerprint_gated(&data[lab].probes.iter().map(to_probe).collect::<Vec<_>>(), 0.10);
    assert_eq!(g("wordpress"), ("WORDPRESS_CMS".to_string(), true));
    assert_eq!(g("django"), ("DJANGO".to_string(), true));
    assert_eq!(g("webgoat").0, "LEGACY_STATIC");
    // catch-alls are abstained by the gate
    assert!(!g("juice-shop").1, "juice-shop catch-all abstained");
    assert!(!g("dvwa").1, "dvwa catch-all abstained");

    // E2: the catch-all labs serve a uniform soft-404 body, so scoring demotes
    // several of their probes; a signal-bearing lab (wordpress) loses far fewer.
    let demote = |lab: &str| {
        let mut p: Vec<ProbeResp> = data[lab].probes.iter().map(to_probe).collect();
        ml::score_soft_404(&mut p, &data[lab].random.iter().map(to_probe).collect::<Vec<_>>())
    };
    assert!(demote("juice-shop") > 0, "juice-shop soft-404 demotions");
    assert!(demote("dvwa") > 0, "dvwa soft-404 demotions");
}
