//! Phase 3 — the adaptive feedback loop.
//!
//! feroxbuster cannot take new words into a running scan, so this drives a
//! sequence of **bounded** scans: probe → classify → seed predictor → then, each
//! round, let the scheduler pick a directory, predict+rank its likely children,
//! scan them, score the result, learn, and enqueue any new directories. The
//! [`FeroxRunner`] is injected, so this whole loop runs offline in tests.

use crate::config::Config;
use crate::dedup::{Signature, SoftNotFoundFilter};
use crate::ferox::{FeroxArgs, FeroxRunner, FeroxResponse};
use crate::fingerprint::{self, feature_vector};

/// Bridge feroxbuster's `FeroxResponse` records into the engine's `ProbeResp`
/// view (the engines live in `ferox-ml-core` and don't know feroxbuster's types).
fn to_probe_views(resps: &[FeroxResponse]) -> Vec<crate::ProbeResp> {
    resps
        .iter()
        .map(|r| crate::ProbeResp {
            url: r.url.clone(),
            status: r.status,
            headers: r.headers.clone(),
            content_length: r.content_length,
            word_count: r.word_count,
            line_count: r.line_count,
        })
        .collect()
}
use crate::interfaces::{Classifier, Predictor};
use crate::markov::MarkovModel;
use crate::profiles::PROBE_PATHS;
use crate::ranking::Bm25;
use crate::rng::Rng;
use crate::scheduler;
use crate::scope::Scope;
use crate::tokenize::{last_segment, path_segments};
use crate::wordlist::{build_round_wordlist, load_wordlist, merge_seed, SeenPaths};

/// Everything the caller learns from a campaign.
#[derive(Debug, Clone)]
pub struct Summary {
    pub target: String,
    pub profile: String,
    pub distances: Vec<(String, f64)>,
    pub rounds: usize,
    pub requests_used: usize,
    pub discovered: Vec<(String, u16)>,
    pub predicted_hits: usize,
    pub dropped_out_of_scope: usize,
    pub filtered_soft404: usize,
}

pub struct Campaign {
    cfg: Config,
    runner: Box<dyn FeroxRunner>,
}

impl Campaign {
    pub fn new(cfg: Config, runner: Box<dyn FeroxRunner>) -> Self {
        Self { cfg, runner }
    }

    fn markov_max_order(&self) -> usize {
        if self.cfg.predictor == "markov1" {
            1
        } else {
            self.cfg.markov_max_order
        }
    }

    /// Deterministic pseudo-random probe paths used to learn soft-404 sigs.
    fn random_probe_paths(&self, n: usize) -> Vec<String> {
        let mut rng = Rng::new(self.cfg.seed ^ 0xA5A5_5A5A);
        (0..n)
            .map(|_| {
                let hi = rng.uniform();
                format!("feroxml-probe-{:08x}", (hi * 4.294_967e9) as u32)
            })
            .collect()
    }

