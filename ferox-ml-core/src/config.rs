//! Every tunable parameter. Defaults run the Tier-1 engine set with only the
//! crate's base dependencies. A TOML file (`--config path.toml`, matching
//! feroxbuster's own `ferox-config.toml` convention) overrides defaults, and
//! CLI flags override the file.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    // --- engine selection (advanced-algorithm feature flags) ---
    /// `nearest_centroid` (default) | `kmeans`
    pub classifier: String,
    /// Prediction algorithm: `markov` (default — the best by benchmarked results:
    /// it ties the tree models for list-mode reach and wins cold-start via its
    /// profile seed) | `trie` | `dynsdt` | `tst` | `auto` (DynSDT in list mode, the
    /// profile-seeded Markov model otherwise). See [`crate::algo`].
    #[serde(default)]
    pub algo: String,
    /// `ppm` (variable-order, default) | `markov1` — Markov order knob (only used
    /// when the selected `algo` is the Markov model).
    pub predictor: String,
    /// `thompson` (default) | `ucb1` | `round_robin`
    pub scheduler: String,
    /// `bm25` (default) | `none`
    pub ranker: String,
    /// drop soft-404 / template responses before they reach the reward signal
    pub use_soft404_filter: bool,

    // --- prediction ---
    pub top_n: usize,
    pub probability_threshold: f64,
    pub laplace_alpha: f64,
    pub markov_max_order: usize,

    // --- feedback-loop budget ---
    pub max_rounds: usize,
    pub max_depth: usize,
    pub request_budget: usize,

    // --- feroxbuster process controls ---
    pub ferox_binary: String,
    pub threads: usize,
    pub rate_limit: usize,
    pub success_codes: Vec<u16>,
    pub extensions: Vec<String>,
    pub tls_verify: bool,
    pub scan_time_limit: String,
    /// Let feroxbuster extract links from responses. Off by default so that every
    /// discovery is attributable to feroxml's own prediction engine rather than
    /// feroxbuster's crawler (turning it on mixes the two and inflates "predicted
    /// hits"). Turn on only when you want maximum coverage over attribution.
    pub ferox_extract_links: bool,

    // --- hybrid coverage (optional; default off keeps discoveries pure-ML) ---
    /// Path to a base wordlist. When set, each expansion also tries up to
    /// `seed_per_round` entries from it alongside the Markov predictions, giving
    /// baseline coverage on targets that don't match a seed profile. Empty = the
    /// engine relies on prediction alone.
    pub seed_wordlist: String,
    /// How many base-wordlist entries to add per expansion (0 = pure prediction).
    pub seed_per_round: usize,

    // --- determinism / scope ---
    pub seed: u64,
    pub scope: Vec<String>,

    // --- persistence / learning ---
    pub state_dir: String,
    /// Path to a learned Markov model (JSON). In scan mode, if it exists it is
    /// merged onto the profile seed so the engine starts from prior learning and
    /// the run's online updates are saved back. In learn mode, this is where the
    /// model trained from the labs is written.
    pub model_path: String,

    // --- directory-of-wordlists driver (list mode) ---
    /// Directory containing wordlist files. When set, the fingerprint/seed phase
    /// is skipped entirely and scanning is driven by these lists.
    #[serde(default)]
    pub list_dir: String,
    /// How many list entries to inject per round.
    #[serde(default = "default_list_chunk")]
    pub list_chunk_size: usize,
    /// Max entries loaded from the wordlist directory (the whole tree, recursively).
    /// `0` = unlimited — load every entry in the directory tree.
    #[serde(default)]
    pub list_max_entries: usize,
}

fn default_list_chunk() -> usize {
    200
}

impl Default for Config {
    fn default() -> Self {
        Self {
            classifier: "nearest_centroid".into(),
            algo: "markov".into(),
            predictor: "ppm".into(),
            scheduler: "thompson".into(),
            ranker: "bm25".into(),
            use_soft404_filter: true,
            top_n: 12,
            probability_threshold: 0.02,
            laplace_alpha: 0.5,
            markov_max_order: 3,
            max_rounds: 25,
            max_depth: 4,
            request_budget: 20_000,
            ferox_binary: "feroxbuster".into(),
            threads: 20,
            rate_limit: 0,
            success_codes: vec![200, 204, 301, 302, 307, 401, 403, 405],
            extensions: vec![],
            tls_verify: true,
            scan_time_limit: String::new(),
            ferox_extract_links: false,
            seed_wordlist: String::new(),
            seed_per_round: 0,
            seed: 1337,
            scope: vec![],
            state_dir: ".feroxml".into(),
            model_path: String::new(),
            list_dir: String::new(),
            list_chunk_size: default_list_chunk(),
            list_max_entries: 0, // unlimited: use every entry in the tree
        }
    }
}

impl Config {
    /// Load a TOML config file, falling back to defaults for anything absent.
    pub fn from_toml_file(path: &str) -> anyhow::Result<Self> {
        let text = std::fs::read_to_string(path)?;
        let cfg: Config = toml::from_str(&text)?;
        Ok(cfg)
    }
}
