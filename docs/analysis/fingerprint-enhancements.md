# Fingerprinter evaluation & enhancements

A labelled evaluation of the framework fingerprinter (`ferox-ml-core`), a
**confusion matrix**, and a ranked list of enhancements — the first of which is
prototyped and measured here. Everything below is produced by
[`feroxbuster-ml/tests/confusion.rs`](../../feroxbuster-ml/tests/confusion.rs),
which also guards these results as a regression test:

```bash
cargo test -p feroxbuster --test confusion -- --nocapture
```

## 1. Confusion matrix — clean labelled set

12 hand-built probe samples (3 per profile) that faithfully represent each
framework. The classifier is perfect here, which confirms the centroids and
feature vector are sound:

| true \ pred | REST | SPRING | WP | STATIC | recall |
|---|---:|---:|---:|---:|---:|
| **REST** | 3 | 0 | 0 | 0 | 1.00 |
| **SPRING** | 0 | 3 | 0 | 0 | 1.00 |
| **WP** | 0 | 0 | 3 | 0 | 1.00 |
| **STATIC** | 0 | 0 | 0 | 3 | 1.00 |
| **precision** | 1.00 | 1.00 | 1.00 | 1.00 | **acc = 1.00** |

### Confidence-gate margin sweep

How `--ml-fp-margin` trades coverage for safety on the clean set (accuracy stays
1.00 because the clean samples are unambiguous; the margin only abstains):

| margin | confident | accuracy-when-confident | abstained |
|---:|---:|---:|---:|
| 0.00 | 12/12 | 1.00 | 0 |
| 0.10 | 12/12 | 1.00 | 0 |
| 0.25 | 11/12 | 1.00 | 1 |
| 0.50 | 10/12 | 1.00 | 2 |
| 1.00 | 8/12 | 1.00 | 4 |

## 2. Confusion matrix — real jsintel labs (the hard case)

Replaying the live captures (`tests/fixtures/jsintel_labs.json`) with
best-judgment true labels (DVWA/Django are classic PHP / server-rendered apps
with no exact profile → `LEGACY_STATIC`):

| lab | true | predicted | correct |
|---|---|---|:-:|
| juice-shop | REST | SPRING | ✗ |
| dvwa | STATIC | WP | ✗ |
| webgoat | SPRING | STATIC | ✗ |
| wordpress | WP | WP | ✓ |
| django | STATIC | STATIC | ✓ |

| true \ pred | REST | SPRING | WP | STATIC |
|---|---:|---:|---:|---:|
| **REST** | 0 | 1 | 0 | 0 |
| **SPRING** | 0 | 0 | 0 | 1 |
| **WP** | 0 | 0 | 1 | 0 |
| **STATIC** | 0 | 0 | 1 | 1 |

**Field accuracy: 2/5 = 0.40.** The classifier is perfect on clean signal but
collapses on real servers for two reasons: **catch-all / soft-404 servers**
(Juice Shop, DVWA answer 200 to *every* probe, so path-presence is noise) and
**missing profiles** (DVWA is PHP-not-WordPress; WebGoat's app lives under
`/WebGoat/` so the probe sees only 404s). This gap is what the enhancements target.

## 3. Enhancements

Ranked by impact/effort. ✅ = **shipped** in the engine and measured;
⬜ = proposed, with how to test it.

### ✅ E1 — Catch-all / soft-404 guard  *(shipped)*

Detect a catch-all server (high fraction of "present" probe paths) with **no
strong, hard-to-fake discriminator** (WordPress cookie, `JSESSIONID`,
`xmlrpc.php`→405) and **abstain** instead of trusting path-presence. Measured on
the field labs:

| | confident-correct | confident-wrong | abstained |
|---|---:|---:|---:|
| baseline (raw classifier) | 2 | 3 | 0 |
| **+ catch-all guard** | **2** | **1** | **2** |

It cuts confident mistakes from 3 to 1 **without losing a single correct answer** —
Juice Shop and DVWA (wrong guesses) become safe abstentions, while WordPress is
kept because its cookie + `xmlrpc.php` 405 survive the catch-all.

