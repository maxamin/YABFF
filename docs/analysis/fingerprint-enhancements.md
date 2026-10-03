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

21 hand-built probe samples (3 per profile across all seven E4 profiles) that
faithfully represent each framework. The classifier is perfect here, which
confirms the centroids and feature vector are sound:

| true \ pred | REST | SPRING | WP | STATIC | PHP | SPA | DJANGO | recall |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| **REST** | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 1.00 |
| **SPRING** | 0 | 3 | 0 | 0 | 0 | 0 | 0 | 1.00 |
| **WP** | 0 | 0 | 3 | 0 | 0 | 0 | 0 | 1.00 |
| **STATIC** | 0 | 0 | 0 | 3 | 0 | 0 | 0 | 1.00 |
| **PHP** | 0 | 0 | 0 | 0 | 3 | 0 | 0 | 1.00 |
| **SPA** | 0 | 0 | 0 | 0 | 0 | 3 | 0 | 1.00 |
| **DJANGO** | 0 | 0 | 0 | 0 | 0 | 0 | 3 | 1.00 |
| **precision** | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | **acc = 1.00** |

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
best-judgment true labels for the actual stack (DVWA → `PHP_GENERIC`, Django →
`DJANGO` now that E4 added those profiles):

| lab | true | predicted | correct |
|---|---|---|:-:|
| juice-shop | REST | REST | ✓ |
| dvwa | PHP | WP | ✗ |
| webgoat | SPRING | STATIC | ✗ |
| wordpress | WP | WP | ✓ |
| django | DJANGO | DJANGO | ✓ |

**Field accuracy: 3/5 = 0.60** (raw classifier). The arc across enhancements: 2/5
before E3; E3 fixed Juice Shop (REST, not Spring); E4 made Django a *genuine* hit
(its root sets a `csrftoken` cookie — the DJANGO profile keys on it — rather than a
coincidental `LEGACY_STATIC`). The two remaining misses are the honest hard cases:
**catch-all servers** (DVWA answers 200 to every probe, so its WP-marker presence
is noise — E1/E2 abstain it in gated mode; Juice Shop's correct REST is likewise a
catch-all the gated path distrusts) and **no same-origin signal** (WebGoat's app
lives under `/WebGoat/`, so the probe sees only 404s).

> **Base-path discovery (E9) addresses the second case.** When the application is
> mounted under a context path, `--discover-base-path` (or an explicit
> `--base-path /WebGoat/`) reroots the whole scan — probe included — under that
> path, and the discriminating probes are matched **base-relative** so
> `has("api")` / `root_json` fire there instead of against the empty origin root.
> WebGoat then fingerprints on real signal rather than 404s. See E9 below. (The
> catch-all case remains the honest limitation: when a host answers everything
> with no hard-to-fake discriminator, abstaining is the correct call, not a guess.)

## 3. Enhancements

All eight are now ✅ **shipped** in the engine and covered by tests; the notes
below record what each does and how it was measured.

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

### ✅ E4 — More profiles  *(shipped)*

Four profiles forced PHP apps and SPAs into ill-fitting buckets (DVWA→WP,
Juice-Shop→Spring). The engine now has **seven**: added `PHP_GENERIC`, `NODE_SPA`,
and `DJANGO`, each with a centroid and a seed Markov matrix. The feature vector
grew two discriminators — `sig_spa` (`manifest.webmanifest` / `/_next` / `/assets`)
and `sig_django` (a `csrftoken` cookie or `/static/admin`) — and the probe set
grew the matching paths.

**Shipped** in [`profiles.rs`](../../ferox-ml-core/src/profiles.rs) (centroids,
`FEATURE_WEIGHTS`, `PROBE_PATHS`, `PROFILES`, seed matrices) and
[`fingerprint.rs`](../../ferox-ml-core/src/fingerprint.rs) (the two new features).
The clean set is a perfect **7×7** diagonal (21 samples), and on the real labs E4
turned Django into a *genuine* hit — the live Django lab sets a `csrftoken` cookie,
so it now classifies as `DJANGO` instead of coincidentally as `LEGACY_STATIC`.
DVWA (a PHP catch-all) is now correctly labelled `PHP_GENERIC` as ground truth; its
raw guess is still WP (catch-all noise), which E1/E2 abstain in gated mode.

