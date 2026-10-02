# Autocomplete-algorithm comparison — Markov vs. tree models

The next-segment predictor in YABFF is pluggable. Every model implements one
`PathModel` interface (`predict` · `learn` · JSON `save`/`merge`) and is selected
at runtime with `--ml-algo` (fork) / `--algo` (feroxml), resolved by
[`ferox-ml-core/src/algo.rs`](../../ferox-ml-core/src/algo.rs). This doc compares
the four shipped algorithms and documents the live benchmark behind the headline
numbers.

## The algorithms

| `--algo` | module | structure | kind |
|----------|--------|-----------|------|
| `markov` | [`markov.rs`](../../ferox-ml-core/src/markov.rs) | variable-order n-gram transition counts | statistical, generalizing |
| `trie`   | [`freqtrie.rs`](../../ferox-ml-core/src/freqtrie.rs) | frequency prefix trie (segment edges) | tree, exact-prefix |
| `dynsdt` | [`dynsdt.rs`](../../ferox-ml-core/src/dynsdt.rs) | Dynamic Score-Decomposed Trie | tree, exact-prefix |
| `tst`    | [`tst.rs`](../../ferox-ml-core/src/tst.rs) | ternary search tree | tree, exact-prefix |

`auto` (the default) picks **DynSDT** in list mode (where the model learns the real
directory tree from scan hits) and the profile-seeded **Markov** model otherwise.

### Markov / PPM
Transition counts keyed on contexts up to order 3 (`--ml-order`), additive
smoothing (α = 0.5), and **PPM back-off**: predict from the longest matching
context, falling back to shorter contexts when a long one is unseen. This is the
only model that **generalizes** — it can predict `users` after an unseen
`/z/v1/` because it learned `v1 → users` elsewhere. It also ships a per-profile
seed matrix for cold starts. Cost: it can over-propose, probing more than a trie.

### Frequency trie (`trie`)
A plain prefix trie over path segments; each node is a prefix with a hit count.
Top-k walks to the prefix locus, then **scans the entire completion subtree** and
sorts — `O(m + c log c)` for `c` completions. The honest naive-tree baseline:
simplest and most compact, but query cost grows with the whole subtree.