    pub fn run(&mut self, target: &str) -> anyhow::Result<Summary> {
        let scope = Scope::new(target, &self.cfg.scope)?;

        // ---- Phase 1: probe + fingerprint ----
        let mut probe_words: Vec<String> =
            PROBE_PATHS.iter().map(|s| s.to_string()).collect();
        let random_paths = self.random_probe_paths(3);
        probe_words.extend(random_paths.iter().cloned());

        let probe_resps = self.runner.run(&FeroxArgs {
            url: target.to_string(),
            words: probe_words,
            no_recursion: true,
            all_codes: true, // fingerprint must see 500/401/etc., not just successes
            extract_links: false,
        })?;

        // E2: learn the soft-404 signature from the random probes, then demote any
        // discriminating probe that returns the same soft-404 body to "absent", so
        // per-path presence reflects real content, not a catch-all's uniform body.
        let (random_views, mut probe_views): (Vec<_>, Vec<_>) = to_probe_views(&probe_resps)
            .into_iter()
            .partition(|v| random_paths.iter().any(|r| v.url.contains(r.as_str())));
        let soft404 = fingerprint::learn_soft_404(&random_views);
        fingerprint::apply_soft_404(&mut probe_views, &soft404);

        let features = feature_vector(&probe_views);
        let classifier: Box<dyn Classifier> =
            fingerprint::build(&self.cfg.classifier, self.cfg.seed);
        let (mut profile, distances) = classifier.classify(&features);
        // E1 catch-all / soft-404 guard: a server that answers nearly every probe
        // with no strong discriminator defeats path-presence fingerprinting, so
        // abstain to the generic profile rather than seed a confident-but-wrong one.
        if fingerprint::is_catch_all(&probe_views) {
            profile = "LEGACY_STATIC".to_string();
        }

        // ---- learn soft-404 signatures from the random probes ----
        let mut filter = SoftNotFoundFilter::new();
        if self.cfg.use_soft404_filter {
            for r in &probe_resps {
                if random_paths.iter().any(|p| r.url.contains(p.as_str())) {
                    filter.learn_bogus(signature_of(r));
                }
            }
        }

        // ---- Phase 2: seed the predictor for the detected profile, then merge
        // any model previously learned (from the labs or earlier runs) so the
        // engine starts from accumulated knowledge, not just the hand seeds.
        let mut markov = MarkovModel::seeded(
            &profile,
            self.markov_max_order(),
            self.cfg.laplace_alpha,
            self.cfg.probability_threshold,
        );
        if !self.cfg.model_path.is_empty() {
            if let Ok(text) = std::fs::read_to_string(&self.cfg.model_path) {
                if let Ok(learned) = MarkovModel::from_json(&text) {
                    markov.merge(&learned);
                }
            }
        }

        let mut bm25 = Bm25::new();
        let mut sched = scheduler::build(&self.cfg.scheduler, self.cfg.seed);
        let mut seen = SeenPaths::new();

        // optional base wordlist for hybrid coverage (default off -> pure ML)
        let seed_words: Vec<String> = if self.cfg.seed_per_round > 0
            && !self.cfg.seed_wordlist.is_empty()
        {
            load_wordlist(&self.cfg.seed_wordlist, 20_000).unwrap_or_default()
        } else {
            Vec::new()
        };

        let mut summary = Summary {
            target: target.to_string(),
            profile: profile.clone(),
            distances,
            rounds: 0,
            requests_used: 0,
            discovered: Vec::new(),
            predicted_hits: 0,
            dropped_out_of_scope: 0,
            filtered_soft404: 0,
        };

        // seed from what the probe already revealed: record discoveries, feed
        // the ranker, and enqueue any discovered directory as an arm to expand.
        for r in &probe_resps {
            if !is_hit(r, &self.cfg) {
                continue;
            }
            if !scope.allows(&r.url) {
                summary.dropped_out_of_scope += 1;
                continue;
            }
            if self.cfg.use_soft404_filter && filter.is_soft_not_found(&signature_of(r)) {
                summary.filtered_soft404 += 1;
                continue;
            }
            if path_segments(&r.url).is_empty() {
                continue; // the root itself, handled as the first arm below
            }
            if !seen.insert(&r.url) {
                continue;
            }
            summary.discovered.push((r.url.clone(), r.status));
            bm25.add_document(&last_segment(&r.url));
            markov.learn(&r.url);
            if r.is_directory() && r.depth() < self.cfg.max_depth {
                sched.add_arm(&ensure_trailing_slash(&r.url));
            }
        }

        // the root target is always the first directory to expand
        sched.add_arm(&ensure_trailing_slash(target));

        // ---- Phase 3: adaptive rounds ----
        while summary.rounds < self.cfg.max_rounds
            && summary.requests_used < self.cfg.request_budget
        {
            let Some(arm) = sched.choose() else {
                break;
            };

            // predict → rank → wordlist
            let mut preds = markov.predict(&arm, self.cfg.top_n * 2);
            if self.cfg.ranker == "bm25" {
                preds = bm25.rerank(&preds);
            }
            let mut words = build_round_wordlist(&preds, self.cfg.top_n);
            if !seed_words.is_empty() {
                words = merge_seed(&words, &seed_words, self.cfg.seed_per_round);
            }
            if words.is_empty() {
                continue;
            }

            // budget guard on request count
            let remaining = self.cfg.request_budget.saturating_sub(summary.requests_used);
            let words: Vec<String> = words.into_iter().take(remaining).collect();
            if words.is_empty() {
                break;
            }

            let resps = self.runner.run(&FeroxArgs {
                url: arm.clone(),
                words: words.clone(),
                no_recursion: true,
                all_codes: false,
                extract_links: false,
            })?;
            summary.requests_used += words.len();
            summary.rounds += 1;

            let mut hits = 0usize;
            for r in &resps {
                if !is_hit(r, &self.cfg) {
                    continue;
                }
                if !scope.allows(&r.url) {
                    summary.dropped_out_of_scope += 1;
                    continue;
                }
                if self.cfg.use_soft404_filter && filter.is_soft_not_found(&signature_of(r)) {
                    summary.filtered_soft404 += 1;
                    continue;
                }
                if !seen.insert(&r.url) {
                    continue;
                }

                hits += 1;
                summary.predicted_hits += 1;
                summary.discovered.push((r.url.clone(), r.status));

                // online learning
                markov.learn(&r.url);
                bm25.add_document(&last_segment(&r.url));

                // enqueue newly found directories within depth
                // (the scheduler de-dups arms, so no seen-guard needed here)
                if r.is_directory() && r.depth() < self.cfg.max_depth {
                    sched.add_arm(&ensure_trailing_slash(&r.url));
                }
            }

            let reward = hits as f64 / words.len().max(1) as f64;
            sched.update(&arm, reward);
        }

        // persist what this run learned so the next scan starts smarter
        if !self.cfg.model_path.is_empty() {
            if let Ok(js) = markov.to_json() {
                if let Some(parent) = std::path::Path::new(&self.cfg.model_path).parent() {
                    let _ = std::fs::create_dir_all(parent);
                }
                let _ = std::fs::write(&self.cfg.model_path, js);
            }
        }

        Ok(summary)
    }

