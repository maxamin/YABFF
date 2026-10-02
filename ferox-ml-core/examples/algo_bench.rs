//! Deep-target benchmark of the `--algo` prediction models.
//!
//! Deterministic and network-free. A synthetic **deep, repetitive** site
//! (`{api,app,shop}/v{1,2,3}/{8 leaves}`) is used two ways:
//!
//! 1. **Prediction recall (the algorithm signal).** Each model learns one version's
//!    branches (`*/v1/*`) and is then asked to predict the children of the *same*
//!    version (memorization) and of the *held-out* versions `v2`/`v3`
//!    (generalization). This is where the algorithms diverge: the n-gram Markov
//!    model backs off across contexts and predicts the repeated leaf names in a
//!    version it never saw; the exact-prefix tree models (trie / DynSDT / TST) can
//!    only return children of a prefix they have literally observed.
//!
//! 2. **End-to-end list-mode scan (a systems note).** Each model drives the real
//!    [`Campaign`] loop over a shared wordlist and budget. List mode re-applies the
//!    wordlist to every directory (per-arm cursors), so the scan fully recurses and
//!    every model reaches full coverage; the predictor affects efficiency, and only
//!    changes *reach* for paths the wordlist doesn't contain.
//!
//! Run: `cargo run -p ferox-ml-core --example algo_bench [--release]`

use std::collections::HashSet;

use ferox_ml_core::algo::{self, Algo};
use ferox_ml_core::config::Config;
use ferox_ml_core::ferox::{FeroxArgs, FeroxResponse, FeroxRunner};
use ferox_ml_core::orchestrator::Campaign;
use ferox_ml_core::tokenize::path_segments;

const HOST: &str = "http://deep.test";
const BASES: [&str; 3] = ["api", "app", "shop"];
const VERSIONS: [&str; 3] = ["v1", "v2", "v3"];
const LEAVES: [&str; 8] = [
    "users", "orders", "admin", "config", "backup", "status", "health", "login",
];

// ---------------------------------------------------------------------------
// 1. Prediction-recall benchmark
// ---------------------------------------------------------------------------

/// Fraction of `truth` leaf names the model returns in its top-k for `prefix`.
fn recall_at_k(model: &dyn ferox_ml_core::interfaces::PathModel, prefix: &str, k: usize) -> f64 {
    let preds: HashSet<String> = model
        .predict(prefix, k)
        .into_iter()
        .map(|(t, _)| t.trim_matches('/').to_string())
        .collect();
    let hit = LEAVES.iter().filter(|l| preds.contains(**l)).count();
    hit as f64 / LEAVES.len() as f64
}

fn prediction_recall() {
    println!("1. PREDICTION RECALL  (train on */v1/*, k=8)\n");
    println!(
        "{:<8} {:>14} {:>16}",
        "algo", "seen v1 (recall)", "held-out v2,v3"
    );

    let cfg = Config::default();
    for algo in Algo::ALL {
        // list_mode=true => even Markov starts empty (no profile seed): this isolates
        // online, learned generalization rather than a hand-seeded matrix.
        let mut model = algo::build(algo, &cfg, "", true);

        // learn the full directory structure + only v1's leaves
        for b in BASES {
            model.learn(&format!("{HOST}/{b}"));
            for v in VERSIONS {
                model.learn(&format!("{HOST}/{b}/{v}")); // v1,v2,v3 all known as dirs
            }
            for l in LEAVES {
                model.learn(&format!("{HOST}/{b}/v1/{l}")); // leaves only under v1
            }
        }

        // seen: predict children of a v1 dir it learned
        let seen: f64 = BASES
            .iter()
            .map(|b| recall_at_k(model.as_ref(), &format!("{HOST}/{b}/v1"), 8))
            .sum::<f64>()
            / BASES.len() as f64;

        // held-out: predict children of v2 / v3 dirs (same leaf names, unseen prefix)
        let mut held = 0.0;
        let mut n = 0;
        for b in BASES {
            for v in ["v2", "v3"] {
                held += recall_at_k(model.as_ref(), &format!("{HOST}/{b}/{v}"), 8);
                n += 1;
            }
        }
        held /= n as f64;

        println!("{:<8} {:>13.0}% {:>15.0}%", algo.name(), seen * 100.0, held * 100.0);
    }
    println!();
}

// ---------------------------------------------------------------------------
// 1b. Cold-start recall from a profile seed (zero observations)
// ---------------------------------------------------------------------------

