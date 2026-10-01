# feroxml

**Adaptive, ML-driven content-discovery orchestrator built on top of [feroxbuster](https://github.com/epi052/feroxbuster).**

feroxml wraps the real `feroxbuster` binary and adds three cooperating engines so
that *which* paths get tried, and in *what order*, is learned and adapted instead
of fixed:

1. **K-Means / nearest-centroid fingerprinting** — a bounded probe classifies the
   target into one of seven framework profiles (`REST_API`,
   `ENTERPRISE_JAVA_SPRING`, `WORDPRESS_CMS`, `LEGACY_STATIC`, `PHP_GENERIC`,
   `NODE_SPA`, `DJANGO`), with a weighted metric and catch-all / soft-404 guards.
2. **Variable-order Markov / PPM path prediction** — seeded per profile, it
   predicts the most probable next path tokens for any discovered directory, and
   learns online as real paths are found.
3. **Thompson-sampling scheduler** — a multi-armed bandit that spends the request
   budget on the directories most likely to yield results, with **BM25**
   re-ranking of candidates and **SimHash / response-signature** soft-404
   filtering to keep noise out of the reward signal.

> **Authorized use only.** feroxml refuses to run without `--i-have-authorization`
> and scope-checks every generated URL: an out-of-scope host is dropped and
> logged. Only point it at systems you own or are explicitly permitted to test.

## How it works

feroxbuster cannot accept new wordlist entries into a *running* scan, so feroxml
drives a sequence of **bounded** scans in a feedback loop:

```
probe ─▶ fingerprint ─▶ seed + merge learned model
                               │
        ┌──────────────────────▼───────────────────────┐
        │  scheduler picks a directory (Thompson/UCB1)  │
        │  predict next tokens (Markov/PPM)             │
        │  BM25-rank ─▶ bounded feroxbuster scan        │
        │  filter soft-404 ─▶ scope-check ─▶ report     │
        │  learn online ─▶ enqueue new directories      │
        └──────────────────────┬───────────────────────┘
                               ▼  (until budget / rounds / depth)
                       persist learned model
```

Discoveries are attributable to feroxml's own engine: feroxbuster link-extraction
is **off by default** during scans (it is used only while learning, where
harvesting maximum structure is the goal). Turn it on with `--extract-links`.

## Learning and adapting (bootstrapping from labs)

feroxml adapts *within* a run (online Markov updates + the bandit) and *across*
runs (a persisted Markov model).

**Learn from authorized targets/labs, then reuse the knowledge:**

```bash
# 1) LEARN: harvest path structure from a set of authorized labs and train a model
feroxml --learn --targets labs.txt --i-have-authorization \
        --model .feroxml/model.json --seed-wordlist /usr/share/seclists/Discovery/Web-Content/common.txt

# 2) SCAN: load the learned model (merged onto the profile seed), keep adapting,
#          and save the updated model back
feroxml --url https://target.example --i-have-authorization --model .feroxml/model.json
```

Learning **accumulates**: each learn run and each scan merges its observations
into the model file, so the engine gets smarter over time. Example learn run over
four local labs:

```
=== feroxml learn summary ===
targets learned : 4
paths ingested  : 10
model contexts  : 6
model transitions: 15
profiles seen   : LEGACY_STATIC=2 WORDPRESS_CMS=1 REST_API=1
```

## Usage

```
feroxml --url <URL> --i-have-authorization [options]
feroxml --learn --targets <file> --i-have-authorization --model <path> [options]
```

| Flag | Meaning |
|---|---|
| `-u, --url <URL>` | Target (required unless `--targets`/`--learn`). |
| `--i-have-authorization` | Required acknowledgement of authorized testing. |
| `--learn` | Learning mode: harvest + train a model instead of scanning. |
| `--targets <file>` | File of target URLs, one per line. |
| `--model <path>` | Learned model to load (scan) or write (learn). |
| `--scope <host>` | Extra in-scope hosts/domains (repeatable). |
| `--seed-wordlist <path>` | Base wordlist for hybrid coverage / learn harvest. |
| `--seed-per-round <n>` | Base-list entries to add per expansion (0 = pure ML). |
| `--top-n <n>` | Predicted tokens per expansion (default 12). |
| `--max-rounds <n>` | Feedback rounds (default 25). |
| `--max-depth <n>` | Max recursion depth (default 4). |
| `--request-budget <n>` | Global request cap (default 20000). |
| `--rate-limit <n>` | Requests/sec passed to feroxbuster (0 = unlimited). |
| `--threads <n>` | feroxbuster threads (default 20). |
| `--scheduler <name>` | `thompson` (default), `ucb1`, `round_robin`. |
| `--classifier <name>` | `nearest_centroid` (default), `kmeans`. |
| `-k, --insecure` | Disable TLS verification (adds `-k` to feroxbuster). |
| `--extract-links` | Let feroxbuster extract links (mixes crawler results in). |
| `--config <path>` | TOML config file (see `ferox-config.toml.example`). |
| `--seed <n>` | Deterministic RNG seed. |

## Build

```bash
cargo build --release      # needs the `feroxbuster` binary on PATH at runtime
cargo test                 # 34 tests, fully offline (runner is injected)
./target/release/feroxml --help
```

## Design notes

- **Dependencies are deliberately lean** (`serde`, `serde_json`, `toml`, `clap`,
  `anyhow`, `url`). K-Means, PPM, Thompson sampling, BM25, SimHash and the RNG
  (SplitMix64 → xoshiro256\*\*, with Gamma/Beta samplers) are implemented in-crate,
  so runs are deterministic from a single seed.
- **The runner is injected** (`FeroxRunner` trait), so the entire pipeline —
  probe, fingerprint, prediction, scheduling, feedback loop, learning — is unit
  tested offline against fixtures, with no live target or network.
- **Engines are swappable** behind the `Classifier` / `Predictor` / `Scheduler`
  traits; the roadmap (LinUCB, gradient-boosted fingerprinting, MCTS, a learned
  path generator) plugs in without touching the orchestrator.

## Roadmap

- Per-profile learned models (a WordPress-learned transition shouldn't weight a
  REST scan); currently one merged model.
- Contextual bandit (LinUCB) using the fingerprint vector as context.
- Optional learned path generator behind the `Predictor` trait.

## License

MIT.
