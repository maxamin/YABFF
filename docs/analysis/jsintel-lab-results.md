# jsintel lab results

Testing the ML layer against the live [jsintel](https://github.com/maxamin) lab
estate — OWASP Juice Shop, DVWA, WebGoat, WordPress, and Django (the
`web_default_labs` in jsintel's `tests/lab/labs_index.py`).

The fingerprint probe (`feroxbuster::ml::probe_paths()`) was replayed against each
live lab and the responses captured into
[`feroxbuster-ml/tests/fixtures/jsintel_labs.json`](../../feroxbuster-ml/tests/fixtures/jsintel_labs.json)
(and `jsintel_labs_full.json`, which adds response sizes + soft-404 baselines), so
the tests are deterministic and offline. Re-capture with
[`scripts/probe_jsintel_labs.py`](../../scripts/probe_jsintel_labs.py) if the labs
change.

Two tests run against these captures:
[`tests/lab_fingerprint.rs`](../../feroxbuster-ml/tests/lab_fingerprint.rs)
(fingerprint + gate verdicts) and
[`tests/lab_features.rs`](../../feroxbuster-ml/tests/lab_features.rs), which drives
**every** engine function against all five labs — feature vector, weighted
classify (E3), calibrated confidence (E5), catch-all guard (E1), per-path soft-404
scoring (E2), the gate, online re-fingerprinting (E6), K-Means refresh (E7), Markov
seed/predict/subword back-off (E8), BM25, and the bandit.

## Fingerprint verdicts (captured 2026-10-01)

Verdicts are the **current** engine (7 profiles, E3-weighted metric, E4 profiles);
the parenthetical shows the original 4-profile / unweighted result for contrast.

| Lab | Stack | Probe behavior | Fingerprint (raw) | Gated (margin 0.10) |
|---|---|---|---|---|
| WordPress | PHP / WP | catch-all 200, but keeps `wordpress_test_cookie`, `xmlrpc.php`→405, `x-powered-by: PHP` | **`WORDPRESS_CMS`** ✓ | `WORDPRESS_CMS` (kept — strong discriminator) |
| Django | Python | only `/` answers, but it sets a `csrftoken` cookie | **`DJANGO`** ✓ (was `LEGACY_STATIC`) | `DJANGO` |
| WebGoat | Java | 404 on all probe paths (app lives under `/WebGoat/`) | `LEGACY_STATIC` | `LEGACY_STATIC` (no signal) |
| Juice Shop | Node/Angular SPA | **catch-all 200** for every path; `/api`,`/rest` → 500 | `REST_API` (was `ENTERPRISE_JAVA_SPRING`) | **abstained** (catch-all) |
| DVWA | PHP | **catch-all 200** for every path | `WORDPRESS_CMS` | **abstained** (catch-all) |

## What this shows

- **It gets the signal-bearing labs right.** WordPress (WP cookie + `xmlrpc.php`
  405 + PHP header) and Django (its root `csrftoken` cookie, keyed by the E4
  `DJANGO` profile) are both identified correctly despite answering 200/404 noise
  on unrelated paths.
- **It degrades sanely on empty signal.** WebGoat exposes nothing on the probe
  paths (its app is under `/WebGoat/`), so with no markers it falls to
  `LEGACY_STATIC` by a clear margin rather than guessing.
- **Catch-all servers are the real limitation — and E1/E2 are why it's safe.**
  Juice Shop and DVWA return 200 for *every* path, so path-presence
  fingerprinting is unreliable (Juice Shop's raw `REST_API` even happens to be
  right, DVWA's raw `WORDPRESS_CMS` is wrong). The shipped catch-all guard (E1)
  and per-path soft-404 scoring (E2) **abstain** both in gated mode rather than
  trust them, and `lab_fingerprint.rs` asserts it.
- **Online learning works on real crawled paths.** `tests/lab_fingerprint.rs`
  also replays the real same-origin paths jsintel crawled from the Juice Shop SPA
  (`output_lab_run/assets/crawled_urls.txt`) through `observe()` and confirms the
  model then predicts the learned `/static` and `/static/jquery` children.

## Lift expectations

The jsintel labs are SPA / catch-all / sparse targets, where per-directory
prediction has little conventional structure to exploit — so ML lift here is
modest, unlike the nested-REST scenario in
[`ml-ab-benchmark.md`](ml-ab-benchmark.md) (19 vs 11 resources). Both results
together are the honest picture: the ML layer pays off on structured, convention-
following APIs and adds little on hashed-asset SPAs or servers that answer
everything.
