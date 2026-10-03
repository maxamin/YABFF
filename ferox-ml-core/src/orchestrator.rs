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
use crate::algo;
use crate::interfaces::{Classifier, PathModel, Predictor};
use crate::markov::MarkovModel;
use crate::profiles::PROBE_PATHS;
use crate::ranking::Bm25;
use crate::rng::Rng;
use crate::scheduler;
use crate::scope::Scope;
use crate::tokenize::{last_segment, path_segments};
use crate::wordlist::{
    build_round_wordlist, load_list_dir_cached, load_wordlist, merge_seed, ListPool, SeenPaths,
};
use std::collections::{HashMap, HashSet};

/// Persist the model learned so far during a campaign every this many rounds, so
/// an uncatchable kill (SIGKILL) or crash loses at most this many rounds of
/// learning rather than everything since the last target completed. Catchable
/// signals stop cooperatively (see [`crate::request_stop`]) and save on exit.
const SAVE_EVERY_ROUNDS: usize = 500;

/// Write `model` to `model_path` (no-op if unset), creating parent dirs. Shared
/// by the periodic flush, the cooperative-stop exit, and the end-of-run save.
/// Magic header for the binary (bincode) model container. A persisted file that
/// does not start with these bytes is treated as legacy JSON on load, so existing
/// `model.json` files keep working after the switch to the binary default.
pub(crate) const MODEL_MAGIC: &[u8; 4] = b"FXM1";

fn write_file(path: &str, bytes: &[u8]) {
    if let Some(parent) = std::path::Path::new(path).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let _ = std::fs::write(path, bytes);
}

/// Load a persisted model at `path` into `model`, auto-detecting the format: a
/// file starting with [`MODEL_MAGIC`] is the binary container (current default);
/// anything else is treated as legacy JSON (an existing `model.json`), so the
/// switch to the binary default is backward-compatible. A read or format error is
/// ignored, leaving `model` as-is (a fresh/seeded model).
fn load_model_file(model: &mut dyn PathModel, path: &str) {
    let Ok(bytes) = std::fs::read(path) else {
        return;
    };
    if let Some(payload) = bytes.strip_prefix(MODEL_MAGIC.as_slice()) {
        let _ = model.merge_bytes(payload);
    } else if let Ok(text) = std::str::from_utf8(&bytes) {
        let _ = model.merge_json(text);
    }
}

/// Persist `model`: the default on-disk format at `model_path` is the compact
/// bincode container (magic header + bincode payload); when `export_json_path`
/// is set, an additional human-readable compact-JSON copy is written there for
/// debugging. Both are no-ops when their path is empty.
fn persist_model(model: &dyn PathModel, model_path: &str, export_json_path: &str) {
    if !model_path.is_empty() {
        if let Ok(mut bytes) = model.save_bytes() {
            let mut out = Vec::with_capacity(bytes.len() + MODEL_MAGIC.len());
            out.extend_from_slice(MODEL_MAGIC);
            out.append(&mut bytes);
            write_file(model_path, &out);
        }
    }
    if !export_json_path.is_empty() {
        if let Ok(js) = model.save_json() {
            write_file(export_json_path, js.as_bytes());
        }
    }
}

/// Everything the caller learns from a campaign.
#[derive(Debug, Clone)]
pub struct Summary {
    pub target: String,
    pub profile: String,
    /// The prediction algorithm used (`markov` | `trie` | `dynsdt` | `tst`).
    pub algo: String,
    pub distances: Vec<(String, f64)>,
    pub rounds: usize,
    pub requests_used: usize,
    pub discovered: Vec<(String, u16)>,
    pub predicted_hits: usize,
    pub dropped_out_of_scope: usize,
    pub filtered_soft404: usize,
}

/// Called with `(url, status)` the moment a resource is recorded as discovered,
/// so a host can stream results (e.g. to `--output`) instead of waiting for the
/// [`Summary`] at the end of the target.
pub type DiscoveryHook = Box<dyn FnMut(&str, u16)>;

pub struct Campaign {
    cfg: Config,
    runner: Box<dyn FeroxRunner>,
    on_discover: Option<DiscoveryHook>,
}

impl Campaign {
    pub fn new(cfg: Config, runner: Box<dyn FeroxRunner>) -> Self {
        Self {
            cfg,
            runner,
            on_discover: None,
        }
    }

    /// Register a hook fired for every discovery as it happens (see [`DiscoveryHook`]).
    pub fn with_on_discover(mut self, hook: impl FnMut(&str, u16) + 'static) -> Self {
        self.on_discover = Some(Box::new(hook));
        self
    }

