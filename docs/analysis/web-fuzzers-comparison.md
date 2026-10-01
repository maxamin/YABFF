# Web Fuzzer Comparison

A side-by-side look at popular tools for web content discovery and fuzzing: what each one is good at, where it falls short, and when to pick it.

> **Note:** wfuzz was listed twice in the original list (#3 and #9), so this covers **9 unique tools**.
>
> **Authorized use only:** Run these tools only against systems you own or have written permission to test (pentest engagements, bug bounty programs within scope, CTFs, lab environments).

---

## Quick Comparison

| Tool | Language | License | Speed | Recursion | Fuzz anywhere in request | Filtering / matching | Interface | Maintenance |
|---|---|---|---|---|---|---|---|---|
| **ffuf** | Go | MIT | Very fast | Yes (opt-in) | Yes (`FUZZ` keyword) | Excellent (status, size, words, lines, regex, time, auto-calibrate) | CLI | Active |
| **gobuster** | Go | Apache-2.0 | Very fast | No | Limited (`fuzz` mode) | Good (status, length) | CLI | Active |
| **wfuzz** | Python | GPL-2.0 | Moderate | Yes (`-R`) | Yes (`FUZZ`, `FUZ2Z`…) | Excellent (filter expression language) | CLI | Low activity |
| **dirsearch** | Python | GPL-2.0 | Moderate–fast | Yes | No (paths only) | Good (status, size, text, regex) | CLI | Active |
| **feroxbuster** | Rust | MIT | Very fast | Yes (default) | No (paths/params) | Very good (+ automatic wildcard filtering) | CLI + interactive menu | Active |
| **dirb** | C | GPL-2.0 | Slow (single-threaded) | Yes (default) | No | Basic | CLI | Unmaintained |
| **kiterunner** | Go | AGPL-3.0 | Fast | No | API routes (methods, params, headers) | Good | CLI | Low activity |
| **DirBuster** | Java | Apache-2.0 | Moderate | Yes | No | Basic | GUI (plus headless mode) | Inactive (replaced by ZAP) |
| **patator** | Python | GPL-2.0 | Moderate | No | Yes (`FILE0`, `NET0`… placeholders) | Very flexible (`-x` action conditions) | CLI | Low activity |

---

## Feature Matrix

| Capability | ffuf | gobuster | wfuzz | dirsearch | feroxbuster | dirb | kiterunner | DirBuster | patator |
|---|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
| Directory/file brute-force | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Recursive scanning | ✅ | ❌ | ✅ | ✅ | ✅ | ✅ | ❌ | ✅ | ❌ |
| Extension appending | ✅ | ✅ | ✅* | ✅ | ✅ | ✅ | ❌ | ✅ | ✅* |
| Virtual host fuzzing | ✅ | ✅ | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ | ✅ |
| DNS subdomain brute-force | ❌ | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ✅ |
| Parameter / POST body fuzzing | ✅ | ⚠️ | ✅ | ❌ | ⚠️ | ❌ | ✅ | ❌ | ✅ |
| Header fuzzing | ✅ | ⚠️ | ✅ | ❌ | ❌ | ❌ | ✅ | ❌ | ✅ |
| Multiple wordlists / combo modes | ✅ | ❌ | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ | ✅ |
| Payload encoders | ❌ | ❌ | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ | ✅ |
| API-aware (method + route schemas) | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ✅ | ❌ | ❌ |
| Wildcard / soft-404 handling | ✅ (`-ac`) | ✅ | ⚠️ manual | ✅ | ✅ auto | ⚠️ | ✅ | ⚠️ | ⚠️ manual |
| Link extraction from responses | ❌ | ❌ | ❌ | ❌ | ✅ | ❌ | ❌ | ✅ | ❌ |
| Pause / resume scan | ⚠️ interactive | ❌ | ❌ | ✅ | ✅ | ✅ | ❌ | ✅ | ✅ |
| Proxy support (e.g. Burp) | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Structured output (JSON, etc.) | ✅ | ⚠️ | ✅ | ✅ | ✅ | ❌ | ✅ | ⚠️ | ⚠️ |

✅ supported · ⚠️ partial/workaround · ❌ not supported · \* via payload tricks or combined wordlists rather than a dedicated flag

---

## Tool Profiles

### 1. ffuf (Fuzz Faster U Fool)
**GitHub:** [ffuf/ffuf](https://github.com/ffuf/ffuf) · **Language:** Go

The general-purpose fuzzer most people reach for today. Put the `FUZZ` keyword anywhere in the request (URL, headers, body, method) and ffuf substitutes wordlist entries at high concurrency.

**Strengths**
- Very fast, single static binary
- Fuzz any part of a request; supports multiple wordlists in `clusterbomb`, `pitchfork` and `sniper` modes
- Rich matchers and filters (`-mc/-fc`, `-ms/-fs`, `-mw/-fw`, `-ml/-fl`, `-mr/-fr`, time)
- Auto-calibration (`-ac`) removes most soft-404 noise
- Can replay raw Burp requests (`-request`)
- Output as JSON, CSV, HTML or Markdown

**Weaknesses**
- Recursion exists but is basic compared with feroxbuster
- No built-in payload encoders (use pre-processed wordlists)

```bash
ffuf -u https://target.example/FUZZ -w words.txt -mc 200,204,301,302,307,401,403 -ac
ffuf -u https://target.example/ -H "Host: FUZZ.target.example" -w vhosts.txt -fs 4242
ffuf -u https://target.example/login -X POST -d "user=admin&pass=FUZZ" -w passwords.txt -fc 401
```

---

### 2. gobuster
**GitHub:** [OJ/gobuster](https://github.com/OJ/gobuster) · **Language:** Go

A fast, focused brute-forcer organized into modes: `dir`, `dns`, `vhost`, `fuzz`, `s3`, `gcs` and `tftp`.

**Strengths**
- Very fast and simple to use
- One tool covers directories, DNS subdomains, virtual hosts and cloud buckets
- Good wildcard detection in `dir` and `dns` modes

**Weaknesses**
- No recursion
- Less flexible filtering and request manipulation than ffuf or wfuzz

```bash
gobuster dir   -u https://target.example -w words.txt -x php,html,txt -t 50
gobuster dns   -d target.example -w subdomains.txt
gobuster vhost -u https://target.example -w vhosts.txt --append-domain
```

---

### 3. wfuzz
**GitHub:** [xmendez/wfuzz](https://github.com/xmendez/wfuzz) · **Language:** Python

A veteran, highly modular fuzzer. It predates ffuf, and ffuf copied its `FUZZ` keyword approach.

**Strengths**
- Modular payloads (files, ranges, lists, stdin, Burp logs, etc.)
- Encoders (`urlencode`, `base64`, `md5`, …) and iterators (`zip`, `product`, `chain`)
- Powerful filter language (`--filter "c=200 and l>10"`) plus `--hc/--hl/--hw/--hh`
- Plugin scripts that parse responses (links, robots.txt, etc.)
- Python library API for scripting

**Weaknesses**
- Noticeably slower than the Go/Rust tools
- Development has slowed (last major release was 3.1.0 in 2020); pycurl dependency can be painful to install

```bash
wfuzz -c -z file,words.txt --hc 404 https://target.example/FUZZ
wfuzz -c -z file,users.txt -z file,pass.txt --hc 401 -d "u=FUZZ&p=FUZ2Z" https://target.example/login
wfuzz -c -z file,params.txt,urlencode --hh 1234 "https://target.example/page?FUZZ=test"
```

---

### 4. dirsearch
**GitHub:** [maurosoria/dirsearch](https://github.com/maurosoria/dirsearch) · **Language:** Python

A mature path scanner with sensible defaults and a good bundled wordlist.

**Strengths**
- Works well out of the box with its built-in wordlist and `%EXT%` substitution
- Recursion with depth control and exclusions
- Many report formats (plain, JSON, XML, Markdown, CSV, HTML, SQLite, …)
- Pause/resume sessions, multiple targets, proxy rotation

**Weaknesses**
- Path discovery only: no arbitrary request fuzzing
- Slower than ffuf, gobuster or feroxbuster at high concurrency

```bash
dirsearch -u https://target.example -e php,html,js
dirsearch -u https://target.example -r --max-recursion-depth 3 -x 404,500
dirsearch -l targets.txt -o report.json --format json
```

---

### 5. feroxbuster
**GitHub:** [epi052/feroxbuster](https://github.com/epi052/feroxbuster) · **Language:** Rust

A fast content-discovery tool built around recursion.

**Strengths**
- Very fast, with recursion enabled by default
- Automatic wildcard/soft-404 filtering
- Extracts links from responses to find more content
- Interactive "Scan Management Menu" to add or cancel scans mid-run
- Resumable state files, smart backup-file collection, JSON output

**Weaknesses**
- Focused on paths: not a general-purpose request fuzzer like ffuf or wfuzz
- Aggressive recursion can generate lots of traffic if not tuned (`--depth`, `--dont-scan`)

```bash
feroxbuster -u https://target.example -w words.txt -x php,txt
feroxbuster -u https://target.example --depth 2 --filter-status 404 --json -o out.json
```

---

### 6. dirb
**Source:** Kali Linux / [SourceForge](https://sourceforge.net/projects/dirb/) · **Language:** C

The classic dictionary-based scanner, preinstalled on Kali and commonly used in older courses and CTF write-ups.

**Strengths**
- Zero configuration, recursive by default
- Ships with useful small wordlists (`/usr/share/dirb/wordlists/common.txt`)

**Weaknesses**
- Single-threaded, so very slow by modern standards
- Minimal filtering and no structured output
- Unmaintained

```bash
dirb https://target.example /usr/share/dirb/wordlists/common.txt
dirb https://target.example -X .php,.bak -N 404
```

---

### 7. kiterunner (kr)
**GitHub:** [assetnote/kiterunner](https://github.com/assetnote/kiterunner) · **Language:** Go

Built for API discovery. Instead of just requesting `GET /word`, it replays real API routes from Assetnote's compiled Swagger/OpenAPI datasets (`.kite` files), using the correct HTTP methods, parameters and headers.

**Strengths**
- Finds API endpoints that plain directory brute-forcers miss (e.g. routes that only answer `POST` with a JSON body)
- Also has a traditional `brute` mode for normal wordlists
- Can replay a finding for inspection (`kr kb replay`)

**Weaknesses**
- Narrow focus on APIs
- Development has been quiet since around 2021
- AGPL-3.0 license

```bash
kr scan  https://api.target.example -w routes-large.kite
kr brute https://target.example -w words.txt -x 10
```

---

### 8. DirBuster
**Source:** OWASP / Kali Linux · **Language:** Java

OWASP's original multi-threaded GUI brute-forcer. Its main legacy is the `directory-list-2.3-*.txt` wordlists that other tools still use.

**Strengths**
- GUI with a live tree view of discovered content
- Recursive, multi-threaded, parses HTML for links
- Headless mode available

**Weaknesses**
- The OWASP project is inactive; its functionality was folded into OWASP ZAP's *Forced Browse* add-on
- Java GUI is heavy, with limited filtering and automation

```bash
dirbuster            # launch GUI
dirbuster -H -u https://target.example -l /usr/share/wordlists/dirbuster/directory-list-2.3-medium.txt
```

---

### 9. patator
**GitHub:** [lanjelot/patator](https://github.com/lanjelot/patator) · **Language:** Python

A multi-purpose brute-forcer. Web fuzzing (`http_fuzz`) is one of many modules; others cover SSH, FTP, SMTP, SMB, databases, DNS, etc.

**Strengths**
- Very flexible: placeholders (`FILE0`, `RANGE1`, `NET0`) in any field, combined with conditions and actions such as `-x ignore:code=404` or `-x quit:fgrep=Welcome`
- One tool for web plus many network protocols
- Good for credential attacks with rate control and retries

**Weaknesses**
- Harder to learn, verbose syntax
- Slower than dedicated Go/Rust web fuzzers
- Not specialized for content discovery (no recursion, no soft-404 heuristics)

```bash
patator http_fuzz url=https://target.example/FILE0 0=words.txt -x ignore:code=404
patator http_fuzz url=https://target.example/login method=POST body='user=admin&pass=FILE0' 0=passwords.txt -x ignore:fgrep='Invalid'
```

---

## Which One Should I Use?

| Goal | Best pick | Alternatives |
|---|---|---|
| General-purpose web fuzzing (any request part) | **ffuf** | wfuzz |
| Deep recursive content discovery | **feroxbuster** | dirsearch, ffuf `-recursion` |
| Quick directory scan with good defaults | **dirsearch** | gobuster, feroxbuster |
| DNS subdomain brute-force | **gobuster** (`dns`) | patator (`dnsdict`) |
| Virtual host discovery | **ffuf** (`Host: FUZZ`) | gobuster (`vhost`) |
| API endpoint discovery | **kiterunner** | ffuf with API wordlists |
| Complex payload transforms / encoders | **wfuzz** | patator |
| Web + other protocol brute-forcing | **patator** | — |
| GUI-based discovery | **OWASP ZAP** (successor to DirBuster) | DirBuster |
| Legacy / minimal environment | **dirb** | — |

### Suggested modern toolkit
1. **ffuf** for flexible fuzzing of anything
2. **feroxbuster** for recursive content discovery
3. **kiterunner** for APIs
4. **gobuster** for DNS/vhost/bucket enumeration

## Common Wordlists
- **SecLists** ([danielmiessler/SecLists](https://github.com/danielmiessler/SecLists)): `Discovery/Web-Content/`, `Discovery/DNS/`
- **Assetnote Wordlists** ([wordlists.assetnote.io](https://wordlists.assetnote.io/)): includes the `.kite` route files for kiterunner
- **DirBuster lists**: `directory-list-2.3-{small,medium}.txt`
- **dirb lists**: `/usr/share/dirb/wordlists/`
- **OneListForAll** ([six2dez/OneListForAll](https://github.com/six2dez/OneListForAll))

## General Tips
- **Calibrate first**: handle wildcard or soft-404 responses (`ffuf -ac`, feroxbuster auto-filter) before trusting results.
- **Throttle responsibly**: respect engagement rules with rate limits (`ffuf -rate`, `feroxbuster --rate-limit`, `gobuster --delay`).
- **Route through Burp** (`-x http://127.0.0.1:8080` or equivalent) to inspect interesting hits.
- **Match wordlists to the tech stack**: IIS vs. Apache vs. Node, and API vs. classic web app.
- **Save structured output** (JSON) so results can be diffed and fed into other tools.
