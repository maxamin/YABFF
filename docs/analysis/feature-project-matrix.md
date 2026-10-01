# Web Fuzzer Feature × Project Matrix

A feature matrix across all nine fuzzers in this workspace. **Rows are projects, columns are features.** Ratings are derived from a full source read of each tool (see the per-project [API references](API-REFERENCES.md)).

> ✅ = supported · ⚠️ = partial / workaround / plugin / not a dedicated mode · ❌ = not supported
>
> **Authorized use only** — these tools are for systems you own or are permitted to test.

## Master matrix

| Project \ Feature | Dir/File | Recurse | Depth | DNS | Vhost | API | FUZZ-pos | Body | Header | Multi-WL | Encoders | Methods | Crawl | robots | DirList | Wildcard | F:Status | F:Size | F:Regex | F:Similar | Rate-lim | Auto-tune | Proxy | Replay | SOCKS | mTLS | Rand-UA | Resume | Interact | JSON-out | Ext-col | Bak-col | Word-col | Multi-proto | GUI | **Score** |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **feroxbuster** | ✅ | ✅ | ✅ | ❌ | ⚠️ | ❌ | ❌ | ⚠️ | ❌ | ⚠️ | ❌ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ❌ | ❌ | 26.5 |
| **ffuf** | ✅ | ✅ | ✅ | ❌ | ✅ | ❌ | ✅ | ✅ | ✅ | ✅ | ❌ | ✅ | ❌ | ❌ | ❌ | ✅ | ✅ | ✅ | ✅ | ⚠️ | ✅ | ❌ | ✅ | ✅ | ✅ | ✅ | ❌ | ❌ | ✅ | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ | 20.5 |
| **wfuzz** | ✅ | ✅ | ✅ | ❌ | ✅ | ❌ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ⚠️ | ⚠️ | ❌ | ⚠️ | ✅ | ✅ | ✅ | ❌ | ⚠️ | ❌ | ✅ | ❌ | ✅ | ⚠️ | ⚠️ | ❌ | ❌ | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ | 19.0 |
| **dirsearch** | ✅ | ✅ | ✅ | ❌ | ❌ | ❌ | ⚠️ | ⚠️ | ❌ | ⚠️ | ❌ | ✅ | ✅ | ❌ | ❌ | ✅ | ✅ | ✅ | ✅ | ⚠️ | ✅ | ❌ | ✅ | ❌ | ✅ | ✅ | ✅ | ✅ | ⚠️ | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ | 18.5 |
| **patator** | ✅ | ❌ | ❌ | ✅ | ⚠️ | ❌ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ❌ | ❌ | ❌ | ⚠️ | ✅ | ✅ | ✅ | ❌ | ✅ | ❌ | ⚠️ | ❌ | ⚠️ | ⚠️ | ❌ | ⚠️ | ⚠️ | ⚠️ | ❌ | ❌ | ❌ | ✅ | ❌ | 17.0 |
| **kiterunner** | ✅ | ⚠️ | ⚠️ | ❌ | ❌ | ✅ | ❌ | ✅ | ✅ | ❌ | ❌ | ✅ | ❌ | ❌ | ❌ | ✅ | ✅ | ⚠️ | ❌ | ❌ | ✅ | ❌ | ✅ | ❌ | ⚠️ | ❌ | ⚠️ | ❌ | ❌ | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ | 12.5 |
| **gobuster** | ✅ | ❌ | ❌ | ✅ | ✅ | ❌ | ✅ | ⚠️ | ⚠️ | ❌ | ❌ | ⚠️ | ❌ | ❌ | ❌ | ✅ | ✅ | ⚠️ | ⚠️ | ❌ | ✅ | ❌ | ✅ | ❌ | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | 11.5 |
| **dirbuster** | ✅ | ✅ | ⚠️ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ⚠️ | ✅ | ❌ | ❌ | ✅ | ⚠️ | ❌ | ❌ | ❌ | ⚠️ | ❌ | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ | ✅ | ⚠️ | ❌ | ❌ | ❌ | ❌ | ✅ | 9.5 |
| **dirb** | ✅ | ✅ | ⚠️ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ⚠️ | ⚠️ | ❌ | ❌ | ❌ | ⚠️ | ❌ | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | 5.0 |