    /// Record a discovery in the summary and notify the streaming hook, if any.
    fn record(&mut self, summary: &mut Summary, url: &str, status: u16) {
        summary.discovered.push((url.to_string(), status));
        if let Some(hook) = self.on_discover.as_mut() {
            hook(url, status);
        }
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

        // List mode gates every new behavior: when `list_dir` is set the
        // fingerprint/seed phase is skipped and scanning is driven by the lists.
        let list_mode = !self.cfg.list_dir.is_empty();

        // ---- Phase 1: probe ----
        // The 3 random probes always run so the soft-404 signature is learned
        // before any real scanning. In list mode that's ALL we probe (no
        // PROBE_PATHS, no fingerprinting); otherwise we also probe PROBE_PATHS.
        let random_paths = self.random_probe_paths(3);
        let probe_words: Vec<String> = if list_mode {
            random_paths.clone()
        } else {
            let mut pw: Vec<String> = PROBE_PATHS.iter().map(|s| s.to_string()).collect();
            pw.extend(random_paths.iter().cloned());
            pw
        };

        let probe_resps = self.runner.run(&FeroxArgs {
            url: target.to_string(),
            words: probe_words,
            no_recursion: true,
            all_codes: true, // fingerprint must see 500/401/etc., not just successes
            extract_links: false,
        })?;

        // ---- runtime soft-404 filter (both modes), learned from the random probes ----
        let mut filter = SoftNotFoundFilter::new();
        if self.cfg.use_soft404_filter {
            for r in &probe_resps {
                if random_paths.iter().any(|p| r.url.contains(p.as_str())) {
                    filter.learn_bogus(signature_of(r));
                }
            }
        }

        // ---- list-mode catch-all guard ----
        // A path that cannot exist (a random probe) returning an in-scope "hit"
        // means the host answers everything. The soft-404 filter can still rescue
        // the scan IF those bogus hits share one signature — it learns the class
        // and drops it (e.g. a blanket redirect to a single fixed Location, or an
        // identical soft-404 shell). But when the random hits each carry a
        // DISTINCT signature — a wildcard redirect whose Location echoes the
        // request path, so every path yields a unique fingerprint — the filter can
        // never generalize, and list mode would record the entire wordlist as
        // findings. That case is unrecoverable, so abandon the target, the same way
        // a confirmed WAF ban bails.
        if list_mode && self.cfg.use_soft404_filter {
            let random_hit_sigs: Vec<Signature> = probe_resps
                .iter()
                .filter(|r| {
                    random_paths.iter().any(|p| r.url.contains(p.as_str()))
                        && is_hit(r, &self.cfg)
                })
                .map(signature_of)
                .collect();
            let distinct: HashSet<Signature> = random_hit_sigs.iter().copied().collect();
            if random_hit_sigs.len() >= 2 && distinct.len() == random_hit_sigs.len() {
                let status = random_hit_sigs[0].status;
                eprintln!(
                    "[ml-loop] catch-all on {target} — random probes returned distinct {status} \
                     hits (wildcard/blanket redirect); nothing discoverable, abandoning target"
                );
                anyhow::bail!("catch-all");
            }
        }

        // ---- fingerprint (skipped entirely in list mode) ----
        let (profile, distances) = if list_mode {
            ("LIST_DRIVEN".to_string(), Vec::new())
        } else {
            // E2: learn the soft-404 signature from the random probes, then demote
            // any discriminating probe that returns the same soft-404 body to
            // "absent", so per-path presence reflects real content.
            let (random_views, mut probe_views): (Vec<_>, Vec<_>) = to_probe_views(&probe_resps)
                .into_iter()
                .partition(|v| random_paths.iter().any(|r| v.url.contains(r.as_str())));
            let soft404 = fingerprint::learn_soft_404(&random_views);
            fingerprint::apply_soft_404(&mut probe_views, &soft404);

            let features = feature_vector(&probe_views);
            let classifier: Box<dyn Classifier> =
                fingerprint::build(&self.cfg.classifier, self.cfg.seed);
            let (mut p, d) = classifier.classify(&features);
            // E1 catch-all guard: a server that answers nearly every probe with no
            // strong discriminator defeats path-presence fingerprinting.
            if fingerprint::is_catch_all(&probe_views) {
                p = "LEGACY_STATIC".to_string();
            }
            (p, d)
        };

        // ---- Phase 2: the predictor model. In list mode it starts EMPTY and
        // learns the target's real structure purely from scan results; otherwise
        // it is seeded from the detected profile. Either way, any previously
        // learned model at `model_path` is merged in.
        // The prediction model is chosen by the `--algo` selector (see `crate::algo`):
        // `auto` => DynSDT in list mode (learns the real directory tree from scan
        // hits) or the profile-seeded Markov model otherwise; or an explicit
        // markov / trie / dynsdt / tst. Any model persisted at `model_path` of the
        // same algorithm is merged in, so repeated runs accumulate.
        let selected = algo::resolve(&self.cfg, list_mode);
        let mut model: Box<dyn PathModel> =
            algo::build(selected, &self.cfg, &profile, list_mode);
        if !self.cfg.model_path.is_empty() {
            load_model_file(&mut *model, &self.cfg.model_path);
        }

        let mut bm25 = Bm25::new();
        let mut sched = scheduler::build(&self.cfg.scheduler, self.cfg.seed);
        let mut seen = SeenPaths::new();

        // list-driven scanning: the wordlist pool is re-applied to every directory
        // (each arm has its own cursor), and `arm_tried` records the words already
        // scheduled per arm so predictions aren't rescanned when an arm is re-served.
        let mut pool = if list_mode {
            // the ranked-pool cache lives under state_dir (empty => caching disabled)
            let (loaded, from_cache) = load_list_dir_cached(
                &self.cfg.list_dir,
                self.cfg.list_max_entries,
                &self.cfg.state_dir,
            )
            .unwrap_or_default();
            let source_kind = if std::path::Path::new(&self.cfg.list_dir).is_file() {
                "file"
            } else {
                "recursive dir"
            };
            eprintln!(
                "[ml-loop] list pool loaded: {} entries ({}, {}) from {}",
                loaded.len(),
                source_kind,
                if from_cache { "from cache" } else { "ranked + cached" },
                self.cfg.list_dir
            );
            Some(ListPool::new(loaded))
        } else {
            None
        };
        let mut arm_tried: HashMap<String, HashSet<String>> = HashMap::new();
        // every arm ever enqueued (root + discovered dirs), for the list-drain
        // phase that re-serves them round-robin once the bandit retires its arms.
        let mut known_arms: Vec<String> = Vec::new();
        let mut list_rr: usize = 0;

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
            algo: selected.name().to_string(),
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
            // The random calibration probes exist only to learn the soft-404
            // signature; they are not scan results, so skip them before any
            // tally. (Otherwise each would match the signature it just helped
            // define and inflate `filtered_soft404` past `requests_used`.)
            if random_paths.iter().any(|p| r.url.contains(p.as_str())) {
                continue;
            }
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
            self.record(&mut summary, &r.url, r.status);
            bm25.add_document(&last_segment(&r.url));
            model.learn(&r.url);
            if r.is_directory() && r.depth() < self.cfg.max_depth {
                let a = ensure_trailing_slash(&r.url);
                sched.add_arm(&a);
                if !known_arms.contains(&a) {
                    known_arms.push(a);
                }
            }
        }

