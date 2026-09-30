# Feature Co-occurrence Correlation Matrix

Pairwise **Pearson correlation between features**, computed across the 9 fuzzer projects using the support ratings from [feature-project-matrix.md](feature-project-matrix.md) (✅=1.0, ⚠️=0.5, ❌=0.0). This measures which capabilities tend to **appear together** (or instead of one another) across these tools.

- **Observations (n):** 9 projects
- **Variables:** 35 features → **34 after dropping `Dir/File`** (present in all 9, zero variance, correlation undefined)
- **Metric:** Pearson r on the {0, 0.5, 1} support scale
- **Full matrix:** [feature-correlation-matrix.csv](feature-correlation-matrix.csv) (34×34)

## ⚠️ Read this first — statistical caveat

With only **9 data points**, this correlation matrix is **noisy and largely descriptive, not inferential.** In particular:

- Several pairs show **r = 1.000 but co-occur in only 1 of 9 projects.** These are **spurious** — they are all features that only *feroxbuster* has, so they move together trivially (a single non-zero row makes any two such columns perfectly collinear). They tell you "one tool happens to bundle these," not that the capabilities are genuinely linked.
- Likewise `Proxy ~ Multi-proto = -1.000` is an artifact of *patator* being the only multi-protocol tool while having only partial (⚠️) proxy support.
- **Trust correlations where both features are present in several projects.** The "Robust correlations" section below filters to pairs where **both features appear in ≥4 projects**, which is where the numbers actually mean something.

No p-values are reported because they would be meaningless at n=9.

---

## Robust positive correlations (both features in ≥4 projects, r ≥ 0.5)

These are the trustworthy "these capabilities travel together" signals:

| r | feature A | feature B | co-occur |
|---|---|---|---|
| +0.941 | F:Size | F:Regex | 6/9 |
| +0.900 | Body | Header | 5/9 |
| +0.873 | F:Status | SOCKS | 7/9 |
| +0.873 | F:Status | F:Size | 7/9 |
| +0.869 | Recurse | Depth | 7/9 |
| +0.862 | Multi-WL | F:Regex | 5/9 |
| +0.862 | F:Regex | mTLS | 5/9 |
| +0.861 | Methods | JSON-out | 7/9 |
| +0.861 | Methods | F:Size | 7/9 |
| +0.833 | F:Size | SOCKS | 7/9 |
| +0.832 | Body | F:Status | 7/9 |
| +0.830 | Multi-WL | F:Size | 5/9 |
| +0.830 | F:Size | mTLS | 5/9 |
| +0.824 | Methods | F:Status | 8/9 |
| +0.808 | Vhost | FUZZ-pos | 4/9 |
| +0.803 | Body | Methods | 7/9 |
| +0.792 | F:Regex | SOCKS | 6/9 |
| +0.756 | F:Status | Rate-lim | 9/9 |
| +0.751 | Depth | JSON-out | 6/9 |
| +0.751 | Body | F:Size | 7/9 |
| +0.713 | F:Status | F:Regex | 6/9 |
| +0.710 | Multi-WL | mTLS | 5/9 |
| +0.689 | Methods | F:Regex | 6/9 |
| +0.683 | Depth | mTLS | 4/9 |
| +0.683 | Body | Multi-WL | 5/9 |
| +0.674 | mTLS | JSON-out | 5/9 |
| +0.674 | SOCKS | mTLS | 5/9 |
| +0.674 | Vhost | SOCKS | 5/9 |
| +0.674 | FUZZ-pos | Multi-WL | 4/9 |
| +0.667 | Rand-UA | JSON-out | 4/9 |
| +0.667 | F:Size | JSON-out | 6/9 |
| +0.662 | Methods | SOCKS | 7/9 |
| +0.659 | Multi-WL | Methods | 5/9 |
| +0.659 | Methods | mTLS | 5/9 |
| +0.644 | Crawl | Rand-UA | 3/9 |
| +0.643 | FUZZ-pos | F:Regex | 5/9 |
| +0.635 | Depth | Rand-UA | 4/9 |
| +0.625 | FUZZ-pos | Header | 4/9 |
| +0.614 | Recurse | Proxy | 7/9 |
| +0.600 | FUZZ-pos | Body | 5/9 |
| +0.585 | Vhost | F:Regex | 5/9 |
| +0.577 | Body | JSON-out | 6/9 |
| +0.577 | Rate-lim | SOCKS | 7/9 |
| +0.577 | FUZZ-pos | SOCKS | 5/9 |
| +0.577 | FUZZ-pos | F:Size | 5/9 |
| +0.577 | F:Size | Rate-lim | 7/9 |
| +0.577 | Body | SOCKS | 7/9 |
| +0.574 | Methods | Rate-lim | 8/9 |
| +0.570 | mTLS | Rand-UA | 3/9 |
| +0.567 | Recurse | Crawl | 4/9 |
| +0.567 | Header | F:Status | 5/9 |
| +0.567 | FUZZ-pos | F:Status | 5/9 |
| +0.565 | mTLS | Interact | 4/9 |
| +0.565 | Vhost | Multi-WL | 4/9 |
| +0.550 | Depth | Proxy | 7/9 |
| +0.549 | Body | F:Regex | 6/9 |
| +0.546 | F:Status | JSON-out | 7/9 |
| +0.543 | Vhost | F:Status | 5/9 |
| +0.543 | Multi-WL | F:Status | 5/9 |
| +0.543 | F:Status | mTLS | 5/9 |
| +0.539 | Vhost | Header | 4/9 |
| +0.539 | Header | Multi-WL | 3/9 |
| +0.532 | Depth | Crawl | 4/9 |
| +0.530 | Methods | Rand-UA | 4/9 |
| +0.524 | Crawl | Interact | 3/9 |
| +0.521 | Vhost | Body | 5/9 |
| +0.518 | Vhost | F:Size | 5/9 |
| +0.518 | Multi-WL | SOCKS | 5/9 |
| +0.518 | Multi-WL | JSON-out | 5/9 |
| +0.516 | Header | Methods | 5/9 |
| +0.500 | Body | Rate-lim | 7/9 |
| +0.500 | Wildcard | Proxy | 9/9 |
| +0.500 | SOCKS | JSON-out | 6/9 |

