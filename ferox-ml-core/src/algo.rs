//! Algorithm selector — pick the prediction model by name, for reuse and A/B.
//!
//! Every model implements [`PathModel`](crate::interfaces::PathModel), so the
//! orchestrator holds a `Box<dyn PathModel>` and never names a concrete type. The
//! `--algo` / `--ml-algo` flag (config field [`Config::algo`](crate::config::Config))
//! chooses one; `auto` keeps the historical behavior (DynSDT in list mode, the
//! profile-seeded Markov model otherwise).
//!
//! | name     | module                       | kind                         |
//! |----------|------------------------------|------------------------------|
//! | `markov` | [`crate::markov`]            | variable-order Markov / PPM  |
//! | `trie`   | [`crate::freqtrie`]          | plain frequency prefix trie  |
//! | `dynsdt` | [`crate::dynsdt`]            | score-decomposed trie (DEPQ) |
//! | `tst`    | [`crate::tst`]               | ternary search tree          |

use crate::config::Config;
use crate::dynsdt::DynSdt;
use crate::freqtrie::FreqTrie;
use crate::interfaces::PathModel;
use crate::markov::MarkovModel;
use crate::tst::Tst;

/// A selectable prediction algorithm.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Algo {
    Markov,
    Trie,
    DynSdt,
    Tst,
}

impl Algo {
    /// Every algorithm, for iteration (benchmarks, `--help`, sweeps).
    pub const ALL: [Algo; 4] = [Algo::Markov, Algo::Trie, Algo::DynSdt, Algo::Tst];

    /// Canonical name, as accepted by [`Algo::parse`] and stored in summaries.
    pub fn name(self) -> &'static str {
        match self {
            Algo::Markov => "markov",
            Algo::Trie => "trie",
            Algo::DynSdt => "dynsdt",
            Algo::Tst => "tst",
        }
    }

    /// One-line description for `--help` and reports.
    pub fn describe(self) -> &'static str {
        match self {
            Algo::Markov => "variable-order Markov/PPM; generalizes across contexts via back-off",
            Algo::Trie => "frequency prefix trie; top-k by subtree scan (naive-tree baseline)",
            Algo::DynSdt => "dynamic score-decomposed trie; top-k via best-first DEPQ, O(|p|+k log k)",
            Algo::Tst => "ternary search tree; BST-linked trie layout, top-k by subtree scan",
        }
    }

    /// Parse a selector string (case-insensitive, with common aliases).
    /// Returns `None` for unknown names and for `auto`/empty (see [`resolve`]).
    pub fn parse(s: &str) -> Option<Algo> {
        match s.trim().to_ascii_lowercase().as_str() {
            "markov" | "ppm" | "markov1" => Some(Algo::Markov),
            "trie" | "freqtrie" | "prefix" => Some(Algo::Trie),
            "dynsdt" | "sdt" | "dyn" => Some(Algo::DynSdt),
            "tst" | "ternary" => Some(Algo::Tst),
            _ => None,
        }
    }
}

/// Which algorithm a campaign should use: an explicit `cfg.algo` wins; `auto` (or
/// empty / unrecognized) picks DynSDT in list mode and Markov otherwise.
pub fn resolve(cfg: &Config, list_mode: bool) -> Algo {
    Algo::parse(&cfg.algo).unwrap_or(if list_mode { Algo::DynSdt } else { Algo::Markov })
}

/// Build the selected model. `profile` and `list_mode` only affect Markov seeding
/// (the tree models always start empty and learn purely from scan hits).
pub fn build(algo: Algo, cfg: &Config, profile: &str, list_mode: bool) -> Box<dyn PathModel> {
    match algo {
        Algo::Markov => {
            let order = if cfg.predictor == "markov1" {
                1
            } else {
                cfg.markov_max_order
            };
            let m = if list_mode {
                MarkovModel::new(order, cfg.laplace_alpha, cfg.probability_threshold)
            } else {
                MarkovModel::seeded(profile, order, cfg.laplace_alpha, cfg.probability_threshold)
            };
            Box::new(m)
        }
        Algo::Trie => Box::new(FreqTrie::new()),
        Algo::DynSdt => Box::new(DynSdt::new()),
        Algo::Tst => Box::new(Tst::new()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_accepts_names_and_aliases() {
        assert_eq!(Algo::parse("DynSDT"), Some(Algo::DynSdt));
        assert_eq!(Algo::parse(" ppm "), Some(Algo::Markov));
        assert_eq!(Algo::parse("ternary"), Some(Algo::Tst));
        assert_eq!(Algo::parse("trie"), Some(Algo::Trie));
        assert_eq!(Algo::parse("auto"), None);
        assert_eq!(Algo::parse("bogus"), None);
    }

    #[test]
    fn resolve_defaults_to_markov_and_honors_auto() {
        // the default is now `markov` (best by benchmarked results), in both modes
        let cfg = Config::default();
        assert_eq!(resolve(&cfg, true), Algo::Markov);
        assert_eq!(resolve(&cfg, false), Algo::Markov);
        // `auto` is still available and stays mode-aware
        let mut auto = Config::default();
        auto.algo = "auto".into();
        assert_eq!(resolve(&auto, true), Algo::DynSdt);
        assert_eq!(resolve(&auto, false), Algo::Markov);
        // an explicit algorithm wins
        let mut cfg2 = Config::default();
        cfg2.algo = "tst".into();
        assert_eq!(resolve(&cfg2, true), Algo::Tst);
    }

    #[test]
    fn build_produces_working_models_that_learn_and_persist() {
        let cfg = Config::default();
        for a in Algo::ALL {
            let mut m = build(a, &cfg, "GENERIC", true);
            m.learn("https://x/api/v1");
            m.learn("https://x/api/v1");
            m.learn("https://x/api/v2");
            let preds = m.predict("https://x/api", 5);
            assert!(
                preds.iter().any(|(t, _)| t == "v1"),
                "{} should predict v1: {preds:?}",
                a.name()
            );
            // round-trips through its own JSON
            let json = m.save_json().unwrap();
            let mut fresh = build(a, &cfg, "GENERIC", true);
            fresh.merge_json(&json).unwrap();
            assert!(fresh.predict("https://x/api", 5).iter().any(|(t, _)| t == "v1"));
        }
    }
}