    /// LEARNING MODE — bootstrap the model from a set of (authorized) lab targets.
    ///
    /// For each target it fingerprints the profile, then harvests as much real
    /// path structure as it can (feroxbuster with link-extraction ON, recursion
    /// to `max_depth`, plus the optional base wordlist), and trains one Markov
    /// model on every in-scope path discovered. The trained model is written to
    /// `model_out` and merged with anything already there, so learning
    /// accumulates across targets and repeated learn runs. A normal scan then
    /// loads it via `--model` and keeps adapting online.
    pub fn learn(&mut self, targets: &[String], model_out: &str) -> anyhow::Result<LearnSummary> {
        // start from existing learning if present, else an empty model
        let mut model = match std::fs::read_to_string(model_out)
            .ok()
            .and_then(|t| MarkovModel::from_json(&t).ok())
        {
            Some(m) => m,
            None => MarkovModel::new(
                self.markov_max_order(),
                self.cfg.laplace_alpha,
                self.cfg.probability_threshold,
            ),
        };

        let seed_words: Vec<String> = if !self.cfg.seed_wordlist.is_empty() {
            load_wordlist(&self.cfg.seed_wordlist, 20_000).unwrap_or_default()
        } else {
            Vec::new()
        };

        let mut summary = LearnSummary::default();
        let total = targets.len();

        for (idx, target) in targets.iter().enumerate() {
            let scope = match Scope::new(target, &self.cfg.scope) {
                Ok(s) => s,
                Err(_) => continue,
            };

            // fingerprint (for reporting which profiles the labs covered)
            let probe_words: Vec<String> =
                PROBE_PATHS.iter().map(|s| s.to_string()).collect();
            if let Ok(probe) = self.runner.run(&FeroxArgs {
                url: target.clone(),
                words: probe_words,
                no_recursion: true,
                all_codes: true,
                extract_links: false,
            }) {
                let (profile, _) = fingerprint::build(&self.cfg.classifier, self.cfg.seed)
                    .classify(&feature_vector(&to_probe_views(&probe)));
                *summary.profiles.entry(profile).or_insert(0) += 1;
                for r in &probe {
                    if is_hit(r, &self.cfg) && scope.allows(&r.url) {
                        model.learn(&r.url);
                    }
                }
            }

            // harvest: extraction ON + base wordlist + recursion, for MAX paths
            let words = if seed_words.is_empty() {
                PROBE_PATHS.iter().map(|s| s.to_string()).collect()
            } else {
                seed_words.clone()
            };
            let resps = self
                .runner
                .run(&FeroxArgs {
                    url: target.clone(),
                    words,
                    no_recursion: false, // let the crawler/recursion find structure
                    all_codes: false,
                    extract_links: true, // harvest maximum structure while learning
                })
                .unwrap_or_default();

            let mut learned_here = 0usize;
            for r in &resps {
                if !is_hit(r, &self.cfg) || !scope.allows(&r.url) {
                    continue;
                }
                model.learn(&r.url);
                learned_here += 1;
            }
            summary.targets_learned += 1;
            summary.paths_ingested += learned_here;

            // progress + periodic checkpoint so a long run over a large host list
            // is observable, resumable, and crash-safe (the model already starts
            // from model_out if it exists, so a re-run continues where it left off).
            eprintln!(
                "[learn {}/{}] {} (+{learned_here} paths; {} contexts total)",
                idx + 1,
                total,
                target,
                model.context_count()
            );
            if (idx + 1) % 25 == 0 {
                if let Ok(js) = model.to_json() {
                    let _ = std::fs::write(model_out, js);
                }
            }
        }

        summary.contexts = model.context_count();
        summary.transitions = model.transition_count();

        if let Some(parent) = std::path::Path::new(model_out).parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(model_out, model.to_json()?)?;
        summary.model_path = model_out.to_string();
        Ok(summary)
    }
}

