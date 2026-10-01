# YABFF — an adaptive, ML-driven web content-discovery fuzzer

**Yet Another Bunch of Fuzzers** is a self-learning forced-browsing / content-
discovery engine for web targets. Instead of grinding a fixed wordlist against
every directory, it **fingerprints** the target's framework, **predicts** the
paths most likely to exist for *that* framework and *that* directory, spends its
request budget where results are actually appearing, and **learns online** —
getting better within a scan and across scans.

It ships in two forms that share one engine: a **fork of
[feroxbuster](https://github.com/epi052/feroxbuster)** with the ML layer compiled
in (`--ml`), and a **standalone orchestrator** (`feroxml`) that drives the stock
feroxbuster binary in a feedback loop.

> **Authorized use only.** This is an offensive security tool. Point it only at
> systems you own or are explicitly permitted to assess (pentests, in-scope bug
> bounties, CTFs, local labs). `feroxml` refuses to run without
> `--i-have-authorization` and scope-checks every generated URL.

---

## Contents

- [What makes it different](#what-makes-it-different)
- [The two forms](#the-two-forms)
- [The algorithms](#the-algorithms)
- [How a scan flows](#how-a-scan-flows)
- [Install](#install)
- [Usage](#usage)
- [Test flow](#test-flow)
- [Evaluation & benchmarks](#evaluation--benchmarks)
- [Repository layout](#repository-layout)
- [The fuzzer study](#the-fuzzer-study)
- [License](#license)

---

## What makes it different

Classic content-discovery tools (ffuf, gobuster, dirsearch, stock feroxbuster)
are **static**: the same wordlist is tried against every directory, in a fixed
order, regardless of what the target is or what has already been found. YABFF is
**adaptive** on three axes:

1. **It knows what it's looking at.** A bounded probe fingerprints the target into
   a framework profile (REST API, Spring/Java, WordPress, static), so it can seed
   *framework-appropriate* guesses — `api → v1 → users → me` on a REST API,
   `wp-content → plugins → …` on WordPress — that a generic wordlist never
   contains.
2. **It predicts per directory, in context.** A variable-order Markov/PPM model
   predicts the likely next path *token* for the specific directory being scanned,
   and re-predicts for every new directory recursion discovers. Discoveries feed
   straight back into the model (online learning), so finding `/shop/checkout/`
   makes `/shop/checkout/receipt` a prediction, not a lucky wordlist hit.
3. **It spends budget where it pays off.** A multi-armed bandit scales how many
   predictions each directory earns by its observed hit-rate, a BM25 ranker orders
   candidates by relevance to paths already seen, and soft-404 detection keeps
   catch-all noise out of the signal.

And it **carries knowledge across runs**: train a model on authorized labs with
`--learn` (or `--ml-model`), then reuse and keep adapting it on the next target.

This isn't a black box bolted on — the pay-off is **measured** (a `--ml` vs stock
A/B finds a whole API subtree the same wordlist can't reach; see
[Evaluation](#evaluation--benchmarks)) and the engine's weak spots are documented
honestly with a confusion matrix.

## The two forms

Both are Cargo workspace members and both depend on the shared engine crate
[`ferox-ml-core/`](ferox-ml-core/), so a fix or a new algorithm lands once.

| | [`feroxbuster-ml/`](feroxbuster-ml/) — the fork | [`feroxml/`](feroxml/) — the orchestrator |
|---|---|---|
| **What it is** | feroxbuster with the ML layer compiled **in-crate** | A tool that **drives** the stock `feroxbuster` binary |
| **Binary** | `feroxbuster` (with `--ml`) | `feroxml` |
| **Use it when** | You want one binary and feroxbuster's full feature set | You want the bounded-scan feedback loop + explicit learn/scan modes around an unmodified feroxbuster |
| **Prediction model** | Per-directory injection into each recursive scan | Bounded scans in a scheduler-driven loop |
| **Learns across runs** | `--ml-model <path>` | `--learn` → `--model <path>` |
| **Docs** | [feroxbuster-ml/README.md](feroxbuster-ml/README.md) | [feroxml/README.md](feroxml/README.md) |

## The algorithms

All five live in [`ferox-ml-core/`](ferox-ml-core/) as pure-Rust, network-free,
independently unit-tested engines. They consume a lightweight `ProbeResp`
(`{ url, status, headers }`) view, so they never depend on either tool's own
response types. The swap-in traits in
[`interfaces.rs`](ferox-ml-core/src/interfaces.rs) (`Classifier`, `Predictor`,
`Scheduler`) let a better algorithm drop in without touching callers.

### 1. Framework fingerprinting — K-Means / nearest-centroid
[`fingerprint.rs`](ferox-ml-core/src/fingerprint.rs) · [`profiles.rs`](ferox-ml-core/src/profiles.rs)

A bounded probe requests ~18 discriminating paths (`/wp-json`, `/actuator`,
`/api/v1`, `/swagger-ui.html`, `/xmlrpc.php`, …). Their statuses and a few headers
(`x-powered-by`, `server`, `set-cookie`, root `content-type`) become an **18-dim
feature vector**, which is classified to the nearest of four profile **centroids**
by Euclidean distance:

`REST_API` · `ENTERPRISE_JAVA_SPRING` · `WORDPRESS_CMS` · `LEGACY_STATIC`

A **confidence gate** (`fingerprint_gated`) only trusts the result when at least
one probe answered *and* the nearest centroid beats the runner-up by a margin
(`--ml-fp-margin`, default `0.10`); otherwise it falls back to a generic profile.
This is what keeps catch-all / soft-404 servers from forcing a confident wrong
guess. (`KMeansClassifier` reduces to nearest-centroid for a single target and
exists to seed real clustering when batch-probing many hosts.)

### 2. Path prediction — variable-order Markov / PPM
[`markov.rs`](ferox-ml-core/src/markov.rs)

Paths are tokenized into segments (`/api/v1/users → [api, v1, users]`). The model
holds transition counts keyed on contexts up to **order 3** (configurable via
`--ml-order`), with additive smoothing (α = 0.5) and a low-probability prune
threshold. Prediction uses **PPM back-off**: the longest matching context wins,
falling back to shorter contexts when a long one is unseen — so it generalizes
from `/a/b/c` to predict `c` after an unseen `/z/a/b`. Each profile ships a **seed
transition matrix** for cold starts; `learn()` updates the model online from every
discovered URL; models serialize to JSON and **merge** across runs.

### 3. Budget scheduling — Thompson / UCB1 bandit
[`scheduler.rs`](ferox-ml-core/src/scheduler.rs)

Each directory is a bandit "arm." `ThompsonScheduler` keeps a Beta(α, β) posterior
over the arm's hit-rate (`Ucb1Scheduler` and `RoundRobinScheduler` are
alternatives via `--ml-scheduler`). The arm's `value()` — the posterior mean —
**scales how many predictions the directory earns** (25 %–100 % of
`--ml-predictions`), so productive branches get more guesses and dead ones get the
floor. The reward is the directory's own base-pass hit-rate.

### 4. Candidate ranking — BM25
[`ranking.rs`](ferox-ml-core/src/ranking.rs)

Discovered path segments form a corpus of subword tokens (`user-profile → [user,
profile]`). Predictions are re-ranked by `prior × (1 + BM25-affinity)`, promoting
candidates whose subwords already appear on the target — relevance boosts, but
never fully overrides, the model's own probability. Disable with `--no-ml-rank`.

### 5. Soft-404 detection — SimHash + response signatures
[`dedup.rs`](ferox-ml-core/src/dedup.rs)

A 64-bit SimHash of response bodies (with Hamming distance) and a coarse
`Signature` (status + bucketed length + word/line counts) identify near-duplicate
and catch-all "soft-404" responses so they don't pollute the bandit's reward
signal.

## How a scan flows

In the fork, every directory recursion discovers becomes its own scanner, so the
model predicts *in context* for each one:

```
  --ml
   │
   ▼
 probe target (concurrent) ── GET the discriminating paths all at once
   │
   ▼
 fingerprint (gated) ──────── feature vector → nearest centroid → profile
   │                          (or generic fallback if blind / ambiguous)
   ▼
 seed + merge ─────────────── MarkovModel::seeded(profile) ⊕ any --ml-model on disk
   │
   ▼
 for each directory ─────────────────────────────────────────────┐
   │  base wordlist pass (feroxbuster's normal discovery)          │
   │     │                                                         │
   │     ▼  observe online  → model.learn(url); BM25 corpus += seg; credit dir
   │     ▼  predict_for_scan → budget = f(bandit value of this dir)│
   │     │                     Markov predict → BM25 rerank → inject extra pass
   │     ▼  recurse into newly discovered directories ─────────────┘
   ▼
 save on shutdown ─────────── persist the updated model to --ml-model
```

`feroxml` runs the same engines as an explicit **bounded-scan feedback loop**:
`probe → fingerprint → seed → {scheduler picks a directory → predict → BM25 rank →
bounded feroxbuster scan → filter soft-404 → scope-check → learn → enqueue}` until
the round/budget/depth limit, then persists the model.

## Install

Requires a [Rust toolchain](https://rustup.rs) (stable). It's a Cargo workspace,
so one build produces both binaries:

```bash
git clone https://github.com/maxamin/YABFF && cd YABFF
cargo build --release
#   ./target/release/feroxbuster   (the --ml fork)
#   ./target/release/feroxml       (the orchestrator)
```

Build or test a single crate with `-p`: `cargo build -p feroxml`,
`cargo test -p ferox-ml-core`. The fork also supports feroxbuster's own install
methods (see the upstream section of [feroxbuster-ml/README.md](feroxbuster-ml/README.md));
`feroxml` needs a `feroxbuster` binary on `PATH` or via `--ferox-binary`.

## Usage

### `feroxbuster --ml` (the fork)

```bash
# one-shot: fingerprint + per-directory predictions, learn online (nothing saved)
feroxbuster --ml -u https://target.test -w /usr/share/seclists/Discovery/Web-Content/common.txt

# persistent learning: load a model, keep adapting, save it back (implies --ml)
feroxbuster --ml-model ./model.json -u https://target.test -w common.txt
```

| Flag | Meaning (default) |
|---|---|
| `--ml` | Enable the ML layer. |
| `--ml-model <path>` | Load + update a learned model; written back on exit. Implies `--ml`. |
| `--ml-order <n>` | Max Markov order / PPM back-off depth (`3`). |
| `--ml-predictions <n>` | Base predictions injected per directory (`25`; bandit scales 25–100 %). |
| `--ml-scheduler <name>` | Budget bandit: `thompson` (default) · `ucb1` · `round_robin`. |
| `--no-ml-rank` | Disable BM25 re-ranking. |

Advanced knobs `ml_soft404` (`0.02`) and `ml_fp_margin` (`0.10`) are set in
[`ferox-config.toml`](feroxbuster-ml/ferox-config.toml.example). All other
feroxbuster flags work unchanged.

### `feroxml` (the orchestrator)

```bash
# LEARN: harvest path structure from authorized labs → train a model
feroxml --learn --targets labs.txt --i-have-authorization \
        --model .feroxml/model.json \
        --seed-wordlist /usr/share/seclists/Discovery/Web-Content/common.txt

# SCAN: load the model (merged onto the profile seed), adapt, save it back
feroxml -u https://target.test --i-have-authorization --model .feroxml/model.json
```

| Flag | Meaning |
|---|---|
| `-u, --url` / `--targets <file>` | Target URL / file of targets. |
| `--i-have-authorization` | **Required** acknowledgement of authorization. |
| `--learn` | Learn mode: train a model instead of scanning. |
| `--model <path>` | Model to load+update (scan) or write (learn). |
| `--scope <host>` | Extra in-scope hosts (repeatable); out-of-scope URLs are dropped. |
| `--scheduler` / `--classifier` | `thompson\|ucb1\|round_robin` / `nearest_centroid\|kmeans`. |
| `--top-n` / `--max-rounds` / `--max-depth` / `--request-budget` | Loop/budget limits. |
| `--seed-wordlist <file>` / `--seed-per-round <n>` | Mix baseline paths alongside predictions. |
| `--ferox-binary <path>` · `--rate-limit` · `--threads` · `-k/--insecure` · `--seed` | Driver settings. |

## Test flow

Every test is **fully offline** — no network, no live targets — so the whole suite
is deterministic and CI-friendly. Live behavior is captured into fixtures and
replayed.

```bash
cargo test --workspace            # everything: 644 tests across 26 binaries
cargo test -p ferox-ml-core       # the engines (47 tests)
cargo test -p feroxbuster ml::    # the in-crate ML façade
cargo test -p feroxbuster --test confusion -- --nocapture   # prints the confusion matrix
```

What the suites cover:

| Suite | What it verifies |
|---|---|
| `ferox-ml-core` units + [`tests/engines.rs`](ferox-ml-core/tests/engines.rs) | Every engine: all 4 profile classifications, Markov back-off / merge / JSON round-trip, each scheduler's `value()`, BM25 promotion, SimHash & soft-404 signatures, tokenizers, deterministic RNG. |
| `feroxbuster` lib `ml::` | The façade wiring: confidence gate (incl. catch-all demotion), per-directory budget scaling, init→predict→observe→save round-trip. |
| [`feroxbuster-ml/tests/lab_fingerprint.rs`](feroxbuster-ml/tests/lab_fingerprint.rs) | Replays real probe captures from the live jsintel labs; asserts correct WordPress ID, static fallback on sparse/404 targets, strict-margin demotion of catch-alls, and online learning on real crawled paths. |
| [`feroxbuster-ml/tests/confusion.rs`](feroxbuster-ml/tests/confusion.rs) | Builds the confusion matrix, guards clean accuracy ≥ 0.90, and asserts the catch-all-guard enhancement strictly reduces confident errors. |
| `feroxml` orchestrator/scope/wordlist | The feedback loop against a **fake feroxbuster runner** that replays a captured NDJSON fixture; scope enforcement and wordlist dedup. |

Reproduce the live-data fixtures and benchmarks with
[`scripts/probe_jsintel_labs.py`](scripts/probe_jsintel_labs.py) and
[`scripts/ab_benchmark.py`](scripts/ab_benchmark.py).

## Evaluation & benchmarks

The ML path is measured, not asserted — all three are reproducible and documented:

- **Stock vs `--ml` A/B** — [`docs/analysis/ml-ab-benchmark.md`](docs/analysis/ml-ab-benchmark.md).
  On a structured REST target, same binary and wordlist, `--ml` reaches the whole
  `/api/v1`+`/api/v2` subtree stock cannot find: **19 vs 11 resources**.
- **Live lab fingerprinting** — [`docs/analysis/jsintel-lab-results.md`](docs/analysis/jsintel-lab-results.md).
  Probed against Juice Shop, DVWA, WebGoat, WordPress, Django: WordPress is
  identified correctly; catch-all servers are the documented limitation.
- **Confusion matrix + enhancement** — [`docs/analysis/fingerprint-enhancements.md`](docs/analysis/fingerprint-enhancements.md).
  Fingerprint accuracy is **100 % on clean signal**, **40 % on real catch-all
  labs**; a prototyped **catch-all guard** cuts confident errors **3 → 1 with no
  loss of correct answers**. Ranked next steps (E1 tested, E2–E8 proposed) included.

## Repository layout

```
ferox-ml-core/    shared ML engine crate (fingerprint · markov · scheduler · ranking · dedup)
feroxbuster-ml/   feroxbuster fork with the native in-crate ML layer (--ml)
feroxml/          standalone adaptive ML orchestrator around feroxbuster
docs/
  analysis/       benchmarks (ML A/B, lab results, confusion matrix), capability + feature matrices, heatmap
  references/     per-tool API references + feroxbuster deep-dive + index
  prompt.md       the engineering spec the tools were built from
scripts/          ab_benchmark.py, probe_jsintel_labs.py, correlation-matrix generation
bench/            local scoring targets + wordlists for the benchmarks
tools/            third-party fuzzer source clones, reference only (git-ignored)
```

## The fuzzer study

The engine was designed after surveying nine content-discovery tools (ffuf,
gobuster, wfuzz, dirsearch, feroxbuster, dirb, kiterunner, DirBuster, patator):

- [`docs/analysis/`](docs/analysis/) — a capability comparison, a feature × project
  support matrix (35 features × 9 tools), and a feature co-occurrence correlation
  analysis with an interactive heatmap.
- [`docs/references/`](docs/references/) — per-item API references for all nine
  tools (every type, function, method, constant, macro); start at
  [`API-REFERENCES.md`](docs/references/API-REFERENCES.md), with a deep
  [feroxbuster architecture write-up](docs/references/feroxbuster-documentation.md).

## License

MIT for the original work in this repository. `feroxbuster-ml/` is a fork of
[feroxbuster](https://github.com/epi052/feroxbuster) and retains its upstream MIT
license. Bundled third-party clones under `tools/` keep their own licenses and are
not committed.
