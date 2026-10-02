# Test run: list-driven engine on the jsintel labs with SecLists

Running notes from exercising the full engine against the live jsintel lab estate
(loopback, authorized) using real SecLists wordlists.

- **Binary:** `target/debug/feroxbuster` (workspace debug build).
- **Lists (`--ml-list-dir /tmp/sl`):** SecLists `Discovery/Web-Content/common.txt`
  (4,749) + `raft-small-directories.txt` (20,115) → merged, deduped pool.
- **Labs:** Juice Shop `127.0.0.1:3000`, DVWA `127.0.0.3:8081`,
  WebGoat `127.0.0.2:8082`, WordPress `127.0.0.8:8083`, Django `127.0.0.11:8092`.
- **Loop bounds (defaults):** max_rounds 25, request_budget 20000, max_depth 4,
  list_chunk 200. Each run `--rate-limit 200 -t 30`.

## Results

### A. List-driven mode (`--ml-list-dir /tmp/sl`)

| lab | profile | rounds | requests | found | predicted | soft404 | oos | wall |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| juice-shop | LIST_DRIVEN | 1 | 200 | 1 | 1 | 200 | 0 | 15s |
| dvwa | LIST_DRIVEN | 1 | 200 | 5 | 5 | 196 | 0 | 2s |
| webgoat | LIST_DRIVEN | 1 | 200 | 0 | 0 | 0 | 0 | 2s |
| wordpress | LIST_DRIVEN | 2 | 401 | 1 | 1 | 403 | 0 | 40s |
| django | LIST_DRIVEN | 1 | 200 | 0 | 0 | 0 | 0 | 4s |

### B. Legacy fingerprint loop (`--ml-loop`, no list dir) — comparison

| lab | profile | rounds | requests | found | predicted | wall |
|---|---|---:|---:|---:|---:|---:|
| juice-shop | NODE_SPA | 3 | 51 | 3 | 1 | 3s |
| wordpress | WORDPRESS_CMS | 1 | 16 | 4 | 2 | 5s |
| django | LEGACY_STATIC | 1 | 18 | 0 | 0 | 2s |

### C. Coverage knob: `--ml-list-chunk 5000`

| lab | rounds | requests | found | soft404 |
|---|---:|---:|---:|---:|
| dvwa | 1 | 5000 | 9 | 4987 |
| webgoat | 1 | 5000 | 0 | 0 |

## Observations

- **List-driven mode works end-to-end on every lab** — all report `LIST_DRIVEN`,
  no fingerprinting, scanning straight from the SecLists pool.
- **Soft-404 filtering is essential and scales.** The catch-all labs (Juice Shop,
  DVWA, WordPress) answer 200 to almost everything; the filter suppresses it
  (dvwa chunk-5000: 4987/5000 filtered, 9 real hits kept). This is exactly why the
  3 random probes + `SoftNotFoundFilter` are retained in list mode.
- **Per-round coverage = `list_chunk_size`.** With the default 200 a flat/catch-all
  target scans ~200 words then stops, because the bandit retires the root arm after
  one selection and the (soft-404-filtered) catch-all yields no new directory arms.
  Raising `--ml-list-chunk` is the lever: at 5000 a single round covers 5000 words
  (budget-bounded at `request_budget`, default 20000). Multi-round consumption also
  happens naturally when real directories are discovered (WordPress: 2 rounds).
- **Clean (non-catch-all) targets behave ideally** — WebGoat/Django return honest
  404s (0 soft-404s, 0 false hits); the list simply doesn't match their routes.
- **Legacy fingerprint loop still works** (regression check): Juice Shop → NODE_SPA,
  WordPress → WORDPRESS_CMS, Django → LEGACY_STATIC. On these tiny mocks it finds a
  few more via seeded paths, but that's the fragile profile-guess path that
  list-driven mode deliberately replaces for real, unknown targets.

## Recommendation

For real SecLists-scale scans on flat targets, run with a larger chunk, e.g.
`--ml-list-chunk 5000` (or up to `request_budget`). A follow-up worth doing: in
list mode, let the orchestrator re-serve an arm while its list cursor still has
entries, so the default chunk drains the whole pool across rounds without the
operator tuning the knob (orchestrator-only change; the scheduler stays untouched).

