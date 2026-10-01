# YABFF

**Yet Another Bunch of Fuzzers** — research, documentation, and two ML-driven
content-discovery tools, all centered on web fuzzing / content discovery.

> **Authorized use only.** Everything here is for security testing against systems
> you own or are explicitly permitted to assess (pentests, bug bounties in scope,
> CTFs, local labs). Do not point these tools at third parties.

## The two tools

This repo ships the same ML ideas in two forms — a standalone orchestrator that
*wraps* feroxbuster, and a fork that bakes the engine *into* feroxbuster:

| | [`feroxbuster-ml/`](feroxbuster-ml/) | [`feroxml/`](feroxml/) |
|---|---|---|
| **What it is** | A fork of [feroxbuster](https://github.com/epi052/feroxbuster) with the ML layer compiled **in-crate** | A standalone orchestrator that **drives** the stock `feroxbuster` binary in a feedback loop |
| **Use it when** | You want one binary, `--ml`, and the full feroxbuster feature set | You want the adaptive loop (bandit scheduler, learn/scan modes) around an unmodified feroxbuster |
| **Engine** | Fingerprint → seed Markov → inject predicted paths → learn online → persist | Fingerprint → seed/merge model → Thompson-sampling scheduler → BM25 rank → bounded scans → persist |
| **Flag** | `--ml` / `--ml-model <path>` | `--learn` / `--i-have-authorization` / `--model <path>` |
| **Docs** | [feroxbuster-ml/README.md](feroxbuster-ml/README.md) | [feroxml/README.md](feroxml/README.md) |

Both share one engine crate, [`ferox-ml-core/`](ferox-ml-core/) (a Cargo
workspace member), so a fix lands once: **K-Means / nearest-centroid** target
fingerprinting (four framework profiles), **variable-order Markov / PPM** path
prediction (seeded per profile, learns online), **Thompson / UCB1** scheduling,
**BM25** candidate ranking, and **SimHash** soft-404 filtering.

The ML path is measured, not just asserted:
[`docs/analysis/ml-ab-benchmark.md`](docs/analysis/ml-ab-benchmark.md) is a stock
vs `--ml` A/B (reproducible with [`scripts/ab_benchmark.py`](scripts/ab_benchmark.py)),
[`docs/analysis/jsintel-lab-results.md`](docs/analysis/jsintel-lab-results.md)
records fingerprinting against a live multi-framework lab estate (Juice Shop,
DVWA, WebGoat, WordPress, Django), and
[`docs/analysis/fingerprint-enhancements.md`](docs/analysis/fingerprint-enhancements.md)
is a labelled evaluation (confusion matrix + a tested catch-all-guard enhancement).

## The study (docs)

A survey of nine content-discovery tools (ffuf, gobuster, wfuzz, dirsearch,
feroxbuster, dirb, kiterunner, DirBuster, patator) that motivated the ML work:

- [`docs/analysis/`](docs/analysis/) — capability comparison, an empirical
  benchmark, a feature × project support matrix (35 features × 9 tools), and a
  feature co-occurrence correlation analysis with an interactive heatmap.
- [`docs/references/`](docs/references/) — per-item API references for all nine
  tools (every type, function, method, constant, macro). Start at
  [`API-REFERENCES.md`](docs/references/API-REFERENCES.md); the deep architecture
  write-up is [`feroxbuster-documentation.md`](docs/references/feroxbuster-documentation.md).
- [`docs/prompt.md`](docs/prompt.md) — the engineering spec the tools were built from.

## Layout

```
ferox-ml-core/    shared ML engine crate (fingerprint, markov, scheduler, ranking, dedup)
feroxbuster-ml/   feroxbuster fork with the native in-crate ML layer (--ml)
feroxml/          standalone adaptive ML orchestrator around feroxbuster
docs/
  analysis/       capability comparison, benchmarks (incl. the ML A/B), matrices, heatmap
  references/     per-tool API references + feroxbuster deep-dive + index
  prompt.md       the engineering spec
scripts/          analysis + benchmark scripts (ab_benchmark.py, correlation matrix)
bench/            local scoring targets + wordlists used by the benchmarks
tools/            third-party fuzzer source clones, reference only (git-ignored)
```

## Building

This repo is a Cargo workspace. From the root, `cargo build --release` builds all
three crates and `cargo test` runs every suite fully offline; `cargo test -p
<crate>` scopes to one. See each tool's README for details.

## License

MIT for the original work in this repository. `feroxbuster-ml/` is a fork of
feroxbuster and retains its upstream MIT license. Bundled third-party clones under
`tools/` retain their own licenses and are not committed.