        // the root target is always the first directory to expand
        let root_arm = ensure_trailing_slash(target);
        sched.add_arm(&root_arm);
        known_arms.push(root_arm);

        // ---- Phase 3: adaptive rounds ----
        while summary.rounds < self.cfg.max_rounds
            && summary.requests_used < self.cfg.request_budget
        {
            // Cooperative stop (set from a host signal handler): save what we've
            // learned and leave the loop cleanly. The end-of-run save below also
            // runs, but saving here means an interrupt keeps the model even if
            // later teardown is skipped.
            if crate::stop_requested() {
                persist_model(&*model, &self.cfg.model_path, &self.cfg.model_export_json);
                break;
            }

            // Periodic flush so a SIGKILL/crash (which can't be caught) loses at
            // most SAVE_EVERY_ROUNDS rounds of learning.
            if summary.rounds > 0 && summary.rounds % SAVE_EVERY_ROUNDS == 0 {
                persist_model(&*model, &self.cfg.model_path, &self.cfg.model_export_json);
            }

            let arm = match sched.choose() {
                Some(a) => a,
                None => {
                    // list-drain: the scheduler gives each directory ONE round, so
                    // re-serve known arms round-robin to re-apply the wordlist to
                    // every directory until each has walked the whole pool (bounded
                    // by max_rounds / request_budget). Pick the next arm that still
                    // has pool entries left.
                    match pool.as_ref() {
                        Some(p) if !known_arms.is_empty() && p.any_remaining(&known_arms) => {
                            let n = known_arms.len();
                            let mut picked = None;
                            for i in 0..n {
                                let cand = &known_arms[(list_rr + i) % n];
                                if !p.arm_exhausted(cand) {
                                    picked = Some(cand.clone());
                                    list_rr = (list_rr + i + 1) % n;
                                    break;
                                }
                            }
                            match picked {
                                Some(a) => a,
                                None => break,
                            }
                        }
                        _ => break,
                    }
                }
            };

            // predict → rank → wordlist
            let mut preds = model.predict(&arm, self.cfg.top_n * 2);
            if self.cfg.ranker == "bm25" {
                preds = bm25.rerank(&preds);
            }
            let mut words = build_round_wordlist(&preds, self.cfg.top_n);
            if !seed_words.is_empty() {
                words = merge_seed(&words, &seed_words, self.cfg.seed_per_round);
            }

            // list fill: ML predictions first, then the next chunk of THIS
            // directory's own cursor over the full wordlist (re-applied per
            // directory), BM25-ranked against the observed corpus, bounded by budget.
            //
            // `arm_tried` records only the *predictions* scheduled for this arm — a
            // small set bounded by the model's vocabulary — so predictions aren't
            // re-issued when the arm is re-served. List words are NOT stored here:
            // the per-arm cursor already serves each list entry once per directory,
            // so with the whole SecLists tree re-applied per directory the memory
            // stays O(arms × predicted-tokens), not O(arms × wordlist).
            if let Some(pool) = pool.as_mut() {
                let tried = arm_tried.entry(arm.clone()).or_default();
                // drop predictions already scheduled for this arm, and record the rest
                words.retain(|w| tried.insert(w.clone()));
                let remaining = self.cfg.request_budget.saturating_sub(summary.requests_used);
                let room = remaining.saturating_sub(words.len());
                if room > 0 {
                    let want = room.min(self.cfg.list_chunk_size);
                    // skip words already scheduled as predictions for this arm; the
                    // cursor position handles list-word uniqueness per directory.
                    let chunk = pool.next_chunk(&arm, &*tried, want);
                    let ranked: Vec<String> = if self.cfg.ranker == "bm25" {
                        let pairs: Vec<(String, f64)> =
                            chunk.iter().map(|w| (w.clone(), 1.0)).collect();
                        bm25.rerank(&pairs).into_iter().map(|(w, _)| w).collect()
                    } else {
                        chunk
                    };
                    for w in ranked.into_iter().take(room) {
                        words.push(w); // list words tracked by the cursor, not `tried`
                    }
                }
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

            // periodic progress for long list-mode scans (full SecLists / unlimited
            // depth), so a large run is observable without waiting for the summary.
            if list_mode && summary.rounds % 25 == 0 {
                eprintln!(
                    "[ml-loop] rounds={} requests={} found={} arms={}",
                    summary.rounds,
                    summary.requests_used,
                    summary.discovered.len(),
                    known_arms.len()
                );
            }

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
                self.record(&mut summary, &r.url, r.status);

                // online learning
                model.learn(&r.url);
                bm25.add_document(&last_segment(&r.url));

                // enqueue newly found directories within depth
                // (the scheduler de-dups arms, so no seen-guard needed here)
                if r.is_directory() && r.depth() < self.cfg.max_depth {
                    let a = ensure_trailing_slash(&r.url);
                    sched.add_arm(&a);
                    if !known_arms.contains(&a) {
                        known_arms.push(a);
                    }
                }
            }

            let reward = hits as f64 / words.len().max(1) as f64;
            sched.update(&arm, reward);
        }