## Offline suite

`cargo test --workspace`: **657 passed, 0 failed, 28 suites** (incl. the 3 new
list-driver tests). `cargo build` workspace: clean.

## Follow-up implemented: arm re-serve (list-drain)

The orchestrator now, in list mode only, re-serves known arms round-robin once the
bandit retires its arms, as long as the list cursor still has entries — so the pool
drains across rounds at the **default** chunk without tuning the knob (bounded by
max_rounds / request_budget; scheduler untouched).

Before vs after on DVWA with the default `--ml-list-chunk 200`:

| | rounds | requests | found | soft404 |
|---|---:|---:|---:|---:|
| before (stopped after 1 chunk) | 1 | 200 | 5 | 196 |
| after (drains across rounds) | 25 | 5110 | 9 | 4987 |

Verified by a new unit test (`list_driven_drains_whole_pool_across_rounds`): a flat
target with no discovered directories now consumes the entire pool across rounds.

## Re-run with the list-drain fix (default `--ml-list-chunk 200`)

| lab | profile | rounds | requests | found | predicted | soft404 | oos | wall |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| juice-shop | LIST_DRIVEN | 25 | 5082 | 7 | 7 | 5030 | 0 | 193s |
| dvwa | LIST_DRIVEN | 25 | 5110 | 9 | 9 | 4987 | 0 | 9s |
| webgoat | LIST_DRIVEN | 25 | 5000 | 0 | 0 | 0 | 0 | 3s |
| wordpress | LIST_DRIVEN | 25 | 5024 | 2 | 2 | 5024 | 0 | 327s |
| django | LIST_DRIVEN | 25 | 5008 | 1 | 1 | 0 | 0 | 85s |

**Before → after (default chunk 200), rounds / requests / found:**
juice-shop 1/200/1 → 25/5082/7 · dvwa 1/200/5 → 25/5110/9 · webgoat 1/200/0 →
25/5000/0 · wordpress 2/401/1 → 25/5024/2 · django 1/200/0 → 25/5008/1. Every lab
now drains the list across the full round budget without tuning the knob; soft-404
filtering scales with it (juice-shop 5030, wordpress 5024 filtered). WebGoat stays
0 (its app is under `/WebGoat/`, which the generic lists don't hit).

## DynSDT model (Dynamic Score-Decomposed Trie) — list-mode predictor

The list-mode predictor is now a **Dynamic Score-Decomposed Trie** (`ferox-ml-core/src/trie.rs`),
following Validark's DynSDT (https://validark.dev/DynSDT/) adapted to URL path segments.
Each node is a path prefix with its own observation `score` and a cached `subtree_max`
(the score decomposition); children are kept **sorted by `subtree_max` descending**
(the horizontal heap property) so top-k autocomplete is a best-first first-child /
next-sibling walk — O(|p| + k log k), no full child scan. `observe()` bumps a
terminal's score and re-sorts the affected nodes up the path online, so the model is
fully dynamic. It serializes to JSON and merges, so repeated runs accumulate.

Non-list mode is unchanged (profile-seeded Markov). The orchestrator picks the
backend via an internal `Model` enum gated on `list_mode`.

**Live verification (Juice Shop, 127.0.0.1:3000, 12-word list dir):**
- Run 1: profile LIST_DRIVEN, 5 resources found (assets, robots.txt, main.js, video, ftp),
  7 soft-404 filtered; model persisted as DynSDT JSON (nodes with seg/score/subtree_max).
- Run 2 (loads + merges the persisted model): every discovered path's score climbed
  1.0 → 2.0 — "more runs = more training", confirmed end-to-end.

Tests: 5 unit tests in `trie.rs` (score-ordered top-k, dynamic reorder, deep-completion
surfacing, merge + JSON round-trip, Predictor trait) plus
`list_mode_persists_a_dynsdt_model_that_accumulates_across_runs` in `tests/engines.rs`.
Full suite green: 41 lib + 34 integration in ferox-ml-core.