*Design note:* the new near-origin centroids (SPA/Django/PHP sit close to the
zero vector) narrowed the margin for signal-less targets (WebGoat's static-fallback
gap went ~1.5 → ~0.9), which is why E3's weighted metric matters more now — it
keeps the denser profile space separable. Natural next step: `RAILS`, `ASPNET`,
`FLASK`, and GraphQL-introspection as its own signal.

### ✅ E5 — Calibrated confidence  *(shipped)*

`softmax_confidence` / `classify_confidence` in
[`fingerprint.rs`](../../ferox-ml-core/src/fingerprint.rs) turn centroid distances
into a probability distribution over profiles (`softmax(-β·distance)`), so callers
get a comparable `[0,1]` confidence rather than a raw distance/margin. The fork
logs it (`p=0.87`) alongside the gate decision. The `engines.rs` test confirms the
probabilities sum to 1 and peak on the nearest centroid.

### ✅ E6 — Online re-fingerprinting  *(shipped)*

`ProfileTracker` (in `fingerprint.rs`) starts from the probe's profile and
accumulates evidence from discovered paths — each segment in a profile's
characteristic vocabulary votes for it, weighted by specificity (shared tokens
count less). Wired into the fork's `ml::observe`: when the evidence favors a
different profile, that profile's seed is **merged** into the live Markov model
(additive, so nothing learned is lost). *Tested:* a tracker seeded `LEGACY_STATIC`
flips to `WORDPRESS_CMS` after observing `/wp-content`, `/wp-admin`.

### ✅ E7 — Real K-Means over many hosts  *(shipped)*

`kmeans_fit` runs Lloyd's algorithm (weighted metric) over many host feature
vectors, seeded from the profile centroids so cluster identity stays aligned to
the named profiles — for batch centroid refresh across a set of authorized hosts.
The single-target classifier is unchanged. *Tested:* feeding the seed centroids
recovers them exactly; a host vector assigns to its profile's refined centroid.

### ✅ E8 — Markov subword back-off  *(shipped)*

When no context has segment-level evidence for the last segment, `predict` backs
off to learned tokens that share subword tokens with it (Jaccard-scored). Because
URL segments are lowercased, it keys on the boundaries that survive — separators
(`-` `_` `.`) and digit edges — so after learning `user-profile` it surfaces it
for `user-settings`. *Tested* in `engines.rs`.

## Takeaway

The fingerprinter is accurate on clean signal (now a perfect 7×7 diagonal) and the
confidence gate degrades safely; real-world catch-all servers are the hard case.
All eight roadmap enhancements are now **shipped**: E1 (catch-all guard) and E2
(per-path soft-404 scoring) turn untrustworthy catch-alls into safe abstentions;
E3 (feature weighting) lifted raw field accuracy 2/5 → 3/5; E4 (seven profiles
incl. PHP_GENERIC / NODE_SPA / DJANGO) added the missing stacks and made Django a
genuine hit; E5 (calibrated confidence) exposes a `[0,1]` probability; E6 (online
re-fingerprinting) lets a scan self-correct its profile from discovered paths; E7
(real K-Means) enables data-driven centroid refresh across many hosts; and E8
(Markov subword back-off) proposes related siblings when a context is novel.

## See also — which classifier?

[`fingerprint-classifier-benchmark.md`](fingerprint-classifier-benchmark.md) (with
an HTML confusion-matrix heatmap,
[`fingerprint-classifier-heatmap.html`](fingerprint-classifier-heatmap.html))
compares eight classifiers — hand centroids, (trained) nearest-centroid, Bernoulli
Naive Bayes, logistic regression, k-NN, linear SVM, random forest, and GBDT — on
this feature space under observation noise. Takeaway: the **algorithm is not the
bottleneck** (the zero-data hand centroids stay within ~0.02 accuracy of the best
trained model); richer features and a labelled host corpus are the real levers,
after which Naive Bayes / logistic regression are the natural trained upgrades
behind the `Classifier` trait.

## Richer feature extraction (shipped)

Following the benchmark's conclusion that *features, not the algorithm*, are the
lever, the feature vector grew from 20 → **24 dims** with technology signals that
path-presence alone misses (in [`fingerprint.rs`](../../ferox-ml-core/src/fingerprint.rs)):

