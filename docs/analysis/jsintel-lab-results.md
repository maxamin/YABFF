# jsintel lab results

Testing the ML layer against the live [jsintel](https://github.com/maxamin) lab
estate — OWASP Juice Shop, DVWA, WebGoat, WordPress, and Django (the
`web_default_labs` in jsintel's `tests/lab/labs_index.py`).

The fingerprint probe (`feroxbuster::ml::probe_paths()`) was replayed against each
live lab and the responses captured into
[`feroxbuster-ml/tests/fixtures/jsintel_labs.json`](../../feroxbuster-ml/tests/fixtures/jsintel_labs.json),
so the test
([`tests/lab_fingerprint.rs`](../../feroxbuster-ml/tests/lab_fingerprint.rs)) is
deterministic and offline. Re-capture with the probe script if the labs change.

## Fingerprint verdicts (captured 2026-10-01)

| Lab | Stack | Probe behavior | Fingerprint (margin 0.10) | Margin |
|---|---|---|---|---|
| WordPress | PHP / WP | catch-all 200, but keeps `wordpress_test_cookie`, `xmlrpc.php`→405, `x-powered-by: PHP` | **`WORDPRESS_CMS`** ✓ | 0.347 |
| WebGoat | Java | 404 on all probe paths (app lives under `/WebGoat/`) | `LEGACY_STATIC` | 1.554 |
| Django | Python | only `/` answers; rest 404 | `LEGACY_STATIC` | 1.554 |
| Juice Shop | Node/Angular SPA | **catch-all 200** for every path; `/api`,`/rest` → 500 | `ENTERPRISE_JAVA_SPRING` | 0.167 |
| DVWA | PHP | **catch-all 200** for every path | `WORDPRESS_CMS` | 0.158 |

## What this shows

- **It gets the clean signal right.** WordPress is identified correctly despite
  the server answering 200 to unrelated probe paths — the WP cookie, the
  `xmlrpc.php` 405, and the PHP header carry enough signal, and it clears the
  margin comfortably (0.347).
- **It degrades sanely on empty signal.** WebGoat and Django expose nothing on the
  probe paths, so with no dynamic markers they fall to `LEGACY_STATIC` by a wide,
  unambiguous margin (1.554) rather than guessing.
- **Catch-all servers are the real limitation — and the gate is why it's safe.**
  Juice Shop and DVWA return 200 for *every* path, so path-presence
  fingerprinting is driven by a weak margin (0.16–0.17) and lands on a
  plausible-but-wrong profile. This is exactly what `--ml-fp-margin` guards: at a
  strict margin (0.5) both are correctly demoted to the generic fallback instead
  of being trusted. The `lab_fingerprint.rs` test asserts this monotonic
  behavior. A follow-up is to feed the soft-404 signatures (`dedup`) into the
  fingerprint step so catch-alls are detected up front.
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