**Legend:** ✅ = supported · ⚠️ = partial / workaround / plugin / not a dedicated mode · ❌ = not supported

## Score summary

| Project | ✅ full | ⚠️ partial | ❌ none | Weighted score /35 |
|---|---|---|---|---|
| feroxbuster | 25 | 3 | 7 | 26.5 |
| ffuf | 20 | 1 | 14 | 20.5 |
| wfuzz | 16 | 6 | 13 | 19.0 |
| dirsearch | 16 | 5 | 14 | 18.5 |
| patator | 13 | 8 | 14 | 17.0 |
| kiterunner | 10 | 5 | 20 | 12.5 |
| gobuster | 9 | 5 | 21 | 11.5 |
| dirbuster | 7 | 5 | 23 | 9.5 |
| dirb | 3 | 4 | 28 | 5.0 |

(Weighted score = ✅×1 + ⚠️×0.5, out of 35 features.)

## Feature definitions

- **Dir/File** — directory & file brute-force
- **Recurse** — recursive scanning into found dirs
- **Depth** — recursion depth control
- **DNS** — DNS subdomain brute-force
- **Vhost** — virtual-host discovery
- **API** — API route/schema awareness
- **FUZZ-pos** — fuzz arbitrary request position (keyword)
- **Body** — parameter / POST-body fuzzing
- **Header** — header fuzzing
- **Multi-WL** — multiple wordlists / combinations
- **Encoders** — payload encoders
- **Methods** — multiple HTTP methods
- **Crawl** — link extraction / crawling
- **robots** — robots.txt parsing
- **DirList** — directory-listing detection
- **Wildcard** — wildcard / soft-404 auto-filter
- **F:Status** — status-code filter
- **F:Size** — size/word/line filter
- **F:Regex** — regex filter
- **F:Similar** — response-similarity filter
- **Rate-lim** — request rate limiting
- **Auto-tune** — adaptive auto-tune / auto-bail
- **Proxy** — HTTP proxy
- **Replay** — replay proxy (matched only)
- **SOCKS** — SOCKS proxy
- **mTLS** — client certs / custom CA
- **Rand-UA** — random user-agent
- **Resume** — save / resume scan state
- **Interact** — interactive pause / menu
- **JSON-out** — structured output (JSON/XML/CSV/…)
- **Ext-col** — extension auto-collection
- **Bak-col** — backup-file auto-collection
- **Word-col** — dynamic word collection
- **Multi-proto** — protocols beyond HTTP
- **GUI** — graphical interface

## How to read it

- The matrix is wide (35 features); scroll horizontally, or use the **Score summary** for a quick capability ranking.
- **Score** is a rough capability breadth metric (✅×1 + ⚠️×0.5), **not** a quality ranking — a focused tool like `gobuster` scores lower but is excellent at what it does, and breadth ≠ speed or reliability (see [benchmark.md](benchmark.md)).
- `⚠️` covers very different situations per cell: a plugin (wfuzz link/robots plugins), a non-dedicated path (feroxbuster vhost via `-H Host`), or a limited form (gobuster length-only size filtering). Check the tool's API reference for specifics.

## Category highlights

- **Broadest HTTP fuzzers:** `ffuf` and `wfuzz` — arbitrary `FUZZ` position, body/header fuzzing, multiple wordlists, encoders (wfuzz).
- **Deepest content-discovery automation:** `feroxbuster` — recursion, crawling, robots.txt, dir-listing detection, auto-tune, extension/backup/word collection, resume.
- **Purpose-built niches:** `gobuster` (DNS/vhost/bucket modes), `kiterunner` (API route schemas), `patator` (multi-protocol brute-forcing), `dirbuster` (GUI).
- **Legacy/minimal:** `dirb` — dependable directory brute-force, little else.

## Related documents

- [web-fuzzers-comparison.md](web-fuzzers-comparison.md) — narrative feature comparison (features as rows).
- [benchmark.md](benchmark.md) — empirical speed/accuracy results.
- [API-REFERENCES.md](API-REFERENCES.md) — full per-item API reference for each tool.