fn cold_start_recall() {
    println!("1b. COLD-START RECALL  (REST_API profile seed, zero observations, k=12)\n");
    // the canonical deep structure the seed encodes, keyed by last segment
    let seed = ferox_ml_core::profiles::seed_matrix("REST_API");
    let queries = [("/", ""), ("/api", "api"), ("/api/v1", "v1"), ("/api/v2", "v2")];

    println!(
        "{:<8} {:>9} {:>9} {:>11} {:>11}",
        "algo", "/", "/api", "/api/v1", "/api/v2"
    );
    let cfg = Config::default();
    for algo in Algo::ALL {
        // non-list build: Markov is seeded from the profile, the tree models start
        // empty (they have no cold-start seeding mechanism).
        let model = algo::build(algo, &cfg, "REST_API", false);
        print!("{:<8}", algo.name());
        for (url_suffix, row_key) in queries {
            let expected: Vec<&str> = seed
                .iter()
                .find(|(k, _)| *k == row_key)
                .map(|(_, toks)| toks.iter().map(|(t, _)| *t).collect())
                .unwrap_or_default();
            let got: HashSet<String> = model
                .predict(&format!("{HOST}{url_suffix}"), 12)
                .into_iter()
                .map(|(t, _)| t.trim_matches('/').to_string())
                .collect();
            let hit = expected.iter().filter(|t| got.contains(**t)).count();
            let pct = if expected.is_empty() {
                0.0
            } else {
                100.0 * hit as f64 / expected.len() as f64
            };
            let width = url_suffix.len().max(9);
            print!(" {:>width$}", format!("{pct:.0}%"), width = width);
        }
        println!();
    }
    println!(
        "\nOnly Markov carries a profile seed, so it predicts the canonical REST tree\n\
         (v1/v2 and their children) with no observations — the documented source of\n\
         its discovery edge on structured targets. The tree models learn purely from\n\
         what they observe, so cold-start recall is 0%.\n"
    );
}

// ---------------------------------------------------------------------------
// 2. End-to-end list-mode scan (systems note)
// ---------------------------------------------------------------------------

struct Site {
    dirs: HashSet<String>,
    leaves: HashSet<String>,
}

impl Site {
    fn build() -> Self {
        let mut dirs = HashSet::new();
        let mut leaves = HashSet::new();
        for b in BASES {
            dirs.insert(b.to_string());
            for v in VERSIONS {
                dirs.insert(format!("{b}/{v}"));
                for l in LEAVES {
                    leaves.insert(format!("{b}/{v}/{l}"));
                }
            }
        }
        Site { dirs, leaves }
    }
    fn total(&self) -> usize {
        self.dirs.len() + self.leaves.len()
    }
    fn tokens(&self) -> Vec<String> {
        let mut t: HashSet<String> = HashSet::new();
        for p in self.dirs.iter().chain(self.leaves.iter()) {
            for seg in p.split('/') {
                t.insert(seg.to_string());
            }
        }
        let mut v: Vec<String> = t.into_iter().collect();
        v.sort();
        v
    }
}

impl FeroxRunner for Site {
    fn run(&self, args: &FeroxArgs) -> anyhow::Result<Vec<FeroxResponse>> {
        let mut out = Vec::new();
        for w in &args.words {
            let full = format!("{}{}", args.url, w);
            let key = path_segments(&full).join("/");
            if self.dirs.contains(&key) {
                out.push(resp(&format!("{full}/"), 200)); // 2xx + trailing '/' = dir
            } else if self.leaves.contains(&key) {
                out.push(resp(&full, 200));
            } else {
                out.push(resp(&full, 404));
            }
        }
        Ok(out)
    }
}

fn resp(url: &str, status: u16) -> FeroxResponse {
    FeroxResponse {
        url: url.to_string(),
        status,
        content_length: 100,
        word_count: 10,
        line_count: 3,
        ..Default::default()
    }
}

fn end_to_end_scan() {
    let site = Site::build();
    let total = site.total();
    let tokens = site.tokens();

    let dir = std::env::temp_dir().join(format!("algo-bench-{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let mut lines = tokens.clone();
    for i in 0..60 {
        lines.push(format!("zzjunk{i:03}"));
    }
    std::fs::write(dir.join("words.txt"), lines.join("\n")).unwrap();
    let list_dir = dir.to_string_lossy().into_owned();

    println!(
        "2. END-TO-END LIST-MODE SCAN  (site: {} resources = {} dirs + {} leaves; {} tokens + 60 junk)\n",
        total,
        site.dirs.len(),
        site.leaves.len(),
        tokens.len()
    );
    println!("{:<8} {:>7} {:>8} {:>7} {:>8}", "algo", "found", "cover%", "reqs", "rounds");
    for algo in ["markov", "trie", "dynsdt", "tst"] {
        let cfg = Config {
            list_dir: list_dir.clone(),
            list_chunk_size: 4,
            algo: algo.to_string(),
            ranker: "none".into(),
            use_soft404_filter: false,
            max_rounds: 400,
            request_budget: 2000,
            max_depth: 4,
            top_n: 12,
            seed: 1337,
            ..Config::default()
        };
        let s = Campaign::new(cfg, Box::new(Site::build())).run(HOST).unwrap();
        println!(
            "{:<8} {:>7} {:>7.0}% {:>7} {:>8}",
            algo,
            s.discovered.len(),
            100.0 * s.discovered.len() as f64 / total as f64,
            s.requests_used,
            s.rounds
        );
    }
    println!(
        "\nList mode re-applies the wordlist to every directory (per-arm cursors), so\n\
         the scan fully recurses and every model reaches 100% coverage. The predictor\n\
         then affects efficiency (requests to full coverage) rather than reach; its\n\
         real reach advantage shows only for paths the wordlist lacks (see 1b)."
    );
    let _ = std::fs::remove_dir_all(&dir);
}

fn main() {
    println!("Deep-target algorithm benchmark (synthetic, deterministic, in-process)\n");
    prediction_recall();
    cold_start_recall();
    end_to_end_scan();
}
