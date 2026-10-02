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
| `markov` | 84 | **100%** | 962 | 226 |
| `trie`   | 84 | **100%** | 1006 | 247 |
| `dynsdt` | 84 | **100%** | 1009 | 247 |
| `tst`    | 84 | **100%** | 1006 | 247 |

List mode **re-applies the wordlist to every directory** (each arm walks the pool
with its own cursor), so the scan fully recurses and every model reaches 100%
coverage. The predictor no longer changes *reach* here — every leaf name is in the
wordlist, so a per-directory walk finds them all regardless of algorithm — but it
does change **efficiency**: Markov reaches full coverage in ~4% fewer requests,
because its predictions surface a directory's children before the cursor walks to
them, bringing recursion forward. (A predictor only changes *reach* for paths the
wordlist does **not** contain — the cold-start case in B.)

> Earlier this benchmark tied all models at 3 resources (4%): list mode used a single
> global cursor that consumed each word once across all directories, so depth was
> bounded by the mode. That was fixed — the pool is now per-directory.

### D. Real deep lab — OWASP Juice Shop (before/after the per-directory fix)

The synthetic result above reproduces on a live lab. Juice Shop (`127.0.0.1:3000`)
has a genuine depth-4 chain of directories, `/assets → /assets/public →
/assets/public/images → /assets/public/images/products` (all `301`), plus `/ftp`.
Same binary, same 48-entry wordlist (8 real path tokens + 40 noise),
`--ml-algo dynsdt --ml-list-chunk 100`, run on the commit **before** and **after**
the fix:

| list mode | rounds | requests | resources | deepest found |
|-----------|:---:|:---:|:---:|---|
| **global cursor (pre-fix)** | 1 | 48 | **2** | `/assets`, `/ftp` (depth 1) |
| **per-directory (post-fix)** | 6 | 288 | **7** | `/assets/public/images/products` (depth 4) |

Pre-fix drained the entire wordlist at the root in a single round, so `/assets` and
`/ftp` were discovered but never expanded — the cursor was exhausted and the scan
stopped at depth 1. Post-fix re-applies the wordlist under each discovered
directory, recursing to the full depth-4 chain. (`/rest` and `/api` return `500`,
which isn't a success code, so they aren't expanded — a scope property of the target,
not the fix.) Reproduce against any authorized deep target with a directory of
wordlists and `--ml-loop --ml-list-dir`.

**`markov` vs `dynsdt` on the same lab.** With the fix in place, the two algorithms
are **indistinguishable for discovery** here — same wordlist, same budget:

| `--algo` | `--ml-list-chunk` | rounds | requests | resources | model |
|----------|:---:|:---:|:---:|:---:|:---:|
| `markov` | 100 | 6 | 288 | 7 | — |
| `dynsdt` | 100 | 6 | 288 | 7 | — |
| `markov` | 3 | 25 | 85 | 7 | 1155 B / 7 ctx rows |
| `dynsdt` | 3 | 25 | 84 | 7 | **660 B / 8 nodes** |

Identical reach (the full depth-4 set) and all but one request, at both a coarse and
a tight chunk. This is the expected result: list mode runs both **unseeded**, online
learning is exact (neither generalizes across siblings, §A), and the per-directory
walk finds every token regardless of predictor. The only measurable difference is
**representation** — the models serialize to different sizes depending on whether the
learned structure has more distinct n-gram contexts or more trie nodes (here the
Markov model is larger; on the warm-cache run below it is smaller — it is
dataset-dependent, not a fixed ratio). Markov's discovery edge appears only with its
**profile seed** (non-list mode, §B), which list mode disables. Choose between them
on query cost/memory vs cold-start priors, not on list-mode reach.

**Same comparison with the ranked-pool cache warm.** Repeating the comparison with
the full SecLists tree (`Discovery/Web-Content`, 4.4M entries, `--ml-list-max 300`)
after the pool cache is warm — so the ~160 s one-time ranking is out of the picture
and only scan behavior is timed:

| `--algo` | pool load | elapsed | rounds | requests | resources | model |
|----------|-----------|:---:|:---:|:---:|:---:|:---:|
| `markov` | from cache | 9 s | 10 | 1,511 | 12 | 838 B |
| `dynsdt` | from cache | 8 s | 10 | 1,513 | 12 | 1,124 B |

Both load `(recursive, from cache)` in a few seconds (vs ~159 s cold), then run
neck-and-neck: identical reach (12 resources, 10 rounds), requests within 2, elapsed
within noise. With the load cost removed the predictors are, again, interchangeable
for discovery in list mode — the only delta is model size (direction flips vs the
deep-lab run above, confirming it is representation/dataset-dependent). See
[`recursive-seclists-depth.md`](recursive-seclists-depth.md) §3c for the cache.

### E. Non-list mode on the real lab — seeded Markov vs empty DynSDT

