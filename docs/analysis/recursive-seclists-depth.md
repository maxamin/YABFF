# Full recursive SecLists at unlimited depth — benchmark

List mode (`--ml-loop --ml-list-dir <dir>`) loads a wordlist directory **recursively**
(the whole tree) and **re-applies it to every directory the scan discovers**. It
**defaults to unlimited recursion depth** and loads **every entry** in the tree; pass
`--depth N` / `--ml-list-max N` to bound it. This note benchmarks that against the
real SecLists tree and a lab.

```bash
# list mode defaults: unlimited depth + every entry in the (recursive) tree
feroxbuster --ml-loop --ml-list-dir /usr/share/seclists/Discovery/Web-Content \
            --ml-algo dynsdt --ml-model ./model.json -u http://TARGET
# bound it: --depth 4 (or --depth 0 for explicit unlimited) · --ml-list-max 200000
```

## 1. Ingest — the loader reads the whole tree

`load_list_dir` walks `Discovery/Web-Content` recursively (**386 files across every
sub-folder** — `CMS/`, `api/`, `Programming-Language-Specific/`, …), trims, drops
comments/blank lines, and de-duplicates:

| metric | value |
|---|---|
| files read (recursive) | 386 |
| unique entries ingested (`--ml-list-max 0`) | **4,409,610** |

Every entry in the directory tree is usable; `--ml-list-max N` caps it when memory
or time is tight (the full load is ~4.4M strings in memory).

## 2. Unlimited depth (the default) is correct and terminates on a finite target

Against a finite directory tree, the default unlimited recursion recurses to the
bottom and **stops** — 404s never become directories, and `SeenPaths` + soft-404
filtering keep it from looping. A recursive wordlist tree (files nested in
sub-folders), list-mode default depth (unlimited), vs. an explicit `--depth 2`:

| run | deepest found | resources |
|---|---|---|
| list-mode default (unlimited) | depth 3 (tree bottom) | 15 |
| `--depth 2` (bounded) | depth 2 | 9 |

Unlimited reaches the tree's bottom; `--depth N` bounds it. Full run at the default: 

| metric | value |
|---|---|
| resources found | 15 |
| deepest discovery | depth 3 (the tree's bottom) |
| requests | 234 |
| rounds | 13 |
| wall-clock | 25 s |
| terminated cleanly | yes |

So "unlimited" does not mean "runaway" on a finite site: it means "as deep as the
site actually goes."

### 2b. The default on a real lab — OWASP Juice Shop

Same recursive list (14 entries across two sub-folders), one knob swept, against the
live Juice Shop app. The default (no `--depth`) behaves exactly as `--depth 0`:

| depth setting | resources | deepest | requests | rounds | wall-clock |
|---------------|:---:|:---:|:---:|:---:|:---:|
| **default (unlimited)** | **10** | **4** | 98 | 7 | 11 s |
| `--depth 1` | 5 | 1 | 14 | 1 | 7 s |
| `--depth 2` | 7 | 2 | 28 | 2 | 8 s |
| `--depth 0` (explicit) | 10 | 4 | 98 | 7 | 9 s |

The default and `--depth 0` are identical (10 resources, depth 4); bounding with
`--depth N` progressively limits reach (5 → 7 → 10) and cost. At the default it walks
Juice Shop's full chain to depth 4 with no flags:

```
/assets  /ftp  /main.js  /robots.txt  /video         (depth 1)
/assets/i18n  /assets/public                          (depth 2)
/assets/public/css  /assets/public/images             (depth 3)
/assets/public/images/products                        (depth 4)
```

## 3. Full SecLists × unlimited default on Juice Shop — the scale benchmark

Running the raw default — `--ml-list-dir /usr/share/seclists/Discovery/Web-Content`,
no `--depth`, no `--ml-list-max` — against the live Juice Shop, time-boxed to 8 min:

| metric | value |
|---|---|
| entries loaded (recursive) | **4,409,610** |
| wall-clock | 480 s (time-boxed) |
| requests issued | 50,000 (250 rounds × 200) |
| throughput | ~104 req/s (against Juice Shop) |
| **resources found** | **0** |
| directories expanded | 0 (still only the root arm) |

This run used the **old** pool order (sorted full path), and zero finds after 50k
requests was a **load-order** effect: the first files read were
`ActiveDirectory-small.txt`, `AdobeXML.fuzz.txt`, `BurpSuite-ParamMiner/…` (HTTP
*header* names, not paths), `CMS/Adobe-AEM…` — all low-signal for Juice Shop — while
the generic high-hit list (`common.txt`, with `/assets`, `/ftp`, `robots.txt`) sorted
far later and the cursor never reached it. Coverage, it turns out, is dominated by
**list order, not raw size**: a 14-entry *curated* list found Juice Shop's full
depth-4 chain in 11 s (§2b), but 50k alphabetical SecLists entries found nothing.

### 3b. Fix — the pool is now ranked by signal

`load_list_dir` no longer returns the tree in path order. It ranks every entry by
**document frequency** (how many of the wordlist files contain it), then shorter,
then lexicographic — so universally-common tokens surface first and one-off esoteric
entries sink. The top of the real `Discovery/Web-Content` tree (4.4M entries) is now:

```
README.md  start  contact  privacy  online  website  extra  index.php  media
direct  test  account  status  video  google  links  package.json  .htaccess
browser  lang  link  service  details  help  home  …
```

These are real generic paths that hit live apps immediately (Juice Shop itself serves
`/video`, `index.php`, …) — versus the old first entries (`ActiveDirectory`,
`BurpSuite` header names). Two consequences:

- The full-tree default now front-loads high-hit tokens, so it finds real paths in the
  first handful of requests instead of grinding through header-name lists.
- `--ml-list-max N` now keeps the **top-N by signal**, so a cap is a high-signal
  subset rather than an alphabetical accident.

Still bound a large run in practice:

- `--depth N` — the single biggest lever on total cost once directories are found.
- `--ml-list-max N` — cap to the top-N highest-signal entries.
- `--scan-limit`, a request/time budget, or Ctrl-C — stop a long run.

Progress is emitted to stderr (`[ml-loop] list pool loaded: N entries` and a
`rounds=/requests=/found=/arms=` line every 25 rounds) so a long run is observable.

Soft-404 filtering and per-path de-duplication guard against catch-all runaway, but
**depth is the primary bound** — unlimited depth against a catch-all/soft-404 server
that answers every path will not self-terminate.

## How prediction fits in

Each discovered endpoint is fuzzed with the full (recursive) list; before the list
chunk, the model's top predictions for that endpoint are tried first, so a learned
or seeded model front-loads likely paths and the list covers the rest. On the finite
tree above every hit was reached this way (`predicted hits = 15`). Prediction changes
*order and reach-beyond-the-list*; the recursive list is what guarantees coverage.

## Takeaway

- **Capability:** the whole SecLists tree (4.4M entries) loads recursively and is
  re-applied to every directory to any depth — **unlimited is the list-mode default**.
- **Safety:** unlimited depth terminates on finite sites; it's the catch-all case and
  sheer scale that need bounding.
- **Practice:** the default (unlimited depth, every entry) is an exhaustive background
  sweep — bound it with `--depth N` and/or `--ml-list-max N` (and `--scan-limit` /
  time) for routine runs.
