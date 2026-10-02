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
the same counts. **Markov takes an extra round and 6 more requests**: its back-off
proposes additional candidates beyond the list, which is exactly its
generalization showing up as extra probing. On this shallow target that extra work
buys nothing; on a deep, structured target it is what lets Markov reach subtrees a
pure exact-prefix trie would never guess (see the stock-vs-`--ml` A/B, where the
generalizing model reaches the whole `/api/v1`+`/api/v2` subtree).

Elapsed time (~6.5 s each) is dominated by the network and the soft-404 probing,
not the algorithm — at this scale the model's own cost is noise. The complexity
differences in the table above matter at scale (large directories, many learned
children), not on a six-path lab.

## Cross-run accumulation

Every model persists to `--ml-model` and merges a prior model of the same algorithm
on load, so repeated runs accumulate. Verified for DynSDT: two runs over the same
lab take each discovered path's score from `1.0 → 2.0` (`list_mode_persists_a_dynsdt_model_that_accumulates_across_runs`
in `ferox-ml-core/tests/engines.rs`). The same `save_json` / `merge_json` contract
holds for all four.

## How to choose

- **List-driven fuzzing against one target** → `dynsdt` (default). Output-sensitive
  top-k as the learned tree grows.
- **Cold start / sparse data / want generalization** → `markov`. It guesses in
  unseen contexts; it ships seed matrices.
- **Smallest persisted model / simplest baseline** → `trie`.
- **Memory-locality-sensitive layout experiment** → `tst`.

Reproduce: build the workspace, point `--ml-list-dir` at any directory of
wordlists, and sweep `--ml-algo` over `markov trie dynsdt tst` against an authorized
target.
