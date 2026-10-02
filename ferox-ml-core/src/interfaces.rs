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

/// A persistable prediction model, selectable at runtime via [`crate::algo`].
///
/// Every autocomplete algorithm (Markov/PPM, a plain frequency trie, the DynSDT,
/// a ternary search tree, ...) implements this one interface, so the orchestrator
/// treats them interchangeably and the `--algo` selector can swap them freely.
/// [`save_json`](PathModel::save_json) / [`merge_json`](PathModel::merge_json)
/// give cross-run persistence (more runs => better model).
pub trait PathModel: Predictor {
    /// Serialize the whole model to JSON for persistence.
    fn save_json(&self) -> anyhow::Result<String>;

    /// Merge a previously persisted model of the **same** algorithm (its JSON).
    /// A format mismatch returns an error and leaves `self` unchanged.
    fn merge_json(&mut self, text: &str) -> anyhow::Result<()>;
}

/// Phase 3 — decide which discovered directory ("arm") to expand next.
pub trait Scheduler {
    fn add_arm(&mut self, arm: &str);
    /// `reward` is in `[0, 1]` (e.g. hit-rate of the last expansion).
    fn update(&mut self, arm: &str, reward: f64);
    /// The arm to expand next, or `None` when every arm is exhausted.
    fn choose(&mut self) -> Option<String>;

    /// Current value estimate for an arm in `[0, 1]` (the posterior mean for a
    /// Beta bandit, a normalized confidence bound for UCB1). Used to scale how
    /// much request budget the arm earns. Defaults to a neutral `0.5` for arms
    /// that don't track a value (e.g. round-robin) or that were never seen.
    fn value(&self, _arm: &str) -> f64 {
        0.5
    }
}
