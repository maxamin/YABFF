# YABFF

**Yet Another Bunch of Fuzzers** — research, documentation, and a new ML-driven
content-discovery tool, all centered on web fuzzing / content discovery.

> **Authorized use only.** Everything here is for security testing against systems
> you own or are explicitly permitted to assess (pentests, bug bounties in scope,
> CTFs, local labs). Do not point these tools at third parties.

## What's here

### `feroxml/` — the tool
An **adaptive, ML-driven fuzzing orchestrator** (Rust) that wraps
[feroxbuster](https://github.com/epi052/feroxbuster) and adds:
- **K-Means / nearest-centroid** target fingerprinting (4 framework profiles),
- **variable-order Markov / PPM** path prediction (seeded per profile, learns online),
- a **Thompson-sampling** scheduler with **BM25** candidate ranking and
  **SimHash** soft-404 filtering,
- **persistent learning** (`--learn`): harvest path structure from authorized
  labs, train a model, and reuse/adapt it across runs.

See [`feroxml/README.md`](feroxml/README.md). Build with `cargo build --release`;
`cargo test` runs 34 fully-offline tests.

### Fuzzer study (docs)
- [web-fuzzers-comparison.md](web-fuzzers-comparison.md) — feature/capability
  comparison of nine tools (ffuf, gobuster, wfuzz, dirsearch, feroxbuster, dirb,
  kiterunner, DirBuster, patator).
- [benchmark.md](benchmark.md) — empirical speed/accuracy benchmark against a
  local scoring target.
- [feature-project-matrix.md](feature-project-matrix.md) — feature × project
  support matrix (35 features × 9 tools).
- [feature-correlation-matrix.md](feature-correlation-matrix.md) +
  [feature-correlation-matrix.csv](feature-correlation-matrix.csv) +
  [feature-correlation-heatmap.html](feature-correlation-heatmap.html) — feature
  co-occurrence correlation analysis and an interactive heatmap.

### API references (from source reads)
Per-item API references for all nine tools — every type, function, method,
constant and macro — see [API-REFERENCES.md](API-REFERENCES.md) for the index, and
[feroxbuster-documentation.md](feroxbuster-documentation.md) for the deep
architecture write-up.

### Supporting
- [prompt.md](prompt.md) — the engineering spec that feroxml was built from.
- `tools-scripts/` — analysis scripts (correlation matrix generation).
- `tools/` — cloned third-party fuzzer source, kept for reference only
  (git-ignored; each has its own upstream repository).

## Layout

```
feroxml/                 the Rust adaptive fuzzer (source, tests, README, config)
*-api-reference.md       per-tool API references
feature-*.{md,csv,html}  feature matrix + correlation analysis + heatmap
benchmark.md             empirical benchmark
web-fuzzers-comparison.md  capability comparison
tools-scripts/           analysis scripts
tools/                   third-party clones (ignored)
```

## License

MIT (for the original work in this repository; bundled third-party clones under
`tools/` retain their own licenses and are not committed).
