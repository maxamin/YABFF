//! Native ML layer for feroxbuster.
//!
//! This is the feroxml engine set ported in-crate: K-Means/nearest-centroid
//! fingerprinting, variable-order Markov/PPM path prediction, a Thompson-sampling
//! scheduler, BM25 ranking, and SimHash soft-404 detection. The engines are
//! pure-Rust and self-contained; [`ProbeResp`] is the lightweight response view
//! the fingerprinter consumes, so the ML layer never depends on the scanner's
//! own types and stays unit-testable in isolation.
//!
//! Runtime wiring (see `main.rs` / `event_handlers::outputs`): when `--ml` is set,
//! feroxbuster probes the target, [`fingerprint`]s a profile, seeds a Markov model
//! (merging any model at `--ml-model`), injects [`predict_words`] into the scan,
//! [`observe`]s every discovered URL to learn online, and [`save`]s on shutdown.

pub mod dedup;
pub mod fingerprint;
pub mod interfaces;
pub mod markov;
pub mod profiles;
pub mod ranking;
pub mod rng;
pub mod scheduler;
pub mod tokenize;

use std::collections::HashMap;
use std::sync::RwLock;

use lazy_static::lazy_static;

use interfaces::Predictor;
use markov::MarkovModel;

/// Lightweight view of a probe/scan response for fingerprinting and learning.
#[derive(Debug, Clone, Default)]
pub struct ProbeResp {
    pub url: String,
    pub status: u16,
    pub headers: HashMap<String, String>,
}

impl ProbeResp {
    pub fn new(url: &str, status: u16) -> Self {
        Self {
            url: url.to_string(),
            status,
            headers: HashMap::new(),
        }
    }

    /// Case-insensitive header lookup.
    pub fn header(&self, name: &str) -> Option<&str> {
        let want = name.to_lowercase();
        self.headers
            .iter()
            .find(|(k, _)| k.to_lowercase() == want)
            .map(|(_, v)| v.as_str())
    }
}

/// The discriminating paths the fingerprint probe requests.
pub fn probe_paths() -> &'static [&'static str] {
    &profiles::PROBE_PATHS
}

/// Classify probe responses into a framework profile + centroid distances.
pub fn fingerprint(probes: &[ProbeResp]) -> (String, Vec<(String, f64)>) {
    use interfaces::Classifier;
    let fv = fingerprint::feature_vector(probes);
    fingerprint::NearestCentroid.classify(&fv)
}

lazy_static! {
    /// The active learned model, shared across scan tasks. `None` until `init`.
    static ref ML: RwLock<Option<MlState>> = RwLock::new(None);
}

struct MlState {
    model: MarkovModel,
    model_path: String,
}

/// Seed a model for `profile`, merge any model persisted at `model_path`, and
/// install it as the active model. Idempotent; a later call replaces the state.
pub fn init(profile: &str, model_path: &str, max_order: usize, alpha: f64, threshold: f64) {
    let mut model = MarkovModel::seeded(profile, max_order, alpha, threshold);
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

/// Learn from a discovered URL (online update of the active model).
pub fn observe(url: &str) {
    if let Ok(mut guard) = ML.write() {
        if let Some(state) = guard.as_mut() {
            state.model.learn(url);
        }
    }
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
        reset();
        let dir = std::env::temp_dir().join(format!("ferox-ml-{}", std::process::id()));
        let _ = std::fs::create_dir_all(&dir);
        let model_path = dir.join("model.json").to_string_lossy().to_string();

        init("REST_API", &model_path, 3, 0.5, 0.02);
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
        init("LEGACY_STATIC", &model_path, 3, 0.5, 0.02);
        let merged = predict_words("https://x.test/shop/checkout", 5);
        assert!(merged.iter().any(|w| w == "receipt"), "merged={merged:?}");

        reset();
        let _ = std::fs::remove_dir_all(&dir);
    }
}