### DynSDT (`dynsdt`)
The [Dynamic Score-Decomposed Trie](https://validark.dev/DynSDT/) adapted to path
segments. Each node caches `subtree_max` (the best score below it — the *score
decomposition*) and keeps its children **sorted by `subtree_max` descending** (the
horizontal heap property). Top-k is a best-first walk that only follows the best
child and the next sibling over a bounded double-ended priority queue — **`O(|p| +
k log k)`**, independent of how many completions the subtree holds. `observe()`
bumps a terminal's score and re-sorts the affected nodes up the path online, so the
structure stays correct as it learns. This output-sensitivity is why it is the
list-mode default: a directory can accumulate thousands of learned children, and
DynSDT still returns the top handful without touching the rest.

### Ternary search tree (`tst`)
The same trie stored compactly: each node holds one segment key and three links —
`lo`/`hi` are BST alternatives at the same position, `eq` advances to the next
segment. Trades the trie's per-node hash map for pointer-light BST navigation and
better cache locality. Top-k scans the completion subtree like the plain trie.

## Complexity & footprint

| `--algo` | top-k query | update | generalizes? | model after lab* |
|----------|-------------|--------|:---:|---|
| `markov` | `O(order · σ)` per context | `O(order)` | **yes** | 443 B |
| `trie`   | `O(m + c log c)` (scan subtree) | `O(m)` | no | **274 B** |
| `dynsdt` | **`O(\|p\| + k log k)`** (best-first DEPQ) | `O(\|p\| · σ log σ)` re-sort | no | 596 B |
| `tst`    | `O(m + c log c)` (scan subtree) | `O(m + log σ)` | no | 361 B |

`p`/`m` = prefix length · `c` = completions under the prefix · `k` = results wanted
· `σ` = fan-out (children per node). *Model = serialized JSON size after the Juice
Shop benchmark below (6 paths learned). DynSDT is largest because it persists the
`subtree_max` and the score-sorted child order; the plain trie is smallest.

**Key point:** the three tree models store identical counts, so they return the
**same predictions** — they differ only in query cost, update cost, and memory
layout. DynSDT wins on top-k asymptotics; the plain trie wins on memory; the TST
sits between. Markov is the odd one out: it generalizes, trading exactness and size
for the ability to guess in contexts it has never seen.

## Live benchmark — OWASP Juice Shop (authorized loopback lab)

Same target (`http://127.0.0.1:3000`), same 20-entry list directory,
`--ml-list-chunk 50`, fresh model each run:

```
feroxbuster --ml-loop --ml-algo <A> --ml-list-dir <dir> --ml-list-chunk 50 \
            --ml-model bench-<A>.json -u http://127.0.0.1:3000
```

| `--algo` | rounds | requests | resources found | soft-404 filtered | model bytes |
|----------|:---:|:---:|:---:|:---:|:---:|
| `markov` | 2 | 26 | 6 | 20 | 443 |
| `trie`   | 1 | 20 | 6 | 14 | 274 |
| `dynsdt` | 1 | 20 | 6 | 14 | 596 |
| `tst`    | 1 | 20 | 6 | 14 | 361 |

All four recover the same six resources (`/assets`, `/ftp`, `/robots.txt`,
`/main.js`, `/video`, and one more). The three tree models are indistinguishable in
behavior here — identical rounds, requests, and hits — as expected, since they hold
the same counts. **Markov takes an extra round and 6 more requests**: its profile
seed proposes additional candidates beyond the list. On this shallow target that
extra work buys nothing; the deeper benchmark below shows exactly when it pays off.

Elapsed time (~6.5 s each) is dominated by the network and the soft-404 probing,
not the algorithm — at this scale the model's own cost is noise. The complexity
differences in the table above matter at scale (large directories, many learned
children), not on a six-path lab.

## Deeper target — where the algorithms actually diverge

A six-path lab can't separate the models. To probe depth, a deterministic,
network-free harness ([`ferox-ml-core/examples/algo_bench.rs`](../../ferox-ml-core/examples/algo_bench.rs),
run with `cargo run -p ferox-ml-core --example algo_bench`) builds a **deep,
repetitive** synthetic site — `{api,app,shop}/v{1,2,3}/{8 leaves}`, 84 resources —
and measures the three places the choice could matter.

### A. Online prediction recall (train on `*/v1/*`, k = 8)

Each model learns the directory structure plus **only `v1`'s** leaves, then predicts
children of a seen version (`v1`) and of the held-out versions (`v2`, `v3`):

| `--algo` | seen `v1` | held-out `v2`,`v3` |
|----------|:---:|:---:|
| `markov` | 100% | **0%** |
| `trie`   | 100% | **0%** |
| `dynsdt` | 100% | **0%** |
| `tst`    | 100% | **0%** |

All four are identical: they **memorize** perfectly and **none generalizes across
sibling directories** from online learning alone. This corrects a common
assumption — Markov's `n`-gram back-off goes `[api,v2] → [v2] → ⌀`, and the empty
context only holds *first* segments, so it does not transfer `v1`'s leaf names to an
unseen `v2`. For discovery purposes the tree models and online Markov are
equivalent; they differ only in query cost and memory (the first table).

### B. Cold-start recall from a profile seed (zero observations, k = 12)

The one capability that is **not** shared: Markov ships per-profile seed matrices.
Built on the `REST_API` profile with **no observations**, each model predicts the
canonical children at increasing depth:

| `--algo` | `/` | `/api` | `/api/v1` | `/api/v2` |
|----------|:---:|:---:|:---:|:---:|
| `markov` | **100%** | **100%** | **100%** | **100%** |
| `trie`   | 0% | 0% | 0% | 0% |
| `dynsdt` | 0% | 0% | 0% | 0% |
| `tst`    | 0% | 0% | 0% | 0% |

This is the real, documented source of Markov's discovery edge on structured
targets (cf. the stock-vs-`--ml` A/B: **19 vs 11** resources): the seed predicts
`v1`/`v2` and their children before anything is observed. The tree models have no
cold-start mechanism, so they rely entirely on the wordlist until they have learned.

### C. End-to-end list-mode scan (84-resource site, 14 tokens + 60 junk)

| `--algo` | found | coverage | requests | rounds |
|----------|:---:|:---:|:---:|:---:|
| `markov` | 3 | 4% | 106 | 19 |
| `trie`   | 3 | 4% | 90 | 19 |
| `dynsdt` | 3 | 4% | 90 | 19 |
| `tst`    | 3 | 4% | 90 | 19 |

All four tie at the three top-level directories. This is a property of **list
mode**, not the predictor: the wordlist pool is consumed by a single forward cursor
that does not re-try words per directory, and online learning can't bootstrap a leaf
it has never seen — so depth is bounded identically for every model. (Markov issues
a few more requests from its seed but finds nothing extra here.) Reaching the deep
leaves needs either a profile seed (non-list mode) or a wordlist re-applied per
directory — not a different tree structure.

### Takeaway

On a deep target the **predictor choice does not change *what* the tree models
find** — they memorize identically and none generalizes online. Two levers do
matter: **cold-start priors** (only Markov has them → use it on cold, structured
targets) and **query cost/memory at scale** (DynSDT's output-sensitive `O(|p| + k
log k)` top-k → use it once a directory has accumulated many learned children). The
best of both is `--learn` to warm a model, then scan with the algorithm whose query
profile fits.

## Cross-run accumulation

Every model persists to `--ml-model` and merges a prior model of the same algorithm
on load, so repeated runs accumulate. Verified for DynSDT: two runs over the same
lab take each discovered path's score from `1.0 → 2.0` (`list_mode_persists_a_dynsdt_model_that_accumulates_across_runs`
in `ferox-ml-core/tests/engines.rs`). The same `save_json` / `merge_json` contract
holds for all four.

## How to choose

- **Cold start on a structured target (REST, Spring, WordPress, …)** → `markov`.
  Its profile seed predicts the canonical tree before anything is observed (table B);
  no tree model can.
- **List-driven fuzzing once you have/accumulate observations** → `dynsdt` (default).
  Output-sensitive top-k as the learned tree grows large.
- **Smallest persisted model / simplest baseline** → `trie`.
- **Memory-locality-sensitive layout experiment** → `tst`.
- **Best of both** → `--learn` to warm a model from authorized labs, then scan with
  the algorithm whose query profile fits.

Reproduce: build the workspace, point `--ml-list-dir` at any directory of
wordlists, and sweep `--ml-algo` over `markov trie dynsdt tst` against an authorized
target.
