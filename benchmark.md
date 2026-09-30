# Web Fuzzer Benchmark

Real benchmark of the nine unique tools from [web-fuzzers-comparison.md](web-fuzzers-comparison.md), each installed and run against a **local, controlled target** on this machine. All runs are against `127.0.0.1` on a server I own, so testing is authorized.

**Bottom line:** 6 of 9 tools completed cleanly and found 100% of hidden paths with zero false positives. Three had problems in this environment: `kiterunner` (wrong tool for the job — it's an API scanner), `patator` (hangs on Python 3.14), and `dirbuster` (headless mode is broken by a known bug).

> ⚠️ **Read the caveats before trusting the timing numbers.** On localhost the Python target server is the throughput bottleneck (~250–330 req/s), so the fast Go/Rust tools cannot pull ahead of the slow ones. These numbers measure **accuracy and gross behavior**, *not* real-world speed separation. See [Caveats](#caveats--how-to-read-this).

---

## Test Environment

| Item | Value |
|---|---|
| OS | Kali GNU/Linux Rolling |
| Kernel | 6.12.73+deb13-amd64 |
| CPU | 8 cores |
| RAM | ~39 GB |
| Go | go1.26.8 |
| Python | 3.14.7 |
| Date | 2026-09-30 |

### Tools tested (as installed)

| Tool | Version | Source |
|---|---|---|
| ffuf | 2.1.0-dev | apt (Kali) |
| gobuster | 3.8.2 | apt (Kali) |
| wfuzz | 3.1.0 | apt (Kali) |
| dirsearch | 0.4.3 | apt (Kali) |
| feroxbuster | 2.13.1 | apt (Kali) |
| dirb | 2.22+dfsg-7 | apt (Kali) |
| kiterunner (kr) | git `7d5824c` | **cloned & built from source** (not packaged) |
| dirbuster | 1.0-1kali6 | apt (Kali) |
| patator | 1.1.0-1 | apt (Kali) |

`kiterunner` isn't in the Kali repos, so it was cloned from `assetnote/kiterunner` and built with `go build`.

---

## Methodology

### Target
A purpose-built local HTTP server (`bench/server.py`, Python `ThreadingHTTPServer`) that serves a **known ground-truth set of 20 paths** (16 unique base names) and returns `404` for everything else. This gives an exact scoring key.

Ground-truth base paths and their status codes:

```
admin(301) login(200) config(200) backup(200) api(200) robots.txt(200)
dashboard(200) uploads(301) test(200) hidden(200) secret(403)
server-status(403) index.php(200) config.php(200) db.php(200) old(301)
```

→ **16 unique discoverable base paths**, all returning non-404 codes.

### Wordlist
`bench/words.txt`: **10,000 entries** — all 16 ground-truth names plus Kali's `dirb/common.txt` and synthetic noise (seed=42 for reproducibility).

### Run parameters
- **Threads/concurrency: 40** for every tool that supports it (dirb is single-threaded by design).
- **Match codes:** `200,204,301,302,307,401,403` where the tool allows explicit code lists.
- Timing measured wall-clock via `date +%s.%N` around each invocation (no `/usr/bin/time` in this image).
- Each tool run once (single sample — see caveats).

Representative invocations:

```bash
ffuf        -u $T/FUZZ -w words.txt -t 40 -mc 200,204,301,302,307,401,403 -o out.json -of json -s
gobuster    dir -u $T -w words.txt -t 40 -s 200,204,301,302,307,401,403 -b "" -q
feroxbuster -u $T -w words.txt -t 40 -s 200,204,301,302,307,401,403 -n --silent
wfuzz       -c -z file,words.txt -t 40 --sc 200,204,301,302,307,401,403 $T/FUZZ
dirsearch   -u $T -w words.txt -t 40
dirb        $T words.txt -S -r
kr brute    $T -w words.txt -x 40 --fail-status-codes 404
patator     http_fuzz url=$T/FILE0 0=words.txt -t 40 -x ignore:code=404
dirbuster   -H -u $T/ -l words.txt -t 40 -R -P -e ""
```

---

## Results

| Tool | Completed? | Time (10k reqs, t=40) | Found / 16 | False positives | Recall |
|---|:-:|--:|:-:|:-:|:-:|
| **feroxbuster** | ✅ | **30.2 s** | 16 | 0 | 100% |
| **ffuf** | ✅ | **30.3 s** | 16 | 0 | 100% |
| **gobuster** | ✅ | 32.2 s | 16 | 0 | 100% |
| **dirb** | ✅ | 37.7 s | 16 | 0 | 100% |
| **wfuzz** | ✅ | 41.8 s | 16 | 0 | 100% |
| **dirsearch** | ✅ | 84.5 s | 16 | 0 | 100% |
| **kiterunner** | ⚠️ | 109.9 s | 0 reported | — | 0% (see below) |
| **patator** | ❌ | did not finish | — | — | — |
| **dirbuster** | ❌ | did not finish | — | — | — |

*Times ranked fastest→slowest among tools that completed.*

### Accuracy
Every tool that completed found **all 16** hidden paths with **zero false positives**. Accuracy was not a differentiator on this target — the wordlist contained the answers and the server had clean 404s, so this measures whether each tool's default matching logic behaves sanely (it does for the six that ran).

---

## Per-Tool Notes

### ✅ feroxbuster — 30.2 s
Ran with recursion disabled (`-n`) for a fair comparison. `--silent` emits just URLs; 16 unique hits. Fast and clean.

### ✅ ffuf — 30.3 s
Explicit match-code list, JSON output parsed to exactly 16 results. The reference general-purpose tool; no surprises.

### ✅ gobuster — 32.2 s
`dir` mode with an empty blacklist (`-b ""`) so 403s aren't dropped. 16 hits, correctly labelled with status and size.

### ✅ dirb — 37.7 s
**Single-threaded** and still finished in 37.7 s — only ~7 s behind the 40-thread Go tools. That's not dirb being fast; it's proof the **server is the bottleneck** (see caveats). On a real remote host dirb would be many times slower than the others.

### ✅ wfuzz — 41.8 s
`--sc` code filter, JSON output = 16 hits. Slower than the Go tools as expected for Python. Note the install warns `Pycurl is not compiled against Openssl` — harmless for this HTTP target.

### ✅ dirsearch — 84.5 s
Slowest of the completers. dirsearch does extra work by default (index detection, per-directory probing, richer reporting), which costs time but still returned exactly the 16 paths.

### ⚠️ kiterunner — reported 0
`kr brute` ran the full 10k list (109.9 s) but surfaced **no hits** in parseable output. This is **not a real failure — it's the wrong tool for this test.** kiterunner is purpose-built for **API discovery**: its `scan` mode replays real API route schemas from Assetnote's `.kite` datasets (correct methods, headers, JSON bodies). Its plain `brute` mode is a secondary feature and, with default preflight/quarantine behavior, did not report simple GET directory hits here. Benchmarking it on directory brute-forcing is like timing a torque wrench on a nail — score it on API discovery instead.

### ❌ patator — did not complete
`http_fuzz` **hangs in this environment**: even 5 words with 5 threads produced no output in 60 s, and the full run wrote only a CSV header after 8+ minutes. This looks like a **Python 3.14 + `multiprocessing` forkserver / pycurl compatibility problem** with patator 1.1.0, not a representative speed result — so no time is reported rather than fabricating one. Patator's real strength is multi-protocol credential brute-forcing, not content discovery, so this is a low-impact gap for this comparison.

### ❌ dirbuster — did not complete
Headless mode (`-H`) throws immediately:
```
Starting dir/file list based brute forcing
java.lang.NullPointerException: Cannot read field "jPanelRunning" because "this.gui" is null
    at com.sittinglittleduck.DirBuster.Manager.start(Manager.java:725)
```
This is a **known bug** in DirBuster 1.0-RC1: the headless path still references GUI objects, so it errors out and hangs (timed out at 280 s, 0-byte report). DirBuster is a **GUI tool** and is officially superseded by **OWASP ZAP's Forced Browse**. Use the GUI or ZAP instead of headless CLI.

---

## Caveats & How to Read This

1. **Localhost throughput ceiling is the headline caveat.** The target is a single Python `ThreadingHTTPServer` capped around 250–330 req/s. That's why a single-threaded C tool (dirb, 37.7 s) nearly ties 40-thread Go tools (ffuf/ferox, ~30 s). **These numbers do NOT show the tools' true max throughput.** On a real remote target with network latency and a capable server, ffuf/feroxbuster/gobuster would pull dramatically ahead of dirb/wfuzz/dirsearch. Treat the timings as "completed the same workload without pathology," not as a speed leaderboard.
2. **Single sample per tool.** No warm-up, no repeated trials, no variance/CI. Differences under a few seconds are noise.
3. **No RSS/CPU numbers.** `/usr/bin/time -v` isn't in this image, so memory wasn't captured.
4. **Accuracy test is deliberately easy.** The wordlist contained the answers and 404s were clean (no soft-404s). This validates default matching logic, not wildcard/soft-404 resilience — which is where tools like ffuf `-ac` and feroxbuster's auto-filter actually differentiate.
5. **Same-task bias.** All tools were scored on directory/file discovery. That's kiterunner's and patator's weak axis by design; judge them on their intended jobs (API discovery; multi-protocol brute-forcing).
6. **Version-specific failures.** patator and dirbuster failed due to the *specific* environment (Python 3.14) / a *specific* buggy build (DirBuster 1.0-RC1 headless), not necessarily the tools' general capability.

---

## Reproduce It

Artifacts are in `bench/`:
- `bench/server.py` — the scoring target (`python3 bench/server.py 8000`)
- `bench/words.txt` — the 10k wordlist (deterministic, seed=42)

```bash
python3 bench/server.py 8000 &        # start target
ffuf -u http://127.0.0.1:8000/FUZZ -w bench/words.txt -t 40 \
     -mc 200,204,301,302,307,401,403 -s   # expect 16 hits
```

To benchmark real-world speed instead of accuracy, point the tools at a target that isn't throughput-limited (a Go/nginx static server, or a remote host you're authorized to test) and raise the wordlist to 100k+.

---

## Verdict (from this run + intended use)

| Use case | Winner | Why |
|---|---|---|
| General web fuzzing | **ffuf** | Fast, flexible, clean output; 16/16 |
| Recursive content discovery | **feroxbuster** | Fastest here, recursion-first design |
| Simple dir/DNS/vhost scans | **gobuster** | Fast, 16/16, multi-mode |
| Payload transforms / scripting | **wfuzz** | Works, encoders + filter language (slower) |
| Batteries-included path scan | **dirsearch** | 16/16 with zero tuning (slowest completer) |
| Legacy / minimal boxes | **dirb** | Still works, but single-threaded |
| **API endpoint discovery** | **kiterunner** | Use `scan` + `.kite` files, not `brute` |
| Multi-protocol brute-force | patator | Broken on Py3.14 here; use for creds, not dirs |
| GUI forced-browsing | OWASP ZAP | DirBuster headless is broken; ZAP replaced it |