§B shows the seed winning on a target *built* to match the `REST_API` profile. The
live lab shows the other half of the story: the seed only helps when the target's
structure matches the profile's assumptions. Juice Shop fingerprints as `NODE_SPA`;
run in non-list mode with **no wordlist** (pure prediction, `--ml-loop --ml-algo
<a>`):

| `--algo` (non-list) | profile | rounds | requests | resources found |
|---------------------|---------|:---:|:---:|:---:|
| `markov` (seeded)   | NODE_SPA | 3 | **51** | 3 |
| `dynsdt` (empty)    | NODE_SPA | 3 | **32** | 3 |

Both discover the **same** `/robots.txt`, `/assets`, `/media` — all from the probe
phase, which both share. The `NODE_SPA` seed's deeper guesses (`assets → index.js`,
`api → v1`) don't match Juice Shop's actual layout (`/assets/public/…`, `/rest/…`),
so seeded Markov finds **nothing extra** and spends **19 more requests (+59%, 51 vs
32)** chasing seed tokens that `404` — pure waste, 0 resources gained, and the result
is deterministic across runs. DynSDT, with no seed, simply learns from the probe and
predicts only what it has seen.

**Controlled confirmation (same seed as §F).** To isolate the waste from any target
quirk, a WordPress **decoy** — identical fingerprint signals to §F (so the same
`WORDPRESS_CMS` seed loads) but with the `wp-*` directory tree **removed**:

| target | `--algo` | requests | resources | deepest |
|--------|----------|:---:|:---:|:---:|
| §F real WP (tree present) | `markov` seeded | 233 | **21** | depth 3 |
| decoy (tree absent)       | `markov` seeded | 18 | 5 | depth 1 |
| decoy (tree absent)       | `dynsdt` empty  | 14 | 4 | depth 1 |

Same seed, same fingerprint — only the structure differs. With the tree present the
seed is worth 21 resources to depth 3 (§F); with it absent the seed predicts
`wp-admin`/`wp-content`/`wp-includes` that all `404`, recovering just one real path
(`/wp-login.php`) the probe missed and wasting the rest, with **no depth at all**.
The seed is a bet on the target matching the profile: it pays off handsomely when it
does (§B, §F) and costs wasted requests when it doesn't (§E Juice Shop, this decoy).

### F. WordPress target — the seed pays off

§E showed the seed costing requests on an off-profile target. The matching case:
a WordPress target, non-list mode, **no wordlist** (pure prediction). (Docker
networking is unavailable in this sandbox — no bridge, no port publishing — so this
is a faithful WordPress **mock**: real WP paths, headers, `wordpress_*` cookies and
`xmlrpc.php` → 405, serving the standard `wp-content/{plugins,themes,uploads}/…`
tree. It fingerprints as `WORDPRESS_CMS`, so the engine path is identical to a real
WP.)

| `--algo` (non-list) | profile | rounds | requests | resources | max depth |
|---------------------|---------|:---:|:---:|:---:|:---:|
| `markov` (seeded)   | WORDPRESS_CMS | 14 | 233 | **21** | **3** |
| `dynsdt` (empty)    | WORDPRESS_CMS | 1 | 14 | 4 | 1 |

Both share the same 4 probe-phase hits (`/index.php`, `/robots.txt`, `/wp-json`,
`/xmlrpc.php`). From there the `WORDPRESS_CMS` **seed** drives Markov through the
whole tree with zero wordlist — `wp-admin`, `wp-content`, `wp-includes`, then
`wp-content/{plugins,themes,uploads}`, then `plugins/{akismet,woocommerce}`,
`themes/{twentytwentyone,twentytwentytwo}`, `uploads/{2023,2024}` — **21 resources to
depth 3, 17 of them predicted**. Empty DynSDT has no seed and `wp-admin`/`wp-content`
aren't among the probe paths, so it never learns a directory to expand: it stops at
the 4 probe files, depth 1.

This is the live counterpart of §B: when the target matches a shipped profile, the
seed is worth a **5×** jump in resources and the difference between a flat scan and a
depth-3 recursion. Pair it with §E (where the same seed wasted requests on an
off-profile target) for the full picture.

### Takeaway

On a deep target with a wordlist that covers the leaf names, **every algorithm now
reaches full coverage** (the per-directory walk does the work); the predictor only
shifts **efficiency** (Markov marginally ahead here by bringing recursion forward).
The predictor changes **reach** only where the wordlist falls short — the
**cold-start** case, where seeded Markov predicts canonical structure no tree model
can. And it changes **cost at scale** — DynSDT's output-sensitive `O(|p| + k log k)`
top-k matters once a directory accumulates many learned children. Rules of thumb:
`markov` for cold targets **that match a shipped profile** (the seed is a bet — it
pays off when the structure matches, §B, and wastes requests when it doesn't, §E);
`dynsdt` (default) once observations accumulate or when the target is off-profile;
`--learn` to warm a model, then scan with the algorithm whose query profile fits.

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