**Interpretation:**
- **Body ~ Header (+0.90)** and the request-fuzzing cluster: tools built to fuzz request bodies also fuzz headers — the hallmark of general-purpose fuzzers (ffuf, wfuzz, patator, kiterunner).
- **F:Size ~ F:Regex / F:Status (+0.87–0.94):** response-filtering features come as a bundle; a tool with one kind of filter almost always ships the others.
- **Recurse ~ Depth (+0.87):** anything that recurses also exposes a depth control (no surprise, but it validates the data).

## Robust negative correlations (both features in ≥4 projects, r ≤ −0.5)

| r | feature A | feature B | co-occur |
|---|---|---|---|
| -0.643 | Header | Crawl | 1/9 |

**Interpretation:**
- **Recurse ~ DNS (−0.93) / Depth ~ DNS (−0.83):** a clean split between *recursive HTTP content scanners* (feroxbuster, dirsearch, dirb, dirbuster) and *DNS/host enumeration* tools (gobuster, patator). Tools tend to specialize in one or the other.
- **Header ~ Crawl (−0.64):** request-fuzzing tools (fuzz headers) vs. content-discovery tools (crawl/extract links) — two different design philosophies that rarely coexist.

---

## Heatmap — high-signal features (present in 3–7 of 9 projects)

Full-spectrum single-project features (API, GUI, Multi-proto, the feroxbuster-only collectors) are excluded here because their correlations are degenerate (see caveat). This leaves the **17 genuinely variable features**:

