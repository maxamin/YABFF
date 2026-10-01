//! Swap-in traits. The orchestrator talks only to these, so any advanced
//! algorithm from the roadmap (LinUCB, GBDT fingerprinting, MCTS, an LSTM path
//! generator, ...) drops in behind the same trait without orchestrator changes.

/// Phase 1 — classify a numeric feature vector into a framework profile.
pub trait Classifier {
    /// Returns `(profile_name, distances_by_profile)`. Lower distance = closer.
    fn classify(&self, features: &[f64]) -> (String, Vec<(String, f64)>);
}

/// Phase 2 — predict likely next path tokens for a discovered path.
pub trait Predictor {
    /// Returns `(token, probability)` pairs, highest probability first.
    fn predict(&self, path: &str, top_n: usize) -> Vec<(String, f64)>;

    /// Online update from a newly discovered path.
    fn learn(&mut self, path: &str);
}

/// Phase 3 — decide which discovered directory ("arm") to expand next.
pub trait Scheduler {
    fn add_arm(&mut self, arm: &str);
    /// `reward` is in `[0, 1]` (e.g. hit-rate of the last expansion).
    fn update(&mut self, arm: &str, reward: f64);
    /// The arm to expand next, or `None` when every arm is exhausted.
    fn choose(&mut self) -> Option<String>;
}