**Shipped** in [`ferox-ml-core/src/fingerprint.rs`](../../ferox-ml-core/src/fingerprint.rs)
as `is_catch_all` / `present_fraction` / `has_strong_discriminator`
(present-fraction measured against `PROBE_PATHS`, so it is correct whether or not
404s are in the probe slice). It is wired into the fork's `fingerprint_gated`
(abstains up front) and into feroxml's orchestrator (falls back to the generic
profile), and guarded by tests in `confusion.rs`, `lab_fingerprint.rs`, and
`ferox-ml-core/tests/engines.rs`. Next step is E2 — replace the global
present-fraction with per-path soft-404 body signatures from `dedup`.

### ✅ E2 — Per-path soft-404 scoring  *(shipped)*

Replaces E1's global present-fraction with **per-path truth**: probe a few random,
almost-certainly-absent paths to learn the server's soft-404 response
[`Signature`](../../ferox-ml-core/src/dedup.rs) (status + bucketed length + word /
line counts), then demote any discriminating probe whose response *matches* that
baseline to status 404. A catch-all's uniform 200 body no longer reads as a marker
— `present` collapses to the real, sparse signal — while a genuine 200 with a
distinct body survives.

**Shipped** in [`ferox-ml-core/src/fingerprint.rs`](../../ferox-ml-core/src/fingerprint.rs)
as `learn_soft_404` + `apply_soft_404` (over a new `content_length` /
`word_count` / `line_count` on `ProbeResp`, with `ProbeResp::signature()`). Wired
in: the fork's `main.rs` reads probe bodies and probes 3 random paths
(`ml::score_soft_404` before fingerprinting); feroxml's orchestrator learns from
the random probes it already sends and cleans the probe views before classifying.
Covered by `ferox-ml-core/tests/engines.rs`. E2 runs *before* E1, which remains a
backstop for callers with no body data (e.g. fixtures with sizes 0). Next: feed
full-body SimHash (not just the coarse signature) for finer soft-404 matching.

### ⬜ E3 — Feature weighting

Classification is Euclidean over an 18-dim, mostly-binary vector where a flaky
presence bit counts as much as a `JSESSIONID` cookie. Weight the vector so
header/cookie/status discriminators dominate path-presence. *Test:* re-run the
field matrix; expect DVWA (PHP, no WP cookie) to stop reading as WordPress.

### ⬜ E4 — More profiles (NODE_SPA, PHP_GENERIC, DJANGO)

Four profiles force PHP apps and SPAs into ill-fitting buckets (DVWA→WP,
Juice-Shop→Spring). Add centroids + seed matrices for the common stacks and add
SPA discriminators to the probe (`/_next/`, `/assets/`, `manifest.webmanifest`,
GraphQL introspection). *Test:* add labelled samples for the new profiles to the
clean set and confirm the diagonal holds.

### ⬜ E5 — Calibrated confidence

Convert centroid distances to a softmax probability over `-distance`, and expose
it instead of (or alongside) the raw margin. *Test:* a reliability check — bucket
predictions by reported confidence and confirm empirical accuracy tracks it.

### ⬜ E6 — Online re-fingerprinting

Re-run classification as real paths are discovered (finding `/wp-content/`
confirms WordPress even if the probe was ambiguous). Hook it into
`ml::observe`. *Test:* feed a WordPress crawl trace and assert the profile
corrects itself mid-scan.

### ⬜ E7 — Real K-Means over many hosts

`KMeansClassifier` degenerates to nearest-centroid for a single target.
Batch-probing many authorized hosts enables genuine clustering and data-driven
centroid refresh. *Test:* cluster a labelled multi-host set and compare recovered
centroids to the seeds.

### ⬜ E8 — Markov subword back-off

When no segment-level transition exists for a context, back off to subword
prediction (`tokenize::subword_tokens` already exists) so `getUserById`-style
siblings are still proposed. *Test:* learn `getUserById`, predict after
`getUser…`, assert a subword-derived candidate appears.

## Takeaway

The fingerprinter is accurate on clean signal (100%) and the confidence gate
degrades safely, but real-world catch-all servers drop field accuracy to 40%.
The catch-all guard (E1) and per-path soft-404 scoring (E2) — both now **shipped**
in the engine — turn the catch-all field cases from confident-wrong into safe
abstentions (confident errors 3→1, no correct answers lost), E2 doing it with
per-path precision rather than a global heuristic. E3–E4 (feature weighting, more
profiles) are the path to actually *classifying* those cases rather than abstaining.
