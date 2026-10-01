//! Shared ML engines for the YABFF content-discovery tools.
//!
//! Both [`feroxbuster-ml`](https://github.com/maxamin/YABFF) (the in-crate `--ml`
//! fork) and `feroxml` (the standalone orchestrator) depend on this crate, so the
//! engines live in exactly one place:
//!
//! * [`fingerprint`] — K-Means / nearest-centroid framework fingerprinting over a
//!   bounded probe (see [`profiles`] for the centroids and probe paths);
//! * [`markov`] — variable-order Markov / PPM path prediction, seeded per profile;
//! * [`scheduler`] — Thompson / UCB1 / round-robin bandits over directories;
//! * [`ranking`] — BM25 re-ranking against the discovered-path corpus;
//! * [`dedup`] — SimHash + response-signature soft-404 detection;
//! * [`tokenize`], [`rng`], [`interfaces`] — supporting primitives and the
//!   swap-in traits ([`interfaces::Classifier`], [`interfaces::Predictor`],
//!   [`interfaces::Scheduler`]).
//!
//! Engines consume [`ProbeResp`], a lightweight response view, so they never
//! depend on either tool's own response types and stay unit-testable in isolation.

use std::collections::HashMap;

pub mod dedup;
pub mod fingerprint;
pub mod interfaces;
pub mod markov;
pub mod profiles;
pub mod ranking;
pub mod rng;
pub mod scheduler;
pub mod tokenize;

pub use profiles::PROFILES;

/// Lightweight view of a probe/scan response for fingerprinting and learning.
///
/// Each tool builds this from its own native response type (feroxbuster's
/// `FeroxResponse`, an NDJSON record, ...) so the engines stay decoupled.
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
