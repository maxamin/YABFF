# ferox-ml-core

Shared, pure-Rust ML engines for the YABFF content-discovery tools. Both
[`feroxbuster-ml`](../feroxbuster-ml/) (the in-crate `--ml` fork) and
[`feroxml`](../feroxml/) (the standalone orchestrator) depend on this crate, so
the algorithms live in exactly one place.

## Engines

| Module | What it does |
|---|---|
| [`fingerprint`](src/fingerprint.rs) + [`profiles`](src/profiles.rs) | K-Means / nearest-centroid classification of a target into one of four framework profiles (`REST_API`, `ENTERPRISE_JAVA_SPRING`, `WORDPRESS_CMS`, `LEGACY_STATIC`) from a bounded probe, with per-profile seed Markov matrices. |
| [`markov`](src/markov.rs) | Variable-order Markov / PPM path predictor; seeded per profile, learns online, JSON-serializable, mergeable across runs. |
| [`scheduler`](src/scheduler.rs) | `Thompson` / `Ucb1` / `RoundRobin` bandits with a `value(arm)` estimate used to scale per-directory budgets. |
| [`ranking`](src/ranking.rs) | BM25 re-ranking of candidates against the corpus of discovered path segments. |
| [`dedup`](src/dedup.rs) | 64-bit SimHash + coarse response signatures for soft-404 / near-duplicate detection. |
| [`tokenize`](src/tokenize.rs), [`rng`](src/rng.rs) | Path/subword tokenizers and a dependency-free deterministic RNG. |
| [`interfaces`](src/interfaces.rs) | The `Classifier` / `Predictor` / `Scheduler` traits — swap in a new algorithm without touching callers. |

Engines consume [`ProbeResp`](src/lib.rs) (a `{ url, status, headers }` view), so
they never depend on either tool's native response type. Each tool bridges its own
responses into `ProbeResp` at the call site.

## Build & test

```bash
cargo test -p ferox-ml-core   # 47 fully-offline tests (inline units + tests/engines.rs)
```

[`tests/engines.rs`](tests/engines.rs) adds broad scenario coverage over the
public API: all four profile classifications, Markov back-off / merge / JSON
round-trip, every scheduler's `value()` estimate, BM25 promotion, SimHash and
soft-404 signatures, and the tokenizers / RNG.

The only dependencies are `serde`, `serde_json`, and `anyhow`. MIT licensed.
