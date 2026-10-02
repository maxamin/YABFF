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