- **`cookie_node`** — Node session cookies (`connect.sid` / `next-auth`) → NODE_SPA.
- **`cookie_php_fw`** — PHP-framework sessions (`laravel_session` / `ci_session` /
  `symfony`) → PHP_GENERIC.
- **`sec_headers`** — modern security headers (CSP / HSTS / X-Frame-Options /
  X-Content-Type-Options): a modern-app vs classic-static cue.
- **`heavy_html`** — a real rendered HTML page was served (word-count cue) vs a
  small JSON/404 body: separates CMS/SPA/static from JSON APIs.

Effect: on the (synthetic) benchmark, every classifier improved — e.g. Bernoulli
Naive Bayes 0.792 → 0.808, the zero-data hand centroids 0.775 → 0.783 — confirming
richer features lift the whole field. Gated lab verdicts are unchanged (WordPress →
WP, Django → DJANGO, WebGoat → static, Juice Shop / DVWA catch-alls abstained). The
natural next features (needing a probe-transport change) are favicon hashes and
JS-framework detection from SPA bundles.

### ✅ E9 — Base-path (application-root) discovery  *(shipped)*

The "no same-origin signal" miss (§2) is not a classifier weakness — it is a
*probe-placement* one: WebGoat lives under `/WebGoat/`, so every discriminating
probe at the origin root 404s and the feature vector is all-zero. E9 fixes the
placement:

- **Discovery** — with `--discover-base-path`, a one-shot probe of generic mount
  points ([`BASE_PATH_CANDIDATES`](../../ferox-ml-core/src/profiles.rs): `app`,
  `api`, `admin`, `portal`, …, most-general first) finds the first that answers as
  a directory; the scan reroots there. An explicit `--base-path /WebGoat/` skips
  the probe and is the surest fix when the context path is already known.
- **Base-relative fingerprinting** — because the probe now runs under
  `…/WebGoat/`, its response URLs are rewritten origin-relative
  (`/WebGoat/api` → `/api`, `/WebGoat/` → `/`) before `feature_vector`, so
  `has("api")` and the `root_json` (empty-path) cue fire exactly as they would for
  a root-mounted app. Without this rewrite the rerooted probe would still look
  blank — the two halves only work together.

Deliberately **off by default** (a scan stays at the given target unless asked)
and **skipped in list mode**. Framework-specific mount names are intentionally
*not* baked into the candidate list — that would be lab-fitting; an unusual
context path is handled by the explicit `--base-path` override. Covered by
`base_path_discovery_reroots_and_fingerprints`,
`explicit_base_path_runs_under_it_without_probing`, and
`base_path_helpers_normalize_and_strip` in
[`orchestrator.rs`](../../ferox-ml-core/src/orchestrator.rs).

## Scheduler & persistence hardening (shipped)

Two engine-wide robustness upgrades landed alongside the fingerprint work:

- **Non-stationarity decay** in the Thompson scheduler
  ([`scheduler.rs`](../../ferox-ml-core/src/scheduler.rs)) — `--scheduler-decay d`
  (`(0,1]`, default `1.0` = classic stationary) discounts an arm's accumulated
  Beta evidence toward the uniform prior before each update, so a directory that
  was productive early but has gone quiet decays and the budget moves on. The
  per-round reward is already `hits ÷ requests`, i.e. cost-normalized, so cheap
  productive branches are preferred without a separate cost term.
- **Versioned model container** ([`orchestrator.rs`](../../ferox-ml-core/src/orchestrator.rs))
  — the persisted binary model now carries an explicit schema-version byte after
  its `FXM` magic (`FXM` + `1` is byte-identical to the old `FXM1` header, so v1
  files load unchanged). A file whose version doesn't match is ignored on load
  rather than fed to a positional bincode decode that could silently misread a
  changed DTO. Bump `MODEL_VERSION` on any non-additive DTO change. Guarded by
  `model_with_unsupported_schema_version_is_ignored`.

The **BM25 corpus build** was also made O(n) (a running token total replaces the
per-insert re-sum in [`ranking.rs`](../../ferox-ml-core/src/ranking.rs)), so warm
scans with large discovered corpora no longer pay a quadratic cost per hit.