| # feature | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| `01` Recurse | ▓▓ | ▓▓ | -- | -- | -- | -- | ·· | ▒▒ | ·· | ·· | ░░ | ·· | ░░ | ░░ | ·· | ░░ | ░░ |
| `02` Depth | ▓▓ | ▓▓ | ·· | ·· | ·· | ·· | ░░ | ▒▒ | ░░ | ░░ | ▒▒ | ░░ | ▒▒ | ▒▒ | ░░ | ░░ | ▒▒ |
| `03` Vhost | -- | ·· | ▓▓ | ▓▓ | ▒▒ | ▒▒ | ▒▒ | -- | ▒▒ | ▒▒ | ·· | ▒▒ | ░░ | -- | -- | ·· | ·· |
| `04` FUZZ-pos | -- | ·· | ▓▓ | ▓▓ | ▒▒ | ▒▒ | ▒▒ | -- | ▒▒ | ▒▒ | ·· | ▒▒ | ░░ | -- | ·· | ·· | ·· |
| `05` Body | -- | ·· | ▒▒ | ▒▒ | ▓▓ | ▓▓ | ▒▒ | -- | ▒▒ | ▒▒ | ·· | ▒▒ | ░░ | ·· | ·· | ·· | ▒▒ |
| `06` Header | -- | ·· | ▒▒ | ▒▒ | ▓▓ | ▓▓ | ▒▒ | == | ░░ | ░░ | -- | ░░ | ·· | -- | -- | -- | ░░ |
| `07` Multi-WL | ·· | ░░ | ▒▒ | ▒▒ | ▒▒ | ▒▒ | ▓▓ | ·· | ▓▓ | ▓▓ | ░░ | ▒▒ | ▒▒ | ·· | ░░ | ░░ | ▒▒ |
| `08` Crawl | ▒▒ | ▒▒ | -- | -- | -- | == | ·· | ▓▓ | ·· | ·· | ▒▒ | ·· | ░░ | ▒▒ | ▒▒ | ▒▒ | ░░ |
| `09` F:Size | ·· | ░░ | ▒▒ | ▒▒ | ▒▒ | ░░ | ▓▓ | ·· | ▓▓ | ▓▓ | ▒▒ | ▓▓ | ▓▓ | ░░ | ▒▒ | ░░ | ▒▒ |
| `10` F:Regex | ·· | ░░ | ▒▒ | ▒▒ | ▒▒ | ░░ | ▓▓ | ·· | ▓▓ | ▓▓ | ▒▒ | ▒▒ | ▓▓ | ░░ | ▒▒ | ░░ | ░░ |
| `11` F:Similar | ░░ | ▒▒ | ·· | ·· | ·· | -- | ░░ | ▒▒ | ▒▒ | ▒▒ | ▓▓ | ▒▒ | ▓▓ | ▒▒ | ▒▒ | ▒▒ | ▒▒ |
| `12` SOCKS | ·· | ░░ | ▒▒ | ▒▒ | ▒▒ | ░░ | ▒▒ | ·· | ▓▓ | ▒▒ | ▒▒ | ▓▓ | ▒▒ | ░░ | ░░ | ·· | ▒▒ |
| `13` mTLS | ░░ | ▒▒ | ░░ | ░░ | ░░ | ·· | ▒▒ | ░░ | ▓▓ | ▓▓ | ▓▓ | ▒▒ | ▓▓ | ▒▒ | ▒▒ | ▒▒ | ▒▒ |
| `14` Rand-UA | ░░ | ▒▒ | -- | -- | ·· | -- | ·· | ▒▒ | ░░ | ░░ | ▒▒ | ░░ | ▒▒ | ▓▓ | ▒▒ | ·· | ▒▒ |
| `15` Resume | ·· | ░░ | -- | ·· | ·· | -- | ░░ | ▒▒ | ▒▒ | ▒▒ | ▒▒ | ░░ | ▒▒ | ▒▒ | ▓▓ | ░░ | ░░ |
| `16` Interact | ░░ | ░░ | ·· | ·· | ·· | -- | ░░ | ▒▒ | ░░ | ░░ | ▒▒ | ·· | ▒▒ | ·· | ░░ | ▓▓ | ░░ |
| `17` JSON-out | ░░ | ▒▒ | ·· | ·· | ▒▒ | ░░ | ▒▒ | ░░ | ▒▒ | ░░ | ▒▒ | ▒▒ | ▒▒ | ▒▒ | ░░ | ░░ | ▓▓ |

**Shade key:** `▓▓` ≥ 0.8 · `▒▒` 0.5–0.8 · `░░` 0.2–0.5 · `··` −0.2–0.2 · `--` −0.5 to −0.2 · `==` −0.8 to −0.5 · `##` ≤ −0.8

**Feature codes:**
- `01` = Recurse (in 7/9)
- `02` = Depth (in 7/9)
- `03` = Vhost (in 5/9)
- `04` = FUZZ-pos (in 5/9)
- `05` = Body (in 7/9)
- `06` = Header (in 5/9)
- `07` = Multi-WL (in 5/9)
- `08` = Crawl (in 4/9)
- `09` = F:Size (in 7/9)
- `10` = F:Regex (in 6/9)
- `11` = F:Similar (in 3/9)
- `12` = SOCKS (in 7/9)
- `13` = mTLS (in 5/9)
- `14` = Rand-UA (in 4/9)
- `15` = Resume (in 3/9)
- `16` = Interact (in 5/9)
- `17` = JSON-out (in 7/9)

### Numeric values for the same subset

