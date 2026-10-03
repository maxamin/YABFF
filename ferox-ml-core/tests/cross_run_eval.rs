//! Cross-run learning evaluation — a regression guard for the README's core
//! promise that YABFF "carries knowledge across runs": train a model on one
//! authorized target, persist it, then show that a *second* scan on a different
//! target reuses that knowledge to reach a path a cold scan never predicts.
//!
//! The mechanism under test is the real one: [`Campaign::learn`] harvests path
//! structure and writes a model to disk; a later [`Campaign::run`] loads it via
//! `model_path` and merges it onto the profile seed, so its online predictions
//! include tokens learned on the earlier target.
//!
//! The fake runner is **word-gated**: like feroxbuster, it only "discovers" a
//! path when the request actually contained that path's final segment. That is
//! what makes warm-vs-cold meaningful — a directory is found only if the engine
//! predicted the right word, so a token that exists only in the carried-over
//! model is the difference between finding `/api/telemetry` and missing it.

use ferox_ml_core::config::Config;
use ferox_ml_core::ferox::{FeroxArgs, FeroxResponse, FeroxRunner};
use ferox_ml_core::orchestrator::Campaign;

/// A learned token that appears in NO shipped profile seed, so the only way a
/// scan can predict it is by having learned it on a previous run.
const CARRIED_TOKEN: &str = "telemetry";

fn dir(url: &str) -> FeroxResponse {
    let url = if url.ends_with('/') {
        url.to_string()
    } else {
        format!("{url}/")
    };
    FeroxResponse {
        url,
        status: 200,
        content_length: 10,
        word_count: 2,
        line_count: 1,
        ..Default::default()
    }
}

fn last_segment(url: &str) -> &str {
    url.trim_end_matches('/').rsplit('/').next().unwrap_or("")
}

/// Training-side runner: on the link-extraction harvest it reveals the real
/// structure `/api` → `/api/telemetry`, which `learn()` ingests into the model.
struct TrainRunner;
impl FeroxRunner for TrainRunner {
    fn run(&self, args: &FeroxArgs) -> anyhow::Result<Vec<FeroxResponse>> {
        if args.extract_links {
            return Ok(vec![
                dir("https://train.test/api"),
                dir(&format!("https://train.test/api/{CARRIED_TOKEN}")),
            ]);
        }
        // fingerprint/probe call: a REST-looking root (has /api)
        Ok(vec![dir("https://train.test/api")])
    }
}

/// Evaluation-side runner for a different host. The probe reveals `/api`; then
/// `/api/<token>` is discoverable ONLY if the scan actually requested that token
/// (word-gating), so a carried-over prediction is the sole way to reach it.
struct EvalRunner;
impl FeroxRunner for EvalRunner {
    fn run(&self, args: &FeroxArgs) -> anyhow::Result<Vec<FeroxResponse>> {
        // the fingerprint probe asks for the known probe paths (incl. wp-json)
        if args.words.iter().any(|w| w == "wp-json") {
            return Ok(vec![dir("https://eval.test/api")]);
        }
        // expansion of a directory: reveal only paths whose final segment was asked
        let mut out = Vec::new();
        if args.url == "https://eval.test/api/" {
            if args.words.iter().any(|w| w == CARRIED_TOKEN) {
                out.push(dir(&format!("https://eval.test/api/{CARRIED_TOKEN}")));
            }
        }
        Ok(out)
    }
}

fn eval_cfg(model_path: &str) -> Config {
    Config {
        // keep the carried token visible: no probability floor, generous top-N,
        // soft-404 filter off (the fake has no soft-404 noise).
        probability_threshold: 0.0,
        top_n: 50,
        use_soft404_filter: false,
        max_rounds: 10,
        max_depth: 4,
        model_path: model_path.to_string(),
        ..Config::default()
    }
}

fn found_carried(summary: &ferox_ml_core::orchestrator::Summary) -> bool {
    summary
        .discovered
        .iter()
        .any(|(u, _)| last_segment(u) == CARRIED_TOKEN)
}

#[test]
fn carried_model_reaches_a_path_a_cold_scan_misses() {
    let dir_tmp = std::env::temp_dir().join(format!("ferox-xrun-{}", std::process::id()));
    std::fs::create_dir_all(&dir_tmp).unwrap();
    let model_path = dir_tmp.join("carried.model").to_string_lossy().to_string();

    // ---- phase 1: train on train.test and persist the model ----
    let mut train_cfg = eval_cfg(&model_path);
    train_cfg.model_path = String::new(); // learn() writes to model_out, not model_path
    let learn = Campaign::new(train_cfg, Box::new(TrainRunner))
        .learn(&["https://train.test".to_string()], &model_path)
        .unwrap();
    assert!(learn.paths_ingested > 0 || learn.transitions > 0, "learn produced nothing: {learn:?}");
    assert!(std::path::Path::new(&model_path).exists(), "model was not written");

    // ---- phase 2a: WARM scan on a different host, with the carried model ----
    let warm = Campaign::new(eval_cfg(&model_path), Box::new(EvalRunner))
        .run("https://eval.test")
        .unwrap();

    // ---- phase 2b: COLD scan, identical except no carried model ----
    let cold = Campaign::new(eval_cfg(""), Box::new(EvalRunner))
        .run("https://eval.test")
        .unwrap();

    assert!(
        found_carried(&warm),
        "warm scan should reuse the learned token to reach /api/{CARRIED_TOKEN}: {:?}",
        warm.discovered
    );
    assert!(
        !found_carried(&cold),
        "cold scan cannot predict the non-seeded token, so must miss it: {:?}",
        cold.discovered
    );

    let _ = std::fs::remove_dir_all(&dir_tmp);
}
