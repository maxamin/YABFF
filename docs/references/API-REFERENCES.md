# Web Fuzzer API References — Index

Full per-item API references for the source of every fuzzer in this workspace. Each file documents **every** type, function, method, constant, and macro in that project — generated from a complete source read of the cloned repository (test files excluded).

Each item is listed with its full signature and doc comment; composite types (structs / classes / enums / interfaces) also list their fields and members.

## References

| Tool | Language | Version | Commit | Items | Reference | Source clone |
|---|---|---|---|--:|---|---|
| **feroxbuster** | Rust | 2.13.1 | `1f595da` | 934 | [feroxbuster-api-reference.md](feroxbuster-api-reference.md) | `tools/feroxbuster` |
| **ffuf** | Go | 2.1.0 | `8d65e03` | 386 | [ffuf-api-reference.md](ffuf-api-reference.md) | `tools/ffuf` |
| **gobuster** | Go | 3.8.2 | `c77583f` | 217 | [gobuster-api-reference.md](gobuster-api-reference.md) | `tools/gobuster` |
| **kiterunner** | Go | 1.0.2 | `7d5824c` | 864 | [kiterunner-api-reference.md](kiterunner-api-reference.md) | `tools/kiterunner` |
| **wfuzz** | Python | 3.1.0 | `2263cd0` | 1200 | [wfuzz-api-reference.md](wfuzz-api-reference.md) | `tools/wfuzz` |
| **dirsearch** | Python | 0.4.3 | `fbf5da9` | 958 | [dirsearch-api-reference.md](dirsearch-api-reference.md) | `tools/dirsearch` |
| **patator** | Python | 1.1 | `964e87c` | 270 | [patator-api-reference.md](patator-api-reference.md) | `tools/patator` |
| **dirb** | C | 2.22 | `2db1375` | 91 | [dirb-api-reference.md](dirb-api-reference.md) | `tools/dirb` |
| **DirBuster** | Java | 1.0-RC1 | `634d166` | 1448 | [dirbuster-api-reference.md](dirbuster-api-reference.md) | `tools/dirbuster` |

**Total: 6,368 documented items across 9 tools (4 languages).**

## Companion documents

- [feroxbuster-documentation.md](feroxbuster-documentation.md) — architecture, runtime flow, CLI flags, and feature narrative for feroxbuster (the deepest write-up; feroxbuster also has the per-item reference above).
- [web-fuzzers-comparison.md](../analysis/web-fuzzers-comparison.md) — feature/capability comparison across all tools.
- [benchmark.md](../analysis/benchmark.md) — empirical speed/accuracy benchmark.

## Notes on method & scope

- **How they were generated:** each project was cloned, then parsed with a language-appropriate extractor — Rust/Go/C/Java via signature parsing, Python via the `ast` module (exact). Every non-test definition is included verbatim with its signature and doc comment.
- **Excluded:** test files (`*_test.go`, `tests/`, `test_*.py`, etc.), vendored/third-party code, and build artifacts. The counts reflect first-party source only.
- **Source attribution:** dirb was cloned from the Kali packaging mirror (`gitlab.com/kalilinux/packages/dirb`); DirBuster from the `KajanM/DirBuster` source mirror (the Kali `dirbuster` package ships only a JAR + wordlists, no source).
- **Caveat:** the C and Java extractors are regex-based, so a small number of field/macro entries in `dirb` and `DirBuster` may be imperfectly delimited. The Python (AST-based), Go, and Rust extractions are structurally exact.