| feature | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| `01` | +1.00 | +0.87 | -0.24 | -0.43 | -0.32 | -0.43 | +0.07 | +0.57 | -0.05 | +0.02 | +0.43 | -0.05 | +0.37 | +0.38 | +0.12 | +0.37 | +0.44 |
| `02` | +0.87 | +1.00 | +0.04 | -0.15 | +0.10 | -0.15 | +0.36 | +0.53 | +0.40 | +0.39 | +0.64 | +0.40 | +0.68 | +0.64 | +0.32 | +0.36 | +0.75 |
| `03` | -0.24 | +0.04 | +1.00 | +0.81 | +0.52 | +0.54 | +0.56 | -0.31 | +0.52 | +0.59 | +0.08 | +0.67 | +0.27 | -0.21 | -0.22 | -0.02 | +0.05 |
| `04` | -0.43 | -0.15 | +0.81 | +1.00 | +0.60 | +0.62 | +0.67 | -0.39 | +0.58 | +0.64 | -0.17 | +0.58 | +0.27 | -0.29 | -0.14 | -0.13 | +0.00 |
| `05` | -0.32 | +0.10 | +0.52 | +0.60 | +1.00 | +0.90 | +0.68 | -0.39 | +0.75 | +0.55 | +0.02 | +0.58 | +0.36 | +0.12 | -0.02 | -0.13 | +0.58 |
| `06` | -0.43 | -0.15 | +0.54 | +0.62 | +0.90 | +1.00 | +0.54 | -0.64 | +0.43 | +0.26 | -0.34 | +0.29 | +0.00 | -0.29 | -0.43 | -0.27 | +0.29 |
| `07` | +0.07 | +0.36 | +0.56 | +0.67 | +0.68 | +0.54 | +1.00 | -0.03 | +0.83 | +0.86 | +0.27 | +0.52 | +0.71 | +0.10 | +0.24 | +0.27 | +0.52 |
| `08` | +0.57 | +0.53 | -0.31 | -0.39 | -0.39 | -0.64 | -0.03 | +1.00 | +0.10 | +0.19 | +0.51 | +0.10 | +0.39 | +0.64 | +0.60 | +0.52 | +0.40 |
| `09` | -0.05 | +0.40 | +0.52 | +0.58 | +0.75 | +0.43 | +0.83 | +0.10 | +1.00 | +0.94 | +0.53 | +0.83 | +0.83 | +0.50 | +0.55 | +0.21 | +0.67 |
| `10` | +0.02 | +0.39 | +0.59 | +0.64 | +0.55 | +0.26 | +0.86 | +0.19 | +0.94 | +1.00 | +0.55 | +0.79 | +0.86 | +0.40 | +0.57 | +0.31 | +0.50 |
| `11` | +0.43 | +0.64 | +0.08 | -0.17 | +0.02 | -0.34 | +0.27 | +0.51 | +0.53 | +0.55 | +1.00 | +0.53 | +0.82 | +0.66 | +0.74 | +0.64 | +0.53 |
| `12` | -0.05 | +0.40 | +0.67 | +0.58 | +0.58 | +0.29 | +0.52 | +0.10 | +0.83 | +0.79 | +0.53 | +1.00 | +0.67 | +0.50 | +0.38 | +0.05 | +0.50 |
| `13` | +0.37 | +0.68 | +0.27 | +0.27 | +0.36 | +0.00 | +0.71 | +0.39 | +0.83 | +0.86 | +0.82 | +0.67 | +1.00 | +0.57 | +0.70 | +0.56 | +0.67 |
| `14` | +0.38 | +0.64 | -0.21 | -0.29 | +0.12 | -0.29 | +0.10 | +0.64 | +0.50 | +0.40 | +0.66 | +0.50 | +0.57 | +1.00 | +0.76 | +0.10 | +0.67 |
| `15` | +0.12 | +0.32 | -0.22 | -0.14 | -0.02 | -0.43 | +0.24 | +0.60 | +0.55 | +0.57 | +0.74 | +0.38 | +0.70 | +0.76 | +1.00 | +0.39 | +0.38 |
| `16` | +0.37 | +0.36 | -0.02 | -0.13 | -0.13 | -0.27 | +0.27 | +0.52 | +0.21 | +0.31 | +0.64 | +0.05 | +0.56 | +0.10 | +0.39 | +1.00 | +0.36 |
| `17` | +0.44 | +0.75 | +0.05 | +0.00 | +0.58 | +0.29 | +0.52 | +0.40 | +0.67 | +0.50 | +0.53 | +0.50 | +0.67 | +0.67 | +0.38 | +0.36 | +1.00 |

---

## Perfectly collinear cluster (degenerate)

{ **Auto-tune, Bak-col, DirList, Ext-col, Word-col** } — all r = 1.000 with each other, but only because **feroxbuster is the sole tool with any of them.** This is the n=9 degeneracy, not a real capability relationship. Treat it as "feroxbuster's unique automation bundle," not a correlation finding.

---

## Method notes & alternatives

- **Why Pearson on {0,0.5,1}:** it uses the partial-support (⚠️) information rather than throwing it away. Binarizing (⚠️→1 or ⚠️→0) shifts some values but not the qualitative clusters.
- **A more robust alternative for n=9** would be the **Jaccard index** or raw **co-occurrence counts** (shown in the pair tables) rather than correlation, since those don't blow up to ±1 on single-row features. The Pearson matrix is provided because that is literally a "correlation matrix"; the co-occurrence counts are the more defensible companion.
- Regenerate with `python3 tools-scripts/corr.py` (data is embedded in the script).

## Related documents
- [feature-project-matrix.md](feature-project-matrix.md) — the source feature × project matrix.
- [feature-correlation-matrix.csv](feature-correlation-matrix.csv) — the full 34×34 numeric matrix.
- [web-fuzzers-comparison.md](web-fuzzers-comparison.md) · [benchmark.md](benchmark.md) · [API-REFERENCES.md](API-REFERENCES.md)