        // persist what this run learned so the next scan starts smarter
        persist_model(&*model, &self.cfg.model_path, &self.cfg.model_export_json);

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

        // list mode: harvest is driven by the user's wordlist directory (no
        // fingerprinting, no profile reporting).
        let list_mode = !self.cfg.list_dir.is_empty();
        let list_pool: Vec<String> = if list_mode {
            load_list_dir_cached(
                &self.cfg.list_dir,
                self.cfg.list_max_entries,
                &self.cfg.state_dir,
            )
            .map(|(p, _)| p)
            .unwrap_or_default()
        } else {
            Vec::new()
        };
        const HARVEST_CHUNK: usize = 5_000; // bound per-scan wordlist size

        let mut summary = LearnSummary::default();
        let total = targets.len();

        for (idx, target) in targets.iter().enumerate() {
            let scope = match Scope::new(target, &self.cfg.scope) {
                Ok(s) => s,
                Err(_) => continue,
            };

            let mut learned_here = 0usize;

            if list_mode {
                // one harvest scan per list chunk; no fingerprinting
                *summary
                    .profiles
                    .entry("LIST_DRIVEN".to_string())
                    .or_insert(0) += 1;
                for chunk in list_pool.chunks(HARVEST_CHUNK) {
                    let resps = self
                        .runner
                        .run(&FeroxArgs {
                            url: target.clone(),
                            words: chunk.to_vec(),
                            no_recursion: false,
                            all_codes: false,
                            extract_links: true,
                        })
                        .unwrap_or_default();
                    for r in &resps {
                        if is_hit(r, &self.cfg) && scope.allows(&r.url) {
                            model.learn(&r.url);
                            learned_here += 1;
                        }
                    }
                }
            } else {
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

                for r in &resps {
                    if !is_hit(r, &self.cfg) || !scope.allows(&r.url) {
                        continue;
                    }
                    model.learn(&r.url);
                    learned_here += 1;
                }
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
    let sig = Signature::new(r.status, r.content_length, r.word_count, r.line_count);
    // A redirect's body is empty/boilerplate, so the only discriminator between a
    // real directory hit and a catch-all soft-404 is where it points: fold the
    // Location in so distinct-target redirects don't collapse onto the probe's.
    if (300..400).contains(&r.status) {
        sig.with_location(r.header("location"))
    } else {
        sig
    }
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

    #[test]
    fn persisted_model_is_binary_and_round_trips() {
        let dir = std::env::temp_dir().join(format!("ferox-model-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("model.bin");
        let p = path.to_string_lossy().to_string();

        let m = MarkovModel::seeded("WORDPRESS_CMS", 3, 0.5, 0.01);
        persist_model(&m, &p, "");

        // on-disk file carries the binary magic header, not JSON
        let raw = std::fs::read(&path).unwrap();
        assert!(raw.starts_with(MODEL_MAGIC), "persisted model must be the binary container");
        assert_ne!(raw.first(), Some(&b'{'), "must not be JSON by default");

        // the sniffing loader restores it and predictions match
        let mut loaded = MarkovModel::new(3, 0.5, 0.01);
        load_model_file(&mut loaded, &p);
        assert_eq!(m.predict("/wp-content", 5), loaded.predict("/wp-content", 5));

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn loader_is_backward_compatible_with_legacy_json() {
        let dir = std::env::temp_dir().join(format!("ferox-model-json-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("model.json");
        let p = path.to_string_lossy().to_string();

        // a pre-switch model.json (plain JSON, no magic header)
        let m = MarkovModel::seeded("WORDPRESS_CMS", 3, 0.5, 0.01);
        std::fs::write(&path, m.to_json().unwrap()).unwrap();

        let mut loaded = MarkovModel::new(3, 0.5, 0.01);
        load_model_file(&mut loaded, &p);
        assert_eq!(m.predict("/wp-content", 5), loaded.predict("/wp-content", 5));

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn export_json_is_written_alongside_binary_model() {
        let dir = std::env::temp_dir().join(format!("ferox-model-exp-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let bin = dir.join("model.bin").to_string_lossy().to_string();
        let js = dir.join("export.json").to_string_lossy().to_string();

        let m = MarkovModel::seeded("WORDPRESS_CMS", 3, 0.5, 0.01);
        persist_model(&m, &bin, &js);

        assert!(std::fs::read(&bin).unwrap().starts_with(MODEL_MAGIC));
        let exported = std::fs::read_to_string(&js).unwrap();
        assert!(exported.starts_with('{'), "export must be JSON");
        // exported JSON reloads to an equivalent model
        let back = MarkovModel::from_json(&exported).unwrap();
        assert_eq!(m.predict("/wp-content", 5), back.predict("/wp-content", 5));

        let _ = std::fs::remove_dir_all(&dir);
    }

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

    #[test]
    fn discovery_hook_streams_every_hit_in_order() {
        // same chained-prediction fixture as full_pipeline_offline: /api comes
        // from the probe, /api/v1 and /api/v1/users from later rounds
        let probe = vec![
            resp("https://x.test/api", 200, true),
            {
                let mut r = resp("https://x.test/", 200, false);
                r.headers.insert("content-type".into(), "application/json".into());
                r
            },
        ];
        let mut by_arm = std::collections::HashMap::new();
        by_arm.insert(
            "https://x.test/api/".to_string(),
            vec![resp("https://x.test/api/v1", 200, true)],
        );
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

        let streamed = std::rc::Rc::new(RefCell::new(Vec::new()));
        let sink = streamed.clone();
        let s = Campaign::new(cfg, Box::new(runner))
            .with_on_discover(move |url, status| {
                sink.borrow_mut().push((url.to_string(), status))
            })
            .run("https://x.test")
            .unwrap();

        // the hook saw exactly what the summary recorded, in discovery order
        assert!(s.discovered.len() >= 3, "summary={s:?}");
        assert_eq!(*streamed.borrow(), s.discovered);
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

    // =====================================================================
    // Behavioral coverage: one test per Config knob exposed through the CLI,
    // verifying the orchestrator actually honors it (not just that it parses).
    // All offline via a programmable fake runner.
    // =====================================================================

    /// A response with an explicit JSON-stream signature (status/len/words/lines)
    /// and no redirect, so soft-404 signatures can be controlled precisely.
    fn sig_resp(url: &str, status: u16, len: u64, words: u64, lines: u64) -> FeroxResponse {
        FeroxResponse {
            url: url.to_string(),
            status,
            content_length: len,
            word_count: words,
            line_count: lines,
            headers: std::collections::HashMap::new(),
            ..Default::default()
        }
    }

    /// A fake runner with full control over the probe and per-arm responses, and
    /// a record of every call made (for budget/round/prediction assertions).
    struct ProgRunner {
        /// responses returned on the fingerprint/probe call (`all_codes == true`)
        probe: Vec<FeroxResponse>,
        /// when true, synthesize a soft-404-signature response for each probed
        /// (random) word so the soft-404 filter learns that signature
        echo_probe_soft404: bool,
        /// when true, synthesize a 301 hit with a DISTINCT signature per probed
        /// word (a wildcard/blanket redirect whose Location echoes the path), to
        /// exercise the list-mode catch-all guard
        echo_probe_distinct_hits: bool,
        /// canned responses per arm URL (round calls)
        by_arm: std::collections::HashMap<String, Vec<FeroxResponse>>,
        /// every (url, word-count, all_codes) tuple the orchestrator requested
        calls: RefCell<Vec<(String, usize, bool)>>,
    }

    impl ProgRunner {
        fn new() -> Self {
            Self {
                probe: Vec::new(),
                echo_probe_soft404: false,
                echo_probe_distinct_hits: false,
                by_arm: std::collections::HashMap::new(),
                calls: RefCell::new(Vec::new()),
            }
        }
        fn arm(mut self, url: &str, resps: Vec<FeroxResponse>) -> Self {
            self.by_arm.insert(url.to_string(), resps);
            self
        }
    }

    impl FeroxRunner for ProgRunner {
        fn run(&self, args: &FeroxArgs) -> anyhow::Result<Vec<FeroxResponse>> {
            self.calls
                .borrow_mut()
                .push((args.url.clone(), args.words.len(), args.all_codes));
            if args.all_codes {
                // fingerprint/probe call
                if self.echo_probe_soft404 {
                    let base = args.url.trim_end_matches('/').to_string();
                    return Ok(args
                        .words
                        .iter()
                        .map(|w| sig_resp(&format!("{base}/{w}"), 200, 10, 2, 1))
                        .collect());
                }
                if self.echo_probe_distinct_hits {
                    let base = args.url.trim_end_matches('/').to_string();
                    return Ok(args
                        .words
                        .iter()
                        .enumerate()
                        // each probe gets a 301 with a unique body length -> unique
                        // signature, mimicking a per-path wildcard redirect
                        .map(|(i, w)| sig_resp(&format!("{base}/{w}"), 301, 100 + i as u64 * 16, 2, 1))
                        .collect());
                }
                return Ok(self.probe.clone());
            }
            Ok(self.by_arm.get(&args.url).cloned().unwrap_or_default())
        }
    }

    /// Write `words` into a one-file wordlist directory under the temp dir and
    /// return its path. Caller is scanning with `state_dir = ""` (cache disabled).
    fn write_list_dir(label: &str, words: &[&str]) -> String {
        let dir = std::env::temp_dir()
            .join(format!("feroxml-test-{}-{label}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("w.txt"), words.join("\n")).unwrap();
        dir.to_string_lossy().to_string()
    }

    /// Base config for list-mode behavioral tests: list dir set, caching off,
    /// soft-404 filtering off (tests that need it turn it back on).
    fn list_cfg(list_dir: &str) -> Config {
        let mut cfg = Config::default();
        cfg.list_dir = list_dir.to_string();
        cfg.state_dir = String::new(); // disable the ranked-pool cache
        cfg.use_soft404_filter = false;
        cfg
    }

    #[test]
    /// use_soft404_filter = true drops responses whose signature matches a learned
    /// soft-404; a distinct (real) response survives.
    fn soft404_filter_on_drops_matching_keeps_distinct() {
        let dir = write_list_dir("s404on", &["ghost", "real"]);
        let mut runner = ProgRunner::new().arm(
            "https://x.test/",
            vec![
                sig_resp("https://x.test/ghost", 200, 10, 2, 1), // == soft-404 sig
                sig_resp("https://x.test/real", 200, 500, 50, 10), // distinct
            ],
        );
        runner.echo_probe_soft404 = true;

        let mut cfg = list_cfg(&dir);
        cfg.use_soft404_filter = true;
        cfg.max_rounds = 5;

        let s = Campaign::new(cfg, Box::new(runner)).run("https://x.test").unwrap();
        assert!(
            s.discovered.iter().any(|(u, _)| u.ends_with("/real")),
            "real page must survive the filter: {:?}",
            s.discovered
        );
        assert!(
            !s.discovered.iter().any(|(u, _)| u.ends_with("/ghost")),
            "ghost (soft-404 signature) must be filtered: {:?}",
            s.discovered
        );
        assert!(s.filtered_soft404 >= 1, "summary={s:?}");
        // invariant: never more filtered than requested (Bug 2 regression)
        assert!(
            s.filtered_soft404 <= s.requests_used,
            "filtered must not exceed requests: {s:?}"
        );
        let _ = std::fs::remove_dir_all(std::path::Path::new(&dir));
    }

    #[test]
    /// A wildcard/blanket-redirect host answers every path (including the random
    /// probes) with an in-scope hit whose signature is unique per path, so the
    /// soft-404 filter can never generalize. List mode must detect this and bail
    /// rather than record the whole wordlist as findings.
    fn list_mode_bails_on_wildcard_catch_all() {
        let dir = write_list_dir("catchall", &["admin", "login", "api"]);
        let mut runner = ProgRunner::new();
        runner.echo_probe_distinct_hits = true; // random probes -> distinct 301 hits

        let mut cfg = list_cfg(&dir);
        cfg.use_soft404_filter = true;
        cfg.max_rounds = 5;

        let err = Campaign::new(cfg, Box::new(runner))
            .run("https://wildcard.test")
            .expect_err("a wildcard catch-all must bail");
        assert!(
            err.to_string().contains("catch-all"),
            "unexpected error: {err}"
        );
        let _ = std::fs::remove_dir_all(std::path::Path::new(&dir));
    }

    #[test]
    /// Bug 2: the random calibration probes define the soft-404 signature and so
    /// match it, but they are not scan results and must not be tallied as
    /// `filtered_soft404` (which previously pushed it past `requests_used`).
    fn soft404_count_excludes_calibration_probes() {
        let dir = write_list_dir("s404probe", &["alpha", "beta"]);
        // round responses are all DISTINCT from the soft-404 signature, so the
        // only soft-404-matching responses in the whole run are the random probes.
        let mut runner = ProgRunner::new().arm(
            "https://x.test/",
            vec![
                sig_resp("https://x.test/alpha", 200, 500, 50, 10),
                sig_resp("https://x.test/beta", 200, 600, 60, 12),
            ],
        );
        runner.echo_probe_soft404 = true; // random probes carry the soft-404 sig

        let mut cfg = list_cfg(&dir);
        cfg.use_soft404_filter = true;
        cfg.max_rounds = 5;

        let s = Campaign::new(cfg, Box::new(runner)).run("https://x.test").unwrap();
        assert_eq!(
            s.filtered_soft404, 0,
            "calibration probes must not be counted as filtered results: {s:?}"
        );
        assert!(
            s.filtered_soft404 <= s.requests_used,
            "filtered must not exceed requests: {s:?}"
        );
        let _ = std::fs::remove_dir_all(std::path::Path::new(&dir));
    }

    #[test]
    /// use_soft404_filter = false keeps even soft-404-looking responses.
    fn soft404_filter_off_keeps_everything() {
        let dir = write_list_dir("s404off", &["ghost", "real"]);
        let runner = ProgRunner::new().arm(
            "https://x.test/",
            vec![
                sig_resp("https://x.test/ghost", 200, 10, 2, 1),
                sig_resp("https://x.test/real", 200, 500, 50, 10),
            ],
        ); // echo_probe_soft404 = false -> empty probe, nothing learned

        let mut cfg = list_cfg(&dir); // soft404 already off
        cfg.max_rounds = 5;

        let s = Campaign::new(cfg, Box::new(runner)).run("https://x.test").unwrap();
        assert!(s.discovered.iter().any(|(u, _)| u.ends_with("/ghost")));
        assert!(s.discovered.iter().any(|(u, _)| u.ends_with("/real")));
        assert_eq!(s.filtered_soft404, 0, "summary={s:?}");
        let _ = std::fs::remove_dir_all(std::path::Path::new(&dir));
    }

    #[test]
    /// max_rounds caps the number of feedback-loop rounds exactly.
    fn max_rounds_caps_the_loop() {
        let words: Vec<String> = (0..50).map(|i| format!("w{i}")).collect();
        let wref: Vec<&str> = words.iter().map(|s| s.as_str()).collect();
        let dir = write_list_dir("rounds", &wref);
        let mut cfg = list_cfg(&dir); // runner finds nothing
        cfg.list_chunk_size = 5;
        cfg.max_rounds = 3;
        cfg.request_budget = usize::MAX;

        let s = Campaign::new(cfg, Box::new(ProgRunner::new()))
            .run("https://x.test")
            .unwrap();
        assert_eq!(s.rounds, 3, "summary={s:?}");
        assert_eq!(s.requests_used, 15, "3 rounds * chunk 5; summary={s:?}");
        let _ = std::fs::remove_dir_all(std::path::Path::new(&dir));
    }

    #[test]
    /// request_budget caps total requests, even mid-round.
    fn request_budget_caps_requests() {
        let words: Vec<String> = (0..50).map(|i| format!("w{i}")).collect();
        let wref: Vec<&str> = words.iter().map(|s| s.as_str()).collect();
        let dir = write_list_dir("budget", &wref);
        let mut cfg = list_cfg(&dir);
        cfg.list_chunk_size = 5;
        cfg.max_rounds = usize::MAX;
        cfg.request_budget = 12;

        let s = Campaign::new(cfg, Box::new(ProgRunner::new()))
            .run("https://x.test")
            .unwrap();
        assert_eq!(s.requests_used, 12, "must stop exactly at budget; summary={s:?}");
        let _ = std::fs::remove_dir_all(std::path::Path::new(&dir));
    }

    #[test]
    /// list_max_entries caps how much of the pool is ever served (one arm here, so
    /// total requests == the cap).
    fn list_max_entries_caps_pool() {
        let words: Vec<String> = (0..20).map(|i| format!("w{i}")).collect();
        let wref: Vec<&str> = words.iter().map(|s| s.as_str()).collect();
        let dir = write_list_dir("listmax", &wref);
        let mut cfg = list_cfg(&dir);
        cfg.list_max_entries = 5;
        cfg.list_chunk_size = 200;
        cfg.max_rounds = usize::MAX;
        cfg.request_budget = usize::MAX;

        let s = Campaign::new(cfg, Box::new(ProgRunner::new()))
            .run("https://x.test")
            .unwrap();
        assert_eq!(s.requests_used, 5, "pool capped to 5 entries; summary={s:?}");
        let _ = std::fs::remove_dir_all(std::path::Path::new(&dir));
    }

    #[test]
    /// max_depth bounds how deep recursion enqueues new directory arms.
    fn max_depth_bounds_recursion() {
        let dir = write_list_dir("depth", &["x", "y", "z"]);
        let make = || {
            ProgRunner::new()
                .arm("https://x.test/", vec![resp("https://x.test/a", 200, true)])
                .arm("https://x.test/a/", vec![resp("https://x.test/a/b", 200, true)])
                .arm(
                    "https://x.test/a/b/",
                    vec![resp("https://x.test/a/b/c", 200, false)],
                )
        };

        // depth 1: /a found but not expanded, so /a/b never reached
        let mut shallow = list_cfg(&dir);
        shallow.max_depth = 1;
        shallow.max_rounds = 30;
        let s = Campaign::new(shallow, Box::new(make())).run("https://x.test").unwrap();
        // resp(.., dir=true) yields a trailing-slash URL, so /a is recorded as /a/
        assert!(s.discovered.iter().any(|(u, _)| u.ends_with("/a/")));
        assert!(
            !s.discovered.iter().any(|(u, _)| u.contains("/a/b")),
            "depth 1 must not recurse into /a/: {:?}",
            s.discovered
        );

        // depth 5: the whole chain is reachable
        let mut deep = list_cfg(&dir);
        deep.max_depth = 5;
        deep.max_rounds = 30;
        let s = Campaign::new(deep, Box::new(make())).run("https://x.test").unwrap();
        assert!(s.discovered.iter().any(|(u, _)| u.contains("/a/b/")));
        assert!(s.discovered.iter().any(|(u, _)| u.ends_with("/a/b/c")));
        let _ = std::fs::remove_dir_all(std::path::Path::new(&dir));
    }

    #[test]
    /// every documented --ml-scheduler value drives a working campaign.
    fn every_scheduler_discovers() {
        let dir = write_list_dir("sched", &["p"]);
        for sched in ["thompson", "ucb1", "round_robin"] {
            let runner = ProgRunner::new()
                .arm("https://x.test/", vec![resp("https://x.test/found", 200, false)]);
            let mut cfg = list_cfg(&dir);
            cfg.scheduler = sched.to_string();
            cfg.max_rounds = 5;
            let s = Campaign::new(cfg, Box::new(runner)).run("https://x.test").unwrap();
            assert!(
                s.discovered.iter().any(|(u, _)| u.ends_with("/found")),
                "scheduler {sched} failed to discover: {:?}",
                s.discovered
            );
        }
        let _ = std::fs::remove_dir_all(std::path::Path::new(&dir));
    }

    #[test]
    /// every documented --ml-algo value builds and drives a working campaign.
    fn every_algo_discovers() {
        let dir = write_list_dir("algo", &["p"]);
        for algo in ["auto", "markov", "trie", "dynsdt", "tst"] {
            let runner = ProgRunner::new()
                .arm("https://x.test/", vec![resp("https://x.test/found", 200, false)]);
            let mut cfg = list_cfg(&dir);
            cfg.algo = algo.to_string();
            cfg.max_rounds = 5;
            let s = Campaign::new(cfg, Box::new(runner)).run("https://x.test").unwrap();
            assert!(
                s.discovered.iter().any(|(u, _)| u.ends_with("/found")),
                "algo {algo} failed to discover: {:?}",
                s.discovered
            );
        }
        let _ = std::fs::remove_dir_all(std::path::Path::new(&dir));
    }

    #[test]
    /// BM25 re-ranking on (ranker=bm25) and off (ranker=none) both discover; this
    /// is the behavioral side of --no-ml-rank.
    fn ranker_on_and_off_both_discover() {
        let dir = write_list_dir("rank", &["p", "q"]);
        for ranker in ["bm25", "none"] {
            let runner = ProgRunner::new()
                .arm("https://x.test/", vec![resp("https://x.test/found", 200, false)]);
            let mut cfg = list_cfg(&dir);
            cfg.ranker = ranker.to_string();
            cfg.max_rounds = 5;
            let s = Campaign::new(cfg, Box::new(runner)).run("https://x.test").unwrap();
            assert!(
                s.discovered.iter().any(|(u, _)| u.ends_with("/found")),
                "ranker {ranker} failed to discover: {:?}",
                s.discovered
            );
        }
        let _ = std::fs::remove_dir_all(std::path::Path::new(&dir));
    }

    #[test]
    /// top_n (--ml-predictions) bounds the number of predicted words per round in
    /// prediction mode (seed_per_round = 0).
    fn top_n_bounds_predictions_per_round() {
        // REST fixture so the Markov profile seed yields real predictions
        let mut inner = ProgRunner::new().arm(
            "https://x.test/api/",
            vec![resp("https://x.test/api/v1", 200, true)],
        );
        inner.probe = vec![
            resp("https://x.test/api", 200, true),
            {
                let mut r = resp("https://x.test/", 200, false);
                r.headers.insert("content-type".into(), "application/json".into());
                r
            },
        ];

        // thin wrapper that records (word-count, is_probe) for every call
        struct Rec {
            inner: ProgRunner,
            log: std::rc::Rc<RefCell<Vec<(usize, bool)>>>,
        }
        impl FeroxRunner for Rec {
            fn run(&self, args: &FeroxArgs) -> anyhow::Result<Vec<FeroxResponse>> {
                self.log.borrow_mut().push((args.words.len(), args.all_codes));
                self.inner.run(args)
            }
        }

        let recorder = std::rc::Rc::new(RefCell::new(Vec::new()));
        let rec = Rec {
            inner,
            log: recorder.clone(),
        };

        let mut cfg = Config::default();
        cfg.use_soft404_filter = false;
        cfg.max_rounds = 6;
        cfg.top_n = 4;
        cfg.seed_per_round = 0;

        let s = Campaign::new(cfg, Box::new(rec)).run("https://x.test").unwrap();
        assert!(s.predicted_hits >= 1, "needs real predictions; summary={s:?}");
        for (n, is_probe) in recorder.borrow().iter() {
            if !*is_probe {
                assert!(
                    *n <= 4,
                    "a prediction round issued {n} words, exceeding top_n=4"
                );
            }
        }
    }

    #[test]
    /// success_codes gate what counts as a hit: a status outside the set is not
    /// recorded as discovered.
    fn success_codes_gate_hits() {
        let dir = write_list_dir("codes", &["p"]);
        let runner = ProgRunner::new().arm(
            "https://x.test/",
            vec![
                sig_resp("https://x.test/ok", 200, 100, 10, 2),
                sig_resp("https://x.test/forbidden", 403, 50, 5, 1),
            ],
        );
        let mut cfg = list_cfg(&dir);
        cfg.success_codes = vec![200]; // 403 no longer a hit
        cfg.max_rounds = 5;
        let s = Campaign::new(cfg, Box::new(runner)).run("https://x.test").unwrap();
        assert!(s.discovered.iter().any(|(u, _)| u.ends_with("/ok")));
        assert!(
            !s.discovered.iter().any(|(u, _)| u.ends_with("/forbidden")),
            "403 must be gated out when not in success_codes: {:?}",
            s.discovered
        );
        let _ = std::fs::remove_dir_all(std::path::Path::new(&dir));
    }

    #[test]
    /// model_path (--ml-model) is written after a scan-mode run so learning persists.
    fn scan_persists_model_to_path() {
        let dir = write_list_dir("persist", &["p"]);
        let model = std::env::temp_dir()
            .join(format!("feroxml-model-{}.json", std::process::id()));
        let model_path = model.to_string_lossy().to_string();
        let _ = std::fs::remove_file(&model);

        let runner = ProgRunner::new()
            .arm("https://x.test/", vec![resp("https://x.test/found", 200, true)]);
        let mut cfg = list_cfg(&dir);
        cfg.model_path = model_path.clone();
        cfg.max_rounds = 5;
        let _ = Campaign::new(cfg, Box::new(runner)).run("https://x.test").unwrap();
        assert!(
            std::path::Path::new(&model_path).exists(),
            "model should be persisted to --ml-model path"
        );
        let _ = std::fs::remove_file(&model);
        let _ = std::fs::remove_dir_all(std::path::Path::new(&dir));
    }
}