/// Result of a learning run.
#[derive(Debug, Default, Clone)]
pub struct LearnSummary {
    pub targets_learned: usize,
    pub paths_ingested: usize,
    pub contexts: usize,
    pub transitions: usize,
    pub profiles: std::collections::HashMap<String, usize>,
    pub model_path: String,
}

fn signature_of(r: &FeroxResponse) -> Signature {
    Signature::new(r.status, r.content_length, r.word_count, r.line_count)
}

fn is_hit(r: &FeroxResponse, cfg: &Config) -> bool {
    r.status != 0 && cfg.success_codes.contains(&r.status)
}

fn ensure_trailing_slash(url: &str) -> String {
    if url.ends_with('/') {
        url.to_string()
    } else {
        format!("{url}/")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    /// A fake runner that returns canned responses keyed by the scan URL.
    struct FakeRunner {
        probe: Vec<FeroxResponse>,
        by_arm: std::collections::HashMap<String, Vec<FeroxResponse>>,
        calls: RefCell<usize>,
    }

    fn resp(url: &str, status: u16, dir: bool) -> FeroxResponse {
        let mut headers = std::collections::HashMap::new();
        let url = if dir && !url.ends_with('/') {
            format!("{url}/")
        } else {
            url.to_string()
        };
        if dir {
            headers.insert("location".into(), format!("{url}"));
        }
        FeroxResponse {
            url,
            status,
            content_length: 10,
            word_count: 2,
            line_count: 1,
            headers,
            ..Default::default()
        }
    }

    impl FeroxRunner for FakeRunner {
        fn run(&self, args: &FeroxArgs) -> anyhow::Result<Vec<FeroxResponse>> {
            *self.calls.borrow_mut() += 1;
            // the probe call includes the known probe paths
            if args.words.iter().any(|w| w == "wp-json") {
                return Ok(self.probe.clone());
            }
            Ok(self.by_arm.get(&args.url).cloned().unwrap_or_default())
        }
    }

    #[test]
    fn full_pipeline_offline() {
        // probe → REST API signals: a JSON root plus a discovered /api directory
        let probe = vec![
            resp("https://x.test/api", 200, true),
            {
                let mut r = resp("https://x.test/", 200, false);
                r.headers.insert("content-type".into(), "application/json".into());
                r
            },
        ];
        let mut by_arm = std::collections::HashMap::new();
        // expanding /api/ (predicts "v1") yields a NEW directory /api/v1
        by_arm.insert(
            "https://x.test/api/".to_string(),
            vec![resp("https://x.test/api/v1", 200, true)],
        );
        // expanding /api/v1/ (predicts "users") yields a NEW file /api/v1/users
        by_arm.insert(
            "https://x.test/api/v1/".to_string(),
            vec![resp("https://x.test/api/v1/users", 200, false)],
        );

        let runner = FakeRunner {
            probe,
            by_arm,
            calls: RefCell::new(0),
        };
        let mut cfg = Config::default();
        cfg.max_rounds = 10;
        cfg.use_soft404_filter = false;

        let mut campaign = Campaign::new(cfg, Box::new(runner));
        let s = campaign.run("https://x.test").unwrap();

        assert_eq!(s.profile, "REST_API", "distances={:?}", s.distances);
        // /api from the probe, /api/v1 and /api/v1/users from prediction
        assert!(
            s.discovered.iter().any(|(u, _)| u.contains("/api/v1/users")),
            "should reach /api/v1/users via chained prediction: {:?}",
            s.discovered
        );
        assert!(s.predicted_hits >= 2, "summary={s:?}");
        assert_eq!(s.dropped_out_of_scope, 0);
    }

    /// A runner that returns REST probe signals, and (when extraction is on, as
    /// learn mode uses) a set of harvested paths to train on.
    struct LearnFake;
    impl FeroxRunner for LearnFake {
        fn run(&self, args: &FeroxArgs) -> anyhow::Result<Vec<FeroxResponse>> {
            if args.extract_links {
                return Ok(vec![
                    resp("https://x.test/api", 200, true),
                    resp("https://x.test/api/v1", 200, true),
                    resp("https://x.test/api/v1/users", 200, false),
                    resp("https://x.test/api/v1/orders", 200, false),
                ]);
            }
            // probe call (fingerprint): REST-ish
            Ok(vec![
                resp("https://x.test/api", 500, false),
                resp("https://x.test/rest", 500, false),
            ])
        }
    }

    #[test]
    fn learn_trains_and_saves_model() {
        let dir = std::env::temp_dir().join(format!("feroxml-learn-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let model_out = dir.join("model.json");
        let model_out = model_out.to_string_lossy().to_string();

        let cfg = Config::default();
        let mut campaign = Campaign::new(cfg, Box::new(LearnFake));
        let summary = campaign
            .learn(&["https://x.test".to_string()], &model_out)
            .unwrap();

        assert!(summary.paths_ingested >= 4, "summary={summary:?}");
        assert!(summary.contexts > 0 && summary.transitions > 0);
        assert!(std::path::Path::new(&model_out).exists(), "model not written");

        // the trained model, loaded fresh, should predict learned transitions
        let text = std::fs::read_to_string(&model_out).unwrap();
        let learned = MarkovModel::from_json(&text).unwrap();
        let preds = learned.predict("https://x.test/api/v1", 5);
        assert!(
            preds.iter().any(|(t, _)| t == "users" || t == "orders"),
            "expected learned children of /api/v1, got {preds:?}"
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn out_of_scope_is_dropped() {
        let probe = vec![resp("https://x.test/", 200, false)];
        let mut by_arm = std::collections::HashMap::new();
        by_arm.insert(
            "https://x.test/".to_string(),
            vec![resp("https://evil.test/api", 200, true)],
        );
        let runner = FakeRunner {
            probe,
            by_arm,
            calls: RefCell::new(0),
        };
        let mut cfg = Config::default();
        cfg.max_rounds = 3;
        cfg.use_soft404_filter = false;
        let mut c = Campaign::new(cfg, Box::new(runner));
        let s = c.run("https://x.test").unwrap();
        assert!(s.dropped_out_of_scope >= 1, "summary={s:?}");
        assert!(!s.discovered.iter().any(|(u, _)| u.contains("evil.test")));
    }
}
