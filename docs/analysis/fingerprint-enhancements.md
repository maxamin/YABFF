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
| juice-shop | REST | REST | ✓ |
| dvwa | STATIC | WP | ✗ |
| webgoat | SPRING | STATIC | ✗ |
| wordpress | WP | WP | ✓ |
| django | STATIC | STATIC | ✓ |

| true \ pred | REST | SPRING | WP | STATIC |
|---|---:|---:|---:|---:|
| **REST** | 1 | 0 | 0 | 0 |
| **SPRING** | 0 | 0 | 0 | 1 |
| **WP** | 0 | 0 | 1 | 0 |
| **STATIC** | 0 | 0 | 1 | 1 |

**Field accuracy: 3/5 = 0.60** (raw classifier, after E3 weighting — it was 2/5
before weighting; Juice Shop now reads REST instead of Spring). The two remaining
misses show the hard cases the enhancements target: **catch-all / soft-404
servers** (Juice Shop, DVWA answer 200 to *every* probe, so path-presence is
noise — note Juice Shop's "correct" REST here is a catch-all coincidence E1/E2
rightly distrust) and **missing profiles** (DVWA is PHP-not-WordPress; WebGoat's
app lives under `/WebGoat/`, so the probe sees only 404s).

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
| baseline (raw classifier) | 3 | 2 | 0 |
| **+ catch-all guard** | **2** | **1** | **2** |

It abstains the two catch-alls (Juice Shop, DVWA) — cutting confident errors (DVWA
was wrong) and declining Juice Shop's coincidentally-correct REST guess, since a
server that answers every path can't be trusted. WordPress is kept (its cookie +
`xmlrpc.php` 405 survive the catch-all). The guard's invariant, asserted in the
test: it **only ever abstains genuine catch-alls**, so it never drops a correct
answer on a trustworthy target.

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

### ✅ E3 — Feature weighting  *(shipped)*

Classification was plain Euclidean over an 18-dim, mostly-binary vector where a
flaky presence bit counted as much as a `JSESSIONID` cookie. The metric is now
**weighted Euclidean** ([`FEATURE_WEIGHTS`](../../ferox-ml-core/src/profiles.rs)):
session cookies, servlet/PHP/JSP signals and `X-Powered-By` are weighted up (2–3×),
ubiquitous `robots.txt` down (0.5×), so a target with a weak accidental marker but
a strong contradicting discriminator is classified by the discriminator.

**Shipped** in [`fingerprint.rs`](../../ferox-ml-core/src/fingerprint.rs)
(`weighted_euclidean` + `classify_with_weights`; `NearestCentroid` now uses
`FEATURE_WEIGHTS`). A centroid still classifies to itself under any positive
weights, so this is a safe refinement — the clean set stays at 100% and the
`engines.rs` test confirms the weighted metric keeps WordPress/Spring correct even
with accidental cross-framework presence bits injected.

*Measured effect:* weighting raised raw field accuracy 2/5 → 3/5 — Juice Shop now
reads REST instead of Spring, because the real REST signals outweigh its spurious
`/actuator` presence. Flips are still rare with only four well-separated centroids
(the clean set was already 100%), so the bigger pay-off comes with E4: as more,
closer profiles are added, a correctly-weighted metric is what keeps them
separable.

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
degrades safely; real-world catch-all servers are the hard case. Three
enhancements are now **shipped** in the engine: E1 (catch-all guard) and E2
(per-path soft-404 scoring) turn untrustworthy catch-alls into safe abstentions,
and E3 (feature weighting) lifted raw field accuracy 2/5 → 3/5 by letting strong
discriminators outweigh accidental presence. E4 (more, closer profiles) is the
next step — actually *classifying* the PHP / SPA cases rather than abstaining, with
the weighted metric from E3 keeping the denser profile space separable.
