# ferox-ml-core

Shared, pure-Rust ML engines for the YABFF content-discovery tools. Both
[`feroxbuster-ml`](../feroxbuster-ml/) (the in-crate `--ml` fork) and
[`feroxml`](../feroxml/) (the standalone orchestrator) depend on this crate, so
the algorithms live in exactly one place.

## Engines

| Module | What it does |
|---|---|
| [`fingerprint`](src/fingerprint.rs) + [`profiles`](src/profiles.rs) | K-Means / nearest-centroid classification of a target into one of seven framework profiles (`REST_API`, `ENTERPRISE_JAVA_SPRING`, `WORDPRESS_CMS`, `LEGACY_STATIC`, `PHP_GENERIC`, `NODE_SPA`, `DJANGO`) from a bounded probe, with per-profile seed Markov matrices, a weighted-Euclidean metric (E3), and a catch-all / per-path soft-404 guard (E1/E2). |
| predictors: [`markov`](src/markov.rs) · [`freqtrie`](src/freqtrie.rs) · [`dynsdt`](src/dynsdt.rs) · [`tst`](src/tst.rs) + [`algo`](src/algo.rs) selector | Interchangeable top-k path predictors behind one `PathModel` interface, chosen with `--algo`: variable-order **Markov/PPM** (default, best by results — seeded per profile, generalizes via back-off), a plain frequency **trie**, the **Dynamic Score-Decomposed Trie** (`O(\|p\|+k log k)` heap top-k), and a **ternary search tree**. All learn online, serialize to JSON, and merge across runs. |
| [`scheduler`](src/scheduler.rs) | `Thompson` / `Ucb1` / `RoundRobin` bandits with a `value(arm)` estimate used to scale per-directory budgets. |
| [`ranking`](src/ranking.rs) | BM25 re-ranking of candidates against the corpus of discovered path segments. |
| [`dedup`](src/dedup.rs) | 64-bit SimHash + coarse response signatures for soft-404 / near-duplicate detection. |
| [`tokenize`](src/tokenize.rs), [`rng`](src/rng.rs) | Path/subword tokenizers and a dependency-free deterministic RNG. |
| [`interfaces`](src/interfaces.rs) | The `Classifier` / `Predictor` / `Scheduler` traits — swap in a new algorithm without touching callers. |
| [`orchestrator`](src/orchestrator.rs) + [`ferox`](src/ferox.rs) / [`config`](src/config.rs) / [`scope`](src/scope.rs) / [`wordlist`](src/wordlist.rs) | The **runner-agnostic adaptive loop**: a budgeted, Thompson-scheduled sequence of bounded scans driving any `FeroxRunner`: `feroxml` uses the subprocess `RealRunner` (drives an unmodified feroxbuster), while `feroxbuster --ml-loop` plugs in an **in-process** runner (direct HTTP via feroxbuster's own client, no subprocess) — same `Campaign`, same trait. **List mode** (`list_dir`) loads a wordlist directory **recursively**, ranks it by document-frequency signal, **caches** the ranked pool under `state_dir`, and re-applies it to every discovered directory to unlimited depth. |

Engines consume [`ProbeResp`](src/lib.rs) (a `{ url, status, headers }` view), so
they never depend on either tool's native response type. Each tool bridges its own
responses into `ProbeResp` at the call site.

## Build & test

```bash
cargo test -p ferox-ml-core   # 90 fully-offline tests (inline units + tests/engines.rs)
```

[`tests/engines.rs`](tests/engines.rs) adds broad scenario coverage over the
public API: all four profile classifications, Markov back-off / merge / JSON
round-trip, every scheduler's `value()` estimate, BM25 promotion, SimHash and
soft-404 signatures, and the tokenizers / RNG.

The only dependencies are `serde`, `serde_json`, and `anyhow`. MIT licensed.
