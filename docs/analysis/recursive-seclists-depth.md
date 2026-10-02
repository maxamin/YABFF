# Full recursive SecLists at unlimited depth — benchmark

List mode (`--ml-loop --ml-list-dir <dir>`) loads a wordlist directory **recursively**
(the whole tree) and **re-applies it to every directory the scan discovers**, to the
recursion depth set by `--depth` (`0` = unlimited — feroxbuster's own convention).
This note benchmarks that against the real SecLists tree and a lab.

```bash
feroxbuster --ml-loop --ml-list-dir /usr/share/seclists/Discovery/Web-Content \
            --ml-list-max 0 --depth 0 --ml-algo dynsdt \
            --ml-model ./model.json -u http://TARGET
# --ml-list-max 0 = every entry in the tree · --depth 0 = unlimited recursion
```

## 1. Ingest — the loader reads the whole tree

`load_list_dir` walks `Discovery/Web-Content` recursively (**386 files across every
sub-folder** — `CMS/`, `api/`, `Programming-Language-Specific/`, …), trims, drops
comments/blank lines, and de-duplicates:

| metric | value |
|---|---|
| files read (recursive) | 386 |
| unique entries ingested (`--ml-list-max 0`) | **4,409,677** |

Every entry in the directory tree is usable; `--ml-list-max N` caps it when memory
or time is tight (the full load is ~4.4M strings in memory).

## 2. Unlimited depth is correct and terminates on a finite target

Against a finite directory tree, `--depth 0` recurses to the bottom and **stops**
— 404s never become directories, and `SeenPaths` + soft-404 filtering keep it from
looping. A recursive wordlist tree (files nested in sub-folders) at `--depth 0`:

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

## 3. Scale reality — full SecLists × unlimited depth is effectively unbounded

Re-applying 4.4M entries to **every** discovered directory, at unlimited depth, is a
very large scan by construction. Even a small slice shows it:

| run | list | depth | result |
|---|---|---|---|
| OWASP Juice Shop (real app) | 5,014 real SecLists entries (recursive) | `0` (unlimited) | **did not finish in 10 min** |
| full `Discovery/Web-Content` | 4,409,677 entries | `0` | 4.4M × every directory — impractical to complete |

The cost is `entries × directories-discovered × depth`, so the full tree at unlimited
depth is a **comprehensive/background** scan, not a quick one. Bound it in practice:

- `--depth N` (e.g. `4`) — the single biggest lever; depth is what makes it finite.
- `--ml-list-max N` — cap how much of the tree is loaded.
- `--scan-limit`, a request/time budget, or Ctrl-C — stop a long run.

Soft-404 filtering and per-path de-duplication guard against catch-all runaway, but
**depth is the primary bound** — `--depth 0` against a catch-all/soft-404 server that
answers every path will not self-terminate.

## How prediction fits in

Each discovered endpoint is fuzzed with the full (recursive) list; before the list
chunk, the model's top predictions for that endpoint are tried first, so a learned
or seeded model front-loads likely paths and the list covers the rest. On the finite
tree above every hit was reached this way (`predicted hits = 15`). Prediction changes
*order and reach-beyond-the-list*; the recursive list is what guarantees coverage.

## Takeaway

- **Capability:** the whole SecLists tree (4.4M entries) loads recursively and is
  re-applied to every directory to any depth — `--depth 0` for unlimited.
- **Safety:** unlimited depth terminates on finite sites; it's the catch-all case and
  sheer scale that need bounding.
- **Practice:** `--depth 4` (default) for routine runs; `--depth 0 --ml-list-max 0`
  for an exhaustive background sweep you bound by time/`--scan-limit`.
