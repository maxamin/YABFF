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

pub mod algo;
pub mod dedup;
pub mod dynsdt;
pub mod fingerprint;
pub mod freqtrie;
pub mod interfaces;
pub mod markov;
pub mod profiles;
pub mod ranking;
pub mod rng;
pub mod scheduler;
pub mod tokenize;
pub mod tst;

// The adaptive orchestration layer: a runner-agnostic budgeted feedback loop that
// drives a `FeroxRunner` (subprocess today; an in-process runner can implement the
// same trait). Shared so either tool can host the loop.
pub mod config;
pub mod ferox;
pub mod orchestrator;
pub mod scope;
pub mod wordlist;

pub use profiles::PROFILES;

use std::sync::atomic::{AtomicBool, Ordering};

/// Cooperative stop flag. A host (e.g. the `--ml-loop` driver) sets this from a
/// signal handler; the orchestrator's round loop checks it each round and exits
/// cleanly, persisting the model on the way out. Signal handlers can only touch
/// async-signal-safe state, so they set this atomic and let normal code do the
/// saving — never save from inside the handler itself.
static STOP_REQUESTED: AtomicBool = AtomicBool::new(false);

/// Request that in-flight campaigns stop at the next round boundary.
pub fn request_stop() {
    STOP_REQUESTED.store(true, Ordering::SeqCst);
}

/// Whether a stop has been requested via [`request_stop`].
pub fn stop_requested() -> bool {
    STOP_REQUESTED.load(Ordering::SeqCst)
}

/// Lightweight view of a probe/scan response for fingerprinting and learning.
///
/// Each tool builds this from its own native response type (feroxbuster's
/// `FeroxResponse`, an NDJSON record, ...) so the engines stay decoupled. The
/// `content_length` / `word_count` / `line_count` fields feed the coarse
/// [`dedup::Signature`](crate::dedup::Signature) used for per-path soft-404
/// scoring (E2); they default to 0 when a caller has no body data, which simply
/// makes soft-404 scoring a no-op.
#[derive(Debug, Clone, Default)]
pub struct ProbeResp {
    pub url: String,
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub content_length: u64,
    pub word_count: u64,
    pub line_count: u64,
    /// 64-bit SimHash of the response body, or 0 when the caller has no body
    /// (e.g. an NDJSON record). Folded into [`signature`](Self::signature) so the
    /// soft-404 filter's body-template match (Layer 2b) can run.
    pub body_simhash: u64,
}

impl ProbeResp {
    pub fn new(url: &str, status: u16) -> Self {
        Self {
            url: url.to_string(),
            status,
            ..Self::default()
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

    /// Coarse response signature for soft-404 / near-duplicate detection. For a
    /// redirect the `Location` target is folded in (see [`dedup::Signature`]), so
    /// distinct-target redirects don't collapse onto a catch-all probe's.
    pub fn signature(&self) -> crate::dedup::Signature {
        let sig = crate::dedup::Signature::new(
            self.status,
            self.content_length,
            self.word_count,
            self.line_count,
        );
        if (300..400).contains(&self.status) {
            sig.with_location(self.header("location"))
        } else {
            sig.with_simhash(self.body_simhash)
        }
    }
}
