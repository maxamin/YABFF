# feroxbuster — Complete Technical Documentation

A fast, simple, recursive content-discovery tool written in Rust.

- **Repository:** [epi052/feroxbuster](https://github.com/epi052/feroxbuster)
- **Documented version:** `2.13.1`
- **Documented commit:** `1f595da` (2026-09-05)
- **License:** MIT
- **Author:** Ben "epi" Risher (@epi052)
- **Edition:** Rust 2021
- **Source size:** ~25,200 lines across 12 modules

This document is generated from a source read of the cloned repository. It covers the architecture, the runtime flow, **every module**, and the public/notable functions, structs, enums, traits, constants, and CLI features in each. Test functions are catalogued at the module level rather than individually (there are ~250 of them).

> Cloned to `tools/feroxbuster/` in this workspace. Companion files: [web-fuzzers-comparison.md](web-fuzzers-comparison.md), [benchmark.md](benchmark.md).

---

## Table of Contents

1. [What it does](#1-what-it-does)
2. [Architecture](#2-architecture)
3. [Runtime flow](#3-runtime-flow)
4. [Dependency stack](#4-dependency-stack)
5. [Module reference](#5-module-reference)
   - [5.1 `main.rs` / `lib.rs`](#51-mainrs--librs--crate-root)
   - [5.2 `config`](#52-config--configuration-loading)
   - [5.3 `parser`](#53-parser--cli-definition)
   - [5.4 `client`](#54-client--http-client-construction)
   - [5.5 `event_handlers`](#55-event_handlers--the-actor-system)
   - [5.6 `scanner`](#56-scanner--the-request-engine)
   - [5.7 `scan_manager`](#57-scan_manager--scan-state--interactive-menu)
   - [5.8 `extractor`](#58-extractor--link-extraction)
   - [5.9 `heuristics`](#59-heuristics--connectivity-wildcard--dir-listing-detection)
   - [5.10 `filters`](#510-filters--response-filtering)
   - [5.11 `nlp`](#511-nlp--tf-idf-word-collection)
   - [5.12 `statistics`](#512-statistics)
   - [5.13 `response` / `url` / `message`](#513-response--url--message--data-types)
   - [5.14 `banner`](#514-banner)
   - [5.15 `progress`](#515-progress)
   - [5.16 `sync`](#516-sync--dynamic-semaphore)
   - [5.17 `logger`](#517-logger)
   - [5.18 `utils` / `traits` / `macros`](#518-utils--traits--macros)
6. [Feature reference](#6-feature-reference)
7. [Complete CLI flag reference](#7-complete-cli-flag-reference)
8. [Configuration reference](#8-configuration-reference)
9. [Output & serialization](#9-output--serialization)
10. [Constants](#10-constants)
11. [Build, test, package](#11-build-test-package)

---

## 1. What it does

feroxbuster performs **forced browsing** (a.k.a. content discovery): it takes a target URL and a wordlist, requests `target/word` for every word (optionally with extensions/methods), and reports responses that survive filtering. Its defining traits:

- **Recursive by default** — every discovered directory becomes a new scan.
- **Link extraction** — parses HTML/JS/`robots.txt` to find paths the wordlist would miss.
- **Automatic noise reduction** — wildcard/soft-404 auto-filtering, directory-listing detection.
- **Adaptive rate control** — `--auto-tune` (slow down on errors) and `--auto-bail` (stop on errors).
- **Resumable** — Ctrl+C saves state to a `.state` file; `--resume-from` continues it.
- **Interactive** — press `Enter` mid-scan to open a management menu (cancel scans, add filters/URLs).

---

## 2. Architecture

feroxbuster is an **async, event-driven, actor-style** program built on the Tokio runtime. Rather than sharing mutable state behind locks everywhere, most subsystems are **independent tasks (actors)** that own their data and communicate over **`mpsc` unbounded channels** carrying a single `Command` enum.

### 2.1 The Handle / Handler pattern

Each subsystem is split into two halves:

- **`*Handler`** — owns the data and runs a consumer loop (`start()`), receiving `Command`s on the `rx` side of a channel.
- **`*Handle`** — a lightweight, cloneable sender (`tx`) that other parts of the program hold to talk to the handler. Handles expose `send(Command)` and often `sync()`.

The four handler/handle pairs:

| Subsystem | Handler | Handle | Owns |
|---|---|---|---|
| Statistics | `StatsHandler` | `StatsHandle` | `Stats` |
| Filters | `FiltersHandler` | `FiltersHandle` | `FeroxFilters` |
| Output | `TermOutHandler` (+ `FileOutHandler`) | `TermOutHandle` | terminal/file writers |
| Scans | `ScanHandler` | `ScanHandle` | `FeroxScans` + wordlist |

### 2.2 `Handles` — the shared hub

`event_handlers::Handles` is an `Arc`-wrapped struct bundling all four handles plus the `Arc<Configuration>` and the `Arc<Vec<String>>` wordlist. It is cloned freely and passed to virtually every function that needs to emit a command, read config, or query state. This is the single most important type to understand — it is the "service locator" the whole codebase threads around.

### 2.3 The `Command` enum

`event_handlers::Command` is the universal message type (28 variants). Any actor can be driven purely by sending it `Command`s. Highlights:

- **Stats:** `AddRequest`, `AddError(StatError)`, `AddStatus(StatusCode)`, `AddToUsizeField`, `SubtractFromUsizeField`, `AddToF64Field`, `CreateBar(u64)`, `UpdateTargets`, `QueryOverallBarEta`, `Save`, `LoadStats`.
- **Filters:** `AddFilter(Box<dyn FeroxFilter>)`, `RemoveFilters(Vec<usize>)`.
- **Output:** `Report(Box<FeroxResponse>)`, `WriteToDisk(Box<FeroxMessage>)`.
- **Scans:** `ScanInitialUrls`, `ScanNewUrl`, `TryRecursion(Box<FeroxResponse>)`, `UpdateWordlist(Arc<Vec<String>>)`, `JoinTasks`, `AddDiscoveredExtension`, `AddScanPermits`, `SubtractScanPermits`.
- **Lifecycle:** `Ping`, `Sync`, `AddHandles`, `Exit`.

### 2.4 Concurrency model

- **Threads** (`-t/--threads`, default 50) is really the **per-directory concurrent-request** limit, implemented with a Tokio semaphore streaming requests via `futures::stream::buffer_unordered`.
- **Scan limit** (`-L/--scan-limit`) caps concurrent *directory scans* via a `DynamicSemaphore`.
- **Parallel** (`--parallel`) spawns *separate child processes*, one per stdin target (see [main flow](#3-runtime-flow)).
- **Rate limit** (`--rate-limit`) is a `leaky-bucket` token bucket, per directory.

---

## 3. Runtime flow

The synchronous `main()` (in `main.rs`) parses config, sets the open-file limit, initializes logging, builds the Tokio runtime, and calls `wrapped_main`. Then:

1. **Warm up** the lazy-static progress printer (so logging works).
2. **`--update`?** → run `update_app()` and exit.
3. **Load wordlist(s)** — `append_words_from_path()` merges every `-w` source (local file *or* `http(s)` URL, downloaded to disk) into one deduplicated `Vec<String>`, preserving order, with a legacy fallback to `SECONDARY_WORDLIST`. An empty leading `""` word is always present (so the bare directory itself is requested).
4. **Spawn the four actors** — `StatsHandler`, `FiltersHandler`, `TermOutHandler`, `ScanHandler` — and assemble `Handles`.
5. **Send user filters** to the filter handler (`filters::initialize`).
6. **Start the time-limit thread** if `--time-limit` set (and not `--parallel`).
7. **Start terminal-input handlers** — Ctrl+C (save state & exit) and Enter (interactive menu).
8. **Resume?** → load serialized scans + stats from the `.state` file.
9. **Get targets** — from `-u`, `--stdin`, `--resume-from`, or `--request-file`.
10. **`--parallel` branch** — re-exec feroxbuster as one child process per target (stripping `--parallel`/`--stdin`, forcing per-child output files), stream children's stdout, then return.
11. **Print banner** (unless quiet/silent) and check GitHub for a newer release.
12. **Connectivity heuristic** — drop unreachable targets (`HeuristicTests::connectivity`); bail if none live.
13. **`scan()`** — kick off scanning of live targets.
14. **`clean_up()`** — flush output, join all tasks, finalize bars, save state as needed.

### 3.1 Per-URL scan (`FeroxScanner::scan_url`)

For each directory scanned:

1. On the **initial** target only: fetch and parse `robots.txt` for links.
2. Acquire a scan-limiter permit (status `Waiting` → `Running`).
3. **Heuristics:** detect directory listing; if found and `--extract-links`, scoop up the links; if `--scan-dir-listings` isn't set, just report it. Detect wildcard/soft-404 responses and install auto-filters.
4. **Stream requests:** for every word, `Requester::request(word)` builds URL variants (`word`, `word.ext…`, `word/`), applies each configured HTTP method, honors the rate limiter and auto-tune/bail policy, makes the request, and routes the `FeroxResponse`.
5. Each response → filter check → (if kept) report to output, replay to proxy, collect extensions/words/backups, and evaluate for **recursion** (`TryRecursion`) and link extraction.

---

## 4. Dependency stack

| Crate | Role |
|---|---|
| `tokio` (full) | async runtime, tasks, mpsc channels, semaphores |
| `reqwest` (socks, native-tls-alpn) | HTTP client |
| `futures` | stream combinators (`buffer_unordered`) |
| `scraper` | HTML parsing for link extraction & dir-listing detection |
| `clap` 4 + `clap_complete` | CLI parsing & shell completions |
| `serde` / `serde_json` / `toml` / `serde_regex` | config + NDJSON state serialization |
| `indicatif` + `console` + `crossterm` | progress bars & terminal UI |
| `regex` | link-finder, denylist, time-spec validation |
| `gaoya` | SimHash implementation for similarity filtering |
| `leaky-bucket` | rate limiting |
| `openssl` (vendored) | TLS backend |
| `rlimit` | raise open-file descriptor limit |
| `self_update` | `--update` (GitHub releases) |
| `uuid` | wildcard-probe random strings, scan IDs |
| `ctrlc` / `anyhow` / `log` / `env_logger` / `dirs` | signals / errors / logging / paths |
| dev: `httpmock`, `assert_cmd`, `predicates`, `tempfile` | testing |

Release profile: `lto = true`, `codegen-units = 1`, `panic = "abort"`.

---

## 5. Module reference

Crate module tree (from `lib.rs`), `pub` unless noted:

```
banner   config   client(priv)   event_handlers   filters   heuristics
logger   parser(priv)   progress   scan_manager   scanner   statistics
sync   traits(priv)   utils   extractor(priv)   macros(priv)   url(priv)
response(priv)   message(priv)   nlp(priv)
```

---

### 5.1 `main.rs` / `lib.rs` — crate root

**`lib.rs`** declares the module tree and all crate-wide constants and type aliases:

- Type aliases: `CommandSender`, `CommandReceiver` (`mpsc::Unbounded*<Command>`), `Joiner` (`JoinHandle<Result<()>>`), `FeroxChannel<T>`, `ExtractionResult` (`HashSet<String>`).
- See [§10 Constants](#10-constants) for the constant list.

**`main.rs`** — the binary entry point.

| Item | Description |
|---|---|
| `fn main() -> Result<()>` | Sync entry: parse `Configuration`, set open-file limit, init logger, build Tokio runtime, call `wrapped_main`. |
| `async fn wrapped_main(config)` | The orchestrator described in [§3](#3-runtime-flow). |
| `fn append_words_from_path(path, words, seen)` | Read a wordlist file, append unique non-comment/non-empty lines to `words`, tracking dupes in `seen`. |
| `async fn scan(targets, handles)` | Dispatch to single-URL or stdin scanning; sends `ScanInitialUrls`. |
| `async fn get_targets(handles)` | Collect targets from CLI or stdin → `Vec<String>`. |
| `async fn clean_up(handles, tasks)` | Graceful shutdown: drain channels, join tasks, finalize bars, save state. |
| `async fn update_app(insecure, server_certs)` | Run `self_update` against GitHub releases. |
| `fn build_update_client(...)` | Build a custom `reqwest` client for `self_update` honoring `--insecure`/`--server-certs`; returns `None` when neither applies (issue #1148). |
| `static PARALLEL_LIMITER: Semaphore` | Caps concurrent child processes under `--parallel`. |

---

### 5.2 `config` — configuration loading

Files: `container.rs` (the `Configuration` struct + loading logic), `utils.rs` (serde defaults, helper types, parsers), `mod.rs`.

**`Configuration`** — the central settings struct (~70 fields; full list in [§8](#8-configuration-reference)). Implements `Default`, `Serialize`/`Deserialize`, and `FeroxSerialize`.

Precedence when building config (`Configuration::new()`), each layer overriding the last:
1. Built-in defaults (`Configuration::default()`).
2. `ferox-config.toml` found in (in order): `/etc/feroxbuster/`, `$CONFIG_DIR/feroxbuster/`, the binary's directory, the CWD.
3. Command-line arguments.

Key methods on `Configuration`:

| Method | Purpose |
|---|---|
| `new() -> Result<Self>` | Build the final singleton config from defaults + config files + CLI. |
| `parse_config_files(&mut)` | Walk the precedence list of `ferox-config.toml` locations. |
| `parse_cli_args(&ArgMatches) -> Self` | Translate clap matches into config (uses the `update_config_if_present!`, `update_if_not_default!`, `came_from_cli!`, `update_config_with_num_type_if_present!` macros). |
| `parse_and_merge_config(path, &mut)` | Read one config file and merge it. |
| `merge_config(&mut, new)` | Field-by-field overwrite. |
| `parse_config(path) -> Result<Self>` | serde-deserialize a TOML file into `Configuration`. |
| `try_rebuild_clients(&mut)` | Rebuild the `reqwest` client(s) when client-affecting settings changed. |
| `parse_data_arg(&mut, arg, content_type)` | Read `--data` body from STDIN or `@file`, set content type. |

**`config/utils.rs`** — serde default providers and free functions:

- **Default providers** (used by `#[serde(default = "...")]`): `serialized_type`, `timeout`, `save_state`, `threads`, `request_protocol`, `status_codes`, `methods`, `ignored_extensions`, `backup_extensions`, `wordlist`, `user_agent`, `depth`, `extract_links`, `response_size_limit`.
- `deserialize_wordlist` — custom deserializer accepting either a single string or a list (legacy state-file compatibility).
- **Enums:**
  - `OutputLevel { Default, Quiet, Silent }` + `determine_output_level(quiet, silent, json)`.
  - `RequesterPolicy { Default, AutoTune, AutoBail }` + `determine_requester_policy(auto_tune, auto_bail)`.
  - `ContentType { Json, UrlEncoded }` + `to_header_value()`.
- **Parsers/helpers:** `split_query("k=v")`, `split_header("K: V")`, `combine_cookies(a, b)`, `parse_request_file(&mut Configuration)` (parse a raw Burp-style HTTP request file into config), `preconfig_log(level, msg)` (log before the real logger exists), `report_and_exit(err) -> !`.

---

### 5.3 `parser` — CLI definition

`parser.rs` builds the entire clap `Command` (68 `Arg`s, grouped by `help_heading`). Notable items:

| Item | Description |
|---|---|
| `pub fn initialize() -> clap::Command` | Construct the full CLI (also consumed by `build.rs` to generate shell completions). |
| `fn valid_time_spec(&str)` | Validator for `--time-limit`, matching `TIMESPEC_REGEX` (`N[smhd]`). |
| `static TIMESPEC_REGEX` | `^\d+[smhdSMHD]$`-style regex for time specs. |
| `const EPILOGUE` | The `NOTE:`/`EXAMPLES:` help footer. |

The generated flags are catalogued in [§7](#7-complete-cli-flag-reference).

---

### 5.4 `client` — HTTP client construction

`client.rs` (private module). Builds `reqwest::Client` instances.

| Item | Description |
|---|---|
| `pub struct ClientConfig<'a, I>` | All inputs needed to build a client (timeout, UA, redirects, proxy, insecure flag, server/client certs, headers, scope for redirect handling…). |
| `pub fn initialize(config) -> Result<Client>` | Build a client, optionally with **scope-aware redirect handling**. |
| `fn create_redirect_policy(config) -> Policy` | Construct a `reqwest` redirect `Policy`; when scope is set, only follows redirects that stay in scope. |

Supports SOCKS proxies, custom root certificates (PEM/DER), and mutual TLS (client cert + key).

---

### 5.5 `event_handlers` — the actor system

Files: `command.rs`, `container.rs`, `statistics.rs`, `filters.rs`, `outputs.rs`, `scans.rs`, `inputs.rs`, `mod.rs`.

**`command.rs`** — the `Command` enum ([§2.3](#23-the-command-enum)).

**`container.rs`:**

| Item | Description |
|---|---|
| `struct Handles` | The shared hub: `stats`, `filters`, `output`, `scans` (set later), `config`, `wordlist`. |
| `Handles::new(...)` / `for_testing(...)` | Construct the hub. |
| `set_scan_handle(handle)` | Attach the `ScanHandle` after init (chicken-and-egg with `ScanHandler`). |
| `send_scan_command(cmd)` | Convenience sender to the scan actor. |
| `ferox_scans() -> Result<Arc<FeroxScans>>` | Reach into the scan handle for the scan collection. |
| `num_collected_extensions()` / `collected_extensions()` | Query auto-collected extensions. |
| `expected_num_requests_per_dir()` / `expected_num_requests_multiplier()` | Request-count estimates: `(base + static exts + collected exts) × #methods × #words`. |
| `struct Tasks` | Bundle of the four `JoinHandle`s (terminal, stats, filters, scans). |

**`statistics.rs`:** `StatsHandle` (`send`, `sync`) + `StatsHandler` (`start` consumer loop, `increment_bar`, `initialize(config) -> (Joiner, StatsHandle)`). Owns the `Stats` object; reacts to all stat-related `Command`s.

**`filters.rs`:** `FiltersHandle` (`send`, `sync`) + `FiltersHandler` (`start`, `initialize() -> (Joiner, FiltersHandle)`). Owns `FeroxFilters`; handles `AddFilter`/`RemoveFilters`.

**`outputs.rs`:** two handlers.
- `TermOutHandle` (the handle: `send`, `sync(send_to_file)`).
- `FileOutHandler` — writes results/log to `--output`/`--debug-log`.
- `TermOutHandler` — the terminal writer; `initialize(...)` spawns both; `start()` consumes `Report`/`WriteToDisk`.
  - `process_response(...)` — on each kept response: print, send to replay proxy, request backups, track it.
  - `generate_backup_urls(response)` — produce 6–7 likely backup URLs (`file~`, `file.bak`, `file.bak2`, `file.old`, `file.1`, `.file.swp`, plus extension-stripped variants). See issue #513 (backups requested from the *found* directory).

**`scans.rs`:** `ScanHandle` (`send`) + `ScanHandler`.
- `initialize(handles) -> (Joiner, ScanHandle)`; `start()` consumes scan commands.
- `ordered_scan_url(targets, order)` — scan a batch with a `ScanOrder` (Initial vs. Latest).
- `try_recursion(response)` — decide whether a response warrants a new directory scan.
- `update_all_bar_lengths()` — recompute expected request counts across all bars (needed when `--collect-extensions` changes the multiplier).
- `get_wordlist(offset)` — hand out an `Arc` view of the wordlist.

**`inputs.rs`:** `TermInputHandler`.
- `initialize(handles)` sets up **Ctrl+C** (`sigint_handler` → save state, exit) and **Enter** (`enter_handler` → toggle `PAUSE_SCAN`, drop into the interactive menu).
- `static SCAN_COMPLETE: AtomicBool` tells the input handler to stop.

---

### 5.6 `scanner` — the request engine

Files: `ferox_scanner.rs`, `requester.rs`, `init.rs`, `limit_heap.rs`, `policy_data.rs`, `utils.rs`, `mod.rs`.

**`ferox_scanner.rs`:**

| Item | Description |
|---|---|
| `struct FeroxScanner` | Owns target URL, `ScanOrder`, wordlist, scan-limiter, handles. |
| `scan_url()` | Primary entry point per directory ([§3.1](#31-per-url-scan-feroxscannerscan_url)). |
| `stream_requests(...)` | Produce & await the stream of per-word request tasks (`buffer_unordered` over threads). |
| `check_for_user_input(...)` | Busy-loop on `PAUSE_SCAN` when the user paused. |
| `static RESPONSES: FeroxResponses` | Global set of all responses seen (dedup + `--unique`). |

**`requester.rs`** (the largest file, ~1,760 lines) — `struct Requester`, one per scan. Handles rate limiting, adaptive tuning, and the actual per-word requests.

| Method | Description |
|---|---|
| `from(scanner, ferox_scan) -> Result<Self>` | Build a requester for a scan. |
| `request(word)` | Make request(s) for a word (all URL/extension/method variants), report, and try recursion. |
| `build_a_bucket(limit) -> RateLimiter` | Build a leaky-bucket for a given req/s (fixed so rates 1–20 don't collapse to 1 — bug #1). |
| `limit()` | Await a rate-limiter token. |
| `set_rate_limiter(new_limit)` | Swap the bucket for a new rate. |
| `should_enforce_policy() -> Option<PolicyTrigger>` | Decide whether auto-tune/bail should engage: general errors ≥ threads×2, ≥90% 403s, or ≥30% 429s (per-scan counts, bug #4). |
| `too_many_errors()` / `too_many_status_errors(trigger)` | Threshold checks. |
| `adjust_limit(trigger, create_limiter)` | Move the rate up/down via the `LimitHeap`. |
| `tune(trigger)` | Enforce **auto-tune**: lower rate, cooldown, retry. |
| `bail(trigger)` | Enforce **auto-bail**: abort the worst-offending scan. |
| `cool_down()` | Sleep + set the cooldown flag. |
| `static TF_IDF: Arc<RwLock<TfIdf>>` | Global TF-IDF model for `--collect-words`. |

**`limit_heap.rs`** — `LimitHeap`: a bespoke array-backed max-heap of 255 candidate req/s values. Auto-tune walks it: left child = faster, right child = slower. Methods: `build`, `move_left/right/up/to`, `value/set_value`, `has_parent/has_children`, `parent_value`, `clamp_to_max` (cap to `--rate-limit` when both flags used).

**`policy_data.rs`** — `PolicyData`: holds the `RequesterPolicy`, per-trigger error counts, the `LimitHeap`, and the current limit. `new(policy, timeout)`, `with_rate_limit(n)`, `set_reqs_sec`, `adjust_up(streak)`, `adjust_down`, `reset_heap`, `heap_initialized`, getters/setters. `MAX_WAIT_TIME_MS = 30_000`.

**`scanner/utils.rs`** — `PolicyTrigger { Status403, Status429, Errors, TryAdjustUp }` + `as_index()`.

**`scanner/init.rs`** — `initialize(num_words, handles)`: one-time warm-up before scanning.

---

### 5.7 `scan_manager` — scan state & interactive menu

Files: `scan.rs`, `scan_container.rs`, `response_container.rs`, `menu.rs`, `state.rs`, `order.rs`, `utils.rs`, `mod.rs`.

**`scan.rs`** — `FeroxScan`, the unit of scan tracking:

- Enums: `ScanType { File, Directory }`, `ScanStatus { NotStarted, Running, Complete, Cancelled }`, `Visibility { Visible, Hidden }`.
- Lifecycle: `new(...)`, `abort(active_bars)`, `finish(active_bars)`, `join()`, `set_task/status/start_time`, `stop_progress_bar`, `progress_bar()`.
- Status predicates: `is_active`, `is_complete`, `is_cancelled`, `is_running`, `is_not_started`.
- Error/stat tracking: `add_403`, `add_429`, `add_error`, `num_errors(trigger)`, `requests`, `requests_per_second`, `requests_made_so_far`.
- Visibility: `visible()`, `swap_visibility()` (for `--limit-bars`).
- Custom `Serialize`/`Deserialize` (NDJSON state), `PartialEq` (by `id`), `Display`.

**`scan_container.rs`** — `FeroxScans`, the collection of all scans (the scan "registry"):

- `new(output_level, bar_limit)`, `insert(scan)`, `contains(url)`, `get_scan_by_url`, `get_base_scan_by_url`.
- `add_scan` / `add_directory_scan` / `add_file_scan` — create + register a scan, returning whether it was new.
- `add_serialized_scans(file, handles)` — load scans + collected extensions on resume.
- `increment_status_code` / `increment_error` — per-URL stat routing.
- Extension collection: `add_discovered_extension(ext) -> bool`.
- Interactive/display: `display_scans`, `display_filters`, `interactive_menu`, `cancel_scans(indexes, force)`, `print_known_responses`, `print_completed_bars`.
- Pause: `pause(...)` busy-loops on `PAUSE_SCAN` every `SLEEP_DURATION` ms.
- Bars: `set_bar_length`, `number_of_bars`, `make_visible`, `has_active_scans`, `get_active_scans`.
- Statics: `PAUSE_SCAN: AtomicBool`, `INTERACTIVE_BARRIER: AtomicUsize`.

**`response_container.rs`** — `FeroxResponses`: thread-safe set of all `FeroxResponse`s (`insert`, `contains`), custom `Serialize`.

**`menu.rs`** — the interactive **Scan Management Menu**:

- `enum MenuCmd { AddUrl(String), Cancel(Vec<usize>, bool), AddFilter(Box<dyn FeroxFilter>), RemoveFilter(Vec<usize>), SetScanPermits(usize) }` and `MenuCmdResult`.
- `struct Menu` with printing helpers (`print_header/border/footer/eta/scan_limit`), screen control (`clear_screen`, `hide/show_progress_bars`, `println`), and input parsing (`get_command_input_from_user`, `split_to_nums`, `confirm_cancellation`). Supported commands include cancel, add-url, new-filter, rm-filter, set-scan-limit.

**`state.rs`** — `FeroxState`: the full serializable snapshot (config + scans + stats + responses) written to the `.state` file. Implements `FeroxSerialize`.

**`order.rs`** — `enum ScanOrder { Initial, Latest }` (user-supplied vs. discovered-during-scan).

**`scan_manager/utils.rs`:** `start_max_time_thread(handles)` (enforce `--time-limit`), `resume_scan(file) -> Configuration` (rehydrate from `.state`), `determine_bar_type(...)` (pick a `BarType` from `--limit-bars` + output level).

---

### 5.8 `extractor` — link extraction

Files: `builder.rs`, `container.rs`, `mod.rs` (private module).

**`builder.rs`:**
- `enum ExtractionTarget { ResponseBody, RobotsTxt, DirectoryListing, ParseHtml }` (variants correspond to extraction sources).
- `struct ExtractorBuilder` — fluent builder: `.handles()`, `.url()`, `.target()`, `.response()`, `.build() -> Extractor`.
- Regex constants: `LINKFINDER_REGEX` (JS link extraction, from GerbenJavado/LinkFinder), `ROBOTS_TXT_REGEX`, `URL_CHARS_REGEX` (strip RFC-3986-invalid chars).

**`container.rs`** — `struct Extractor` with the extraction logic:

| Method | Description |
|---|---|
| `extract()` | Dispatch to the right source based on `ExtractionTarget`. |
| `extract_from_body()` | Parse an HTML body: detect dir listing, run LinkFinder on embedded/linked JS, extract HTML link attributes. |
| `extract_from_robots()` | Request `/robots.txt` from the URL root and parse `Allow`/`Disallow` paths. |
| `extract_from_dir_listing()` | Scoop all `<a>` links from a directory-listing page. |
| `extract_all_links_from_javascript(...)` | Apply LinkFinder regex, expand each fragment into all sub-paths. |
| `extract_all_links_from_html_tags(...)` / `extract_links_by_attr(...)` | Pull links from many tag/attribute pairs. |
| `add_all_sub_paths(path, links)` / `get_sub_paths_from_path(path)` | Expand `a/b/c/f.svg` → `a/b/c/`, `a/b/`, `a/`. |
| `normalize_url_path(path)` | Trim whitespace/slashes/query/fragment. |
| `parse_url_and_add_subpaths(...)` | Parse a URL, drop out-of-scope, add sub-paths. |
| `request_links(links)` | Task the request handler to fetch discovered links. |
| `make_extract_request(location)` | Request `<base>/<location>`. |
| `request_link(url, handles)` (module fn) | Dedup check + request a single link. |
| `update_stats(num_links)` | Bump `LinksExtracted` + expected responses. |

---

### 5.9 `heuristics` — connectivity, wildcard & dir-listing detection

`heuristics.rs` — `struct HeuristicTests`.

| Item | Description |
|---|---|
| `connectivity(target_urls)` | Try to reach all targets before scanning; exit if none reachable; return live ones. |
| `directory_listing(url) -> Option<DirListingResult>` | Detect a directory-listing page. |
| `detect_directory_listing(html) -> Option<DirListingType>` | Title-based detection: Apache (`Index of /`), Tomcat/Python (`Directory Listing for /`), ASP.NET (`Directory Listing -- /`), or **custom** (2+ combined signals). |
| `has_parent_directory_link(html)` | Signal: a `../` / "parent directory" link. |
| `has_directory_table_headers(html)` | Signal: ≥2 of name/size/time column headers. |
| `has_sorting_query_params(html)` | Signal: Apache-style `?C=N&O=A` sort links. |
| `has_high_link_density(html)` | Signal: ≥3 file/dir-looking links (`MIN_LINKS = 3`). |
| `detect_404_like_responses(...)` / `examine_404_like_responses(...)` | Probe with random UUID paths, learn the server's soft-404 pattern(s), install wildcard filters. |
| `unique_string(length)` | Concatenate `length` hyphen-free UUIDs (32 chars each) for wildcard probing. |

Enums: `DirListingType { Apache, TomCatOrPython, AspDotNet, Custom }`, `WildcardResult { WildcardDirectory(usize), FourOhFourLike(usize) }`; struct `DirListingResult`. `static FILE_EXTENSION_SET` — combined `COMMON_FILE_EXTENSIONS` + backups for O(1) lookup.

---

### 5.10 `filters` — response filtering

Files: `mod.rs`, `container.rs`, `init.rs`, `utils.rs`, plus one file per filter type: `status_code.rs`, `words.rs`, `lines.rs`, `size.rs`, `regex.rs`, `similarity.rs`, `wildcard.rs`, `empty.rs`.

All filters implement the **`FeroxFilter` trait** (from `traits.rs`):

```rust
trait FeroxFilter: Debug + Send + Sync {
    fn should_filter_response(&self, response: &FeroxResponse) -> bool;
    fn box_eq(&self, other: &dyn Any) -> bool;   // for equality across trait objects
    fn as_any(&self) -> &dyn Any;
}
```

| Filter | CLI flag | Filters when… |
|---|---|---|
| `StatusCodeFilter` | `-C/--filter-status` | status is in the deny list |
| `WordsFilter` | `-W/--filter-words` | body word count matches |
| `LinesFilter` | `-N/--filter-lines` | body line count matches |
| `SizeFilter` | `-S/--filter-size` | content length matches |
| `RegexFilter` | `-X/--filter-regex` | regex matches body/headers |
| `SimilarityFilter` | `--filter-similar-to` | body is SimHash-similar to a reference page |
| `WildcardFilter` | (auto) | response matches a learned wildcard/soft-404 pattern |
| `EmptyFilter` | (placeholder) | never (`false`) |

**`container.rs`** — `FeroxFilters` (the collection): `push(filter)`, `remove(indices)` (1-based, for the menu), `should_filter_response(...)` (the DRY gate over all filters), custom `Serialize`.

**`similarity.rs`** — `SimilarityFilter::new(hash, url, cutoff)`, `From<&FeroxResponse>`; uses `static SIM_HASHER: SimHash<SimSipHasher64, u64, 64>` (gaoya). Near-duplicate detection uses Hamming distance ≤ `NEAR_DUPLICATE_DISTANCE (3)`; `--unique` uses `UNIQUE_DISTANCE (1)`.

**`wildcard.rs`** — `WildcardFilter::new(dont_filter)`; compares size/words/lines + method to a learned wildcard baseline. Implements `Display`.

**`init.rs`** — `initialize(handles)`: turn all user-supplied filter flags into filter objects and push them to the handler.

**`utils.rs`** — `create_similarity_filter(...)` (fetch reference page, hash it, build the filter), `filter_lookup(type, value)` (menu helper: `"lines 40"` → a `Box<dyn FeroxFilter>`).

---

### 5.11 `nlp` — TF-IDF word collection

Private module powering `--collect-words`. Files: `constants.rs`, `document.rs`, `model.rs`, `term.rs`, `utils.rs`, `mod.rs`.

| Item | Description |
|---|---|
| `struct TfIdf` (`model.rs`) | The TF-IDF corpus model: `new`, `add_document`, `calculate_tf_idf_scores`, `all_words()` (terms with non-zero score), `num_documents`. |
| `struct Document` (`document.rs`) | One page's bag of terms: `new(text)`, `from_html(raw)` (skips `<script>`/`<style>`), `add_term`, `term_frequency(term)`, `terms()`. |
| `struct Term` / `TermMetaData` (`term.rs`) | A word + its per-corpus metadata (document frequency, per-doc counts, tf-idf score). |
| `utils.rs` | `preprocess(text)` (remove punctuation, lowercase, drop stop words), `normalize_case`, `remove_punctuation`, `remove_stop_words`, `inverse_document_frequency`, `tf_idf_score`. |
| `constants.rs` | `BOUNDED_WORD_REGEX` (`\b[a-zA-Z0-9_-]+\b`) and `STOP_WORDS` (323 spaCy-derived stop words). |

When enabled, high-scoring TF-IDF terms from response bodies are fed back into the wordlist for subsequent requests.

---

### 5.12 `statistics`

Files: `container.rs` (`Stats`), `field.rs`, `error.rs`, `macros.rs`, `init.rs`, `mod.rs`.

**`Stats`** — the global atomic statistics object. Tracks (all as atomics): requests, expected requests (per-scan + total), errors, per-class status counts (2xx/3xx/4xx/5xx), specific codes (200/301/302/401/403/429/500/503/504/508), timeouts, connection/redirect/request/URL-format/certificate errors, links extracted, extensions collected, resources discovered, files found, wildcards filtered, responses filtered, total scans, initial targets, directory scan times, total runtime.

Methods: `new(is_json)`, getters (`expected_per_scan`, `resources_discovered`, `errors`, `status_403s`, `status_429s`, `total_expected`, `initial_targets`), mutators (`add_request`, `add_error(StatError)`, `add_status_code(StatusCode)`, `update_usize_field`, `subtract_from_usize_field`, `update_f64_field`, `update_targets`, `update_runtime`), persistence (`save(seconds, path)`, `merge_from(file)` for resume). Custom `Serialize`/`Deserialize` + `FeroxSerialize`.

- `enum StatField` — `ExpectedPerScan, TotalScans, LinksExtracted, ExtensionsCollected, TotalExpected, WildcardsFiltered, ResponsesFiltered, ResourcesDiscovered, InitialTargets, DirScanTimes` (plus more) — names the field a `Command` targets.
- `enum StatError` — `Timeout, UrlFormat, Redirection, Connection, Request, Certificate, Other`.
- `macros.rs` — `atomic_increment!`, `atomic_load!`, `atomic_store!`.

---

### 5.13 `response` / `url` / `message` — data types

**`response.rs`** — `FeroxResponse`, the internal representation of an HTTP response.

- Getters: `status`, `method`, `wildcard`, `url`, `text`, `headers`, `content_length`, `timestamp`, `truncated`, `line_count`, `word_count`.
- Setters: `set_url`, `set_wildcard`, `set_text`, `drop_text` (free body memory).
- `from(response, ...)` — build from a `reqwest::Response` (reads up to `response_size_limit`, marking `truncated`).
- `parse_extension(handles)` — for `--collect-extensions`, extract & report the file extension.
- `reached_max_depth(...)` — has recursion depth been hit?
- `is_directory()` — recursion suitability (2xx ending in `/`, or 3xx `Location` → base+`/`).
- `send_report(sender)` — ship it to the output actor.
- Custom `Serialize`/`Deserialize`, `Display`, `FeroxSerialize` (human line + NDJSON).

**`url.rs`:**
- `trait UrlExt for Url` — `is_in_scope(scope)` (same domain or subdomain), `is_subdomain_of(parent)`.
- `struct FeroxUrl` — URL builder/analyzer: `from_string`/`from_url`, `formatted_urls(...)` (base + each extension + optional slash), `format(word, ext)`, `normalize()` (ensure trailing slash for dedup), `depth()` (count path segments).

**`message.rs`** — `FeroxMessage`: a structured log entry (message, level, time offset, module). `FeroxSerialize` → human string or NDJSON `{"type":"log",...}`.

---

### 5.14 `banner`

Files: `container.rs`, `entry.rs`, `mod.rs`.

- `struct Banner` — the startup banner. `new(targets, config)`, `print_to(writer, config)`, `check_for_updates(url, handles)` (query `UPDATE_URL` = GitHub latest-release API, set `UpdateStatus`), private `header()`/`footer()`.
- `struct BannerEntry` — one `emoji / title : value` row; `new`, `format_emoji` (fallback when no emoji support), `Display`. `INDENT = 3`, `COL_WIDTH = 22`.
- `enum UpdateStatus` — up-to-date / newer available / unknown.

---

### 5.15 `progress`

`progress.rs` — thin wrapper over `indicatif`.

- `static PROGRESS_BAR: MultiProgress` — container for all bars (drawn to stdout).
- `static PROGRESS_PRINTER: ProgressBar` — hidden bar for message printing that won't corrupt other bars.
- `enum BarType { Hidden, Default, Total, Message, Quiet }`.
- `add_bar(prefix, length, bar_type) -> ProgressBar`, `update_style(bar, bar_type)`.

---

### 5.16 `sync` — dynamic semaphore

`sync/dynamic_semaphore.rs` — `DynamicSemaphore`, a Tokio-semaphore wrapper whose capacity can change **at runtime** (needed because `-L/--scan-limit` can be changed live from the menu, and auto-tune adjusts concurrency).

- `new(permits)`, `acquire() -> DynamicSemaphorePermit`, `try_acquire()`.
- `reduce_capacity(n)` / `increase_capacity(n)` → returns previous capacity.
- `current_capacity()`, `available_permits()`, `permits_in_use()`, `close()`, `is_closed()`.
- `DynamicSemaphorePermit` — RAII permit; its `Drop` decides atomically whether to release or "forget" the permit (so over-capacity reductions take effect safely without races). Fully documented with rustdoc examples in-source.

---

### 5.17 `logger`

`logger.rs` — `initialize(config) -> Result<()>`: build a customized `env_logger` with a time-offset timestamp, color, and a level derived from `-v` verbosity (and NDJSON formatting under `--json`).

---

### 5.18 `utils` / `traits` / `macros`

**`utils.rs`** — crate-wide helpers:

| Function | Purpose |
|---|---|
| `make_request(...)` | Core HTTP request execution. |
| `logged_request(...)` | Wrapper around `make_request` that routes result codes/errors into per-scan stats (for auto-tune/bail). |
| `send_try_recursion_command(...)` | Emit a `TryRecursion` command (callable from scanner & extractor). |
| `create_report_string(...)` | Format the standard result line: `200  127l  283w  4134c http://…`. |
| `open_file(name)` / `write_to(...)` | Buffered append-mode file writing. |
| `status_colorizer(status)` / `module_colorizer(name)` | ANSI coloring (green 2xx, yellow 3xx, red 4xx/5xx/error, cyan wildcard, blue 1xx). |
| `ferox_print(msg, bar)` | Print via the progress bar (terminal) or plain stdout (piped, colors stripped). |
| `should_deny_url(url, handles)` + `should_deny_absolute` / `should_deny_regex` | `--dont-scan` denylist logic (absolute-URL child match, or path regex). |
| `parse_url_with_raw_path(url)` | Parse a URL without silently collapsing `..` traversal. |
| `slugify_filename(url, prefix, suffix)` | Build state/log filenames, e.g. `ferox-http_tesla_com-1606947491.state`. |
| `set_open_file_limit(limit) -> bool` | Raise the OS soft FD limit (nix). |
| `is_certificate_error(err)` / `fmt_err(msg)` / `timestamp()` | Misc helpers. |

**`traits.rs`** — `FeroxFilter` ([§5.10](#510-filters--response-filtering)) and `FeroxSerialize` (`as_str()` human form + `as_json()` NDJSON; implemented by `Configuration`, `FeroxResponse`, `FeroxMessage`, `Stats`, `FeroxState`).

**`macros.rs`** — `send_command!` (readability wrapper around `handle.send(...)`) and `skip_fail!` (in a loop, unwrap `Ok` or `continue` on `Err`).

---

## 6. Feature reference

| Feature | Flags / mechanism | Where implemented |
|---|---|---|
| Recursive discovery | default; `-d/--depth`, `-n/--no-recursion`, `--force-recursion` | `response::is_directory`, `scans::try_recursion` |
| Link extraction | `--extract-links` (default on), `--dont-extract-links` | `extractor` |
| robots.txt parsing | automatic with extract-links | `extractor::extract_from_robots` |
| Directory-listing detection & scan | `--scan-dir-listings` | `heuristics`, `extractor::extract_from_dir_listing` |
| Wildcard / soft-404 auto-filter | default; `-D/--dont-filter` to disable | `heuristics::detect_404_like_responses`, `filters::WildcardFilter` |
| Extension fuzzing | `-x/--extensions` (supports `@file`) | `url::FeroxUrl::formatted_urls` |
| Extension auto-collection | `-E/--collect-extensions`, `-I/--dont-collect` | `response::parse_extension`, `scan_container::add_discovered_extension` |
| Backup-file collection | `-B/--collect-backups` | `outputs::generate_backup_urls` |
| Word auto-collection | `-g/--collect-words` | `nlp` (TF-IDF) |
| Multiple HTTP methods | `-m/--methods` | `requester::request` |
| Request bodies | `--data`, `--data-json`, `--data-urlencoded` (support `@file`) | `config::parse_data_arg` |
| Raw request template | `--request-file`, `--protocol` | `config::parse_request_file` |
| Headers / cookies / queries | `-H`, `-b`, `-Q` | `config` |
| Scope control | `--scope`, `--dont-scan` (URL or regex) | `url::UrlExt`, `utils::should_deny_url` |
| Response filters | `-S -W -N -X -C --filter-similar-to` | `filters` |
| Unique responses | `--unique` | `filters::SimilarityFilter` + `RESPONSES` |
| Replay proxy | `-P/--replay-proxy`, `-R/--replay-codes` | `client`, `outputs::process_response` |
| Proxy | `-p/--proxy` (HTTP/SOCKS), `--burp`, `--burp-replay` | `client` |
| TLS options | `-k/--insecure`, `--server-certs`, `--client-cert`, `--client-key` | `client` |
| Rate limiting | `--rate-limit` | `requester`, `leaky-bucket` |
| Adaptive tuning | `--auto-tune`, `--auto-bail`, `--smart`, `--thorough` | `requester`, `LimitHeap`, `PolicyData` |
| Concurrency | `-t/--threads`, `-L/--scan-limit`, `--parallel` | scanner, `DynamicSemaphore`, `main` |
| Time limit | `--time-limit` | `scan_manager::start_max_time_thread` |
| Response size cap | `--response-size-limit` (default 4 MB) | `response::from` |
| Save / resume state | Ctrl+C, `--resume-from`, `--no-state` | `scan_manager::state`, `FeroxState` |
| Interactive menu | press `Enter` during scan | `scan_manager::menu` |
| Output | `-o/--output`, `--debug-log`, `--json`, `-v`, `--silent`, `--quiet`, `--limit-bars` | `event_handlers::outputs`, `logger`, `progress` |
| Wordlist from URL | `-w http://…` | `main::wrapped_main` |
| Self-update | `-U/--update` | `main::update_app`, `self_update` |
| Shell completions | generated at build time | `build.rs` + `parser` |

---

## 7. Complete CLI flag reference

`feroxbuster [OPTIONS]` (v2.13.1). Options that take multiple values accept repeated flags, space-separated, or comma-separated interchangeably.

**Target selection**
| Flag | Arg | Description |
|---|---|---|
| `-u, --url` | URL | Target URL (required unless `--stdin`/`--resume-from`/`--request-file`). |
| `--stdin` | | Read URL(s) from STDIN. |
| `--resume-from` | STATE_FILE | Resume a partial scan from a `.state` file. |
| `--request-file` | FILE | Raw HTTP request template for all requests. |

**Composite settings**
| Flag | Effect |
|---|---|
| `--burp` | `--proxy http://127.0.0.1:8080` + `--insecure`. |
| `--burp-replay` | `--replay-proxy http://127.0.0.1:8080` + `--insecure`. |
| `--data-urlencoded` | Set form content-type, `--data` (supports `@file`), method POST. |
| `--data-json` | Set JSON content-type, `--data` (supports `@file`), method POST. |
| `--smart` | `--auto-tune` + `--collect-words` + `--collect-backups`. |
| `--thorough` | `--smart` + `--collect-extensions` + `--scan-dir-listings`. |

**Proxy settings**
| Flag | Arg | Description |
|---|---|---|
| `-p, --proxy` | PROXY | HTTP(S)/SOCKS proxy for all requests. |
| `-P, --replay-proxy` | PROXY | Send only *unfiltered* requests through this proxy. |
| `-R, --replay-codes` | CODE… | Status codes to replay (default: `--status-codes`). |

**Request settings**
| Flag | Arg | Description |
|---|---|---|
| `-a, --user-agent` | UA | Set User-Agent (default `feroxbuster/2.13.1`). |
| `-A, --random-agent` | | Random UA from a built-in list of 12. |
| `-x, --extensions` | EXT… | Extensions to fuzz (`@file` supported). |
| `-m, --methods` | METHOD… | HTTP methods (default GET). |
| `--data` | DATA | Request body (`@file` supported). |
| `-H, --headers` | HEADER… | Extra headers. |
| `-b, --cookies` | COOKIE… | Cookies. |
| `-Q, --query` | QUERY… | URL query parameters. |
| `-f, --add-slash` | | Append `/` to each request. |
| `--protocol` | PROTO | Protocol for domain-only `--url`/`--request-file` (default https). |

**Request filters**
| Flag | Arg | Description |
|---|---|---|
| `--dont-scan` | URL… | Exclude URL(s)/regex from recursion/scans. |
| `--scope` | URL… | Extra in-scope domains/URLs. |

**Response filters**
| Flag | Arg | Description |
|---|---|---|
| `-S, --filter-size` | SIZE… | Filter by content length. |
| `-X, --filter-regex` | REGEX… | Filter by regex on body/headers. |
| `-W, --filter-words` | WORDS… | Filter by word count. |
| `-N, --filter-lines` | LINES… | Filter by line count. |
| `-C, --filter-status` | CODE… | Filter out status codes (deny list). |
| `--filter-similar-to` | URL… | Filter pages similar to a reference (SimHash). |
| `-s, --status-codes` | CODE… | Allow-list of status codes (default: all). |
| `--unique` | | Show only unique responses. |

**Client settings**
| Flag | Arg | Description |
|---|---|---|
| `-T, --timeout` | SECONDS | Request timeout (default 7). |
| `-r, --redirects` | | Follow redirects. |
| `-k, --insecure` | | Disable TLS validation. |
| `--server-certs` | PEM/DER… | Custom root certs. |
| `--client-cert` | PEM | mTLS client cert. |
| `--client-key` | PEM | mTLS client key. |

**Scan settings**
| Flag | Arg | Description |
|---|---|---|
| `-t, --threads` | N | Concurrent requests per directory (default 50). |
| `-n, --no-recursion` | | Disable recursion. |
| `-d, --depth` | N | Max recursion depth (0 = infinite, default 4). |
| `--force-recursion` | | Recurse on all found endpoints (respects depth). |
| `--dont-extract-links` | | Disable link extraction. |
| `-L, --scan-limit` | N | Max concurrent directory scans (0 = unlimited). |
| `--parallel` | N | Run N parallel child processes (one per stdin URL). |
| `--rate-limit` | N | Max requests/sec per directory (0 = unlimited). |
| `--response-size-limit` | BYTES | Max body bytes to read (default 4 MB). |
| `--time-limit` | SPEC | Max total runtime, e.g. `10m` (`N[smhd]`). |
| `-w, --wordlist` | FILE/URL | Wordlist path or URL (repeatable; merged). |
| `--auto-tune` | | Lower rate on excessive errors. |
| `--auto-bail` | | Stop scanning on excessive errors. |
| `-D, --dont-filter` | | Don't auto-filter wildcard responses. |
| `--scan-dir-listings` | | Recurse into directory listings. |

**Dynamic collection settings**
| Flag | Arg | Description |
|---|---|---|
| `-E, --collect-extensions` | | Auto-discover extensions. |
| `-B, --collect-backups` | [EXT…] | Auto-request backup files (default `~ .bak .bak2 .old .1`). |
| `-g, --collect-words` | | Auto-discover words (TF-IDF) into the wordlist. |
| `-I, --dont-collect` | EXT… | Extensions to ignore while collecting. |

**Output settings**
| Flag | Arg | Description |
|---|---|---|
| `-v, --verbosity` | (count) | Increase log level (`-v`…`-vvvv`). |
| `--silent` | | Only URLs (or JSON), no logging — good for piping. |
| `-q, --quiet` | | Hide progress bars & banner. |
| `--json` | | NDJSON logs to `--output`/`--debug-log`. |
| `-o, --output` | FILE | Write results to a file. |
| `--debug-log` | FILE | Write log entries to a file. |
| `--no-state` | | Disable `.state` output file. |
| `--limit-bars` | N | Max directory scan bars shown at once. |

**Update settings**
| Flag | Description |
|---|---|
| `-U, --update` | Update feroxbuster to the latest release (compatible with `-k`/`--server-certs`). |

---

## 8. Configuration reference

`Configuration` fields (all settable via `ferox-config.toml` unless marked `#[serde(skip)]`). Defaults shown where relevant.

| Field | Type | Default | Notes |
|---|---|---|---|
| `kind` | String | `"configuration"` | serialization tag |
| `wordlist` | Vec\<String> | `raft-medium-directories.txt` | repeatable; merged |
| `config` | String | — | path to loaded config file |
| `proxy` / `replay_proxy` | String | — | HTTP/SOCKS |
| `server_certs` | Vec\<String> | — | custom root certs |
| `client_cert` / `client_key` | String | — | mTLS |
| `target_url` | String | — | |
| `status_codes` | Vec\<u16> | all | allow list |
| `replay_codes` | Vec\<u16> | = status_codes | |
| `filter_status` | Vec\<u16> | — | deny list |
| `client` / `replay_client` | reqwest::Client | (skip) | built at runtime |
| `threads` | usize | 50 | per-dir concurrency |
| `timeout` | u64 | 7 | seconds |
| `verbosity` | u8 | 0 | |
| `silent` / `quiet` | bool | false | |
| `output_level` | OutputLevel | (skip) Default | |
| `auto_bail` / `auto_tune` | bool | false | |
| `requester_policy` | RequesterPolicy | (skip) Default | |
| `json` | bool | false | |
| `output` / `debug_log` | String | — | |
| `user_agent` | String | `feroxbuster/VERSION` | |
| `random_agent` | bool | false | |
| `redirects` | bool | false | |
| `insecure` | bool | false | |
| `extensions` | Vec\<String> | — | |
| `methods` | Vec\<String> | `[GET]` | |
| `data` | Vec\<u8> | — | request body |
| `headers` | HashMap | — | |
| `queries` | Vec\<(String,String)> | — | |
| `no_recursion` | bool | false | |
| `extract_links` | bool | true | |
| `add_slash` | bool | false | |
| `stdin` | bool | false | |
| `cached_stdin` | Vec\<String> | (skip) | |
| `depth` | usize | 4 | 0 = infinite |
| `scan_limit` | usize | 0 | 0 = unlimited |
| `parallel` | usize | 0 | |
| `rate_limit` | usize | 0 | per directory |
| `filter_size` | Vec\<u64> | — | |
| `filter_line_count` / `filter_word_count` | Vec\<usize> | — | |
| `filter_regex` | Vec\<String> | — | |
| `filter_similar` | Vec\<String> | — | |
| `dont_filter` | bool | false | |
| `resumed` / `resume_from` | bool / String | false / — | |
| `save_state` | bool | true | |
| `time_limit` | String | — | `N[smhd]` |
| `url_denylist` | Vec\<Url> | — | `--dont-scan` |
| `regex_denylist` | Vec\<Regex> | — | `--dont-scan` regex |
| `scope` | Vec\<Url> | — | |
| `collect_extensions` | bool | false | |
| `dont_collect` | Vec\<String> | 43 media exts | |
| `collect_backups` | bool | false | |
| `backup_extensions` | Vec\<String> | `~ .bak .bak2 .old .1` | |
| `collect_words` | bool | false | |
| `force_recursion` | bool | false | |
| `update_app` | bool | (skip) false | |
| `scan_dir_listings` | bool | false | |
| `request_file` | String | (skip) | |
| `protocol` | String | `https` | |
| `limit_bars` | usize | 0 | |
| `unique` | bool | false | |
| `response_size_limit` | usize | 4 MB | |

An example config ships as `ferox-config.toml.example`.

---

## 9. Output & serialization

- **Human output** (default): one line per result via `create_report_string`:
  `200      127l      283w     4134c http://localhost/faq` (status, lines, words, chars, URL). Colored by status class when a terminal is attached; colors stripped when piped.
- **NDJSON** (`--json`): each `FeroxSerialize` type emits one JSON object per line with a `"type"` tag — `configuration`, `response`, `log`, `statistics`, `state`. Example response object includes `url`, `path`, `status`, `content_length`, `line_count`, `word_count`, `headers`.
- **State file** (`ferox-<slug>-<timestamp>.state`): a `FeroxState` snapshot (config + scans + stats + responses), written on Ctrl+C or time-limit, consumed by `--resume-from`. Disable with `--no-state`.

---

## 10. Constants

Declared in `lib.rs` (crate-wide):

| Constant | Value | Meaning |
|---|---|---|
| `VERSION` | from Cargo.toml | build-time version |
| `DEFAULT_OPEN_FILE_LIMIT` | 8192 | max FDs raised at start |
| `DEFAULT_WORDLIST` | `…/seclists/…/raft-medium-directories.txt` | default wordlist (Kali path; `.\SecLists\…` on Windows) |
| `SECONDARY_WORDLIST` | `/usr/local/share/seclists/…` | fallback path |
| `DEFAULT_IGNORED_EXTENSIONS` | 43 exts | media/binary exts skipped when auto-collecting |
| `DEFAULT_BACKUP_EXTENSIONS` | `~ .bak .bak2 .old .1` | backup suffixes |
| `COMMON_FILE_EXTENSIONS` | 154 exts | for dir-listing link-density detection |
| `DEFAULT_STATUS_CODES` | all 60 codes | default allow list |
| `DEFAULT_METHOD` | `GET` | |
| `DEFAULT_CONFIG_NAME` | `ferox-config.toml` | |
| `USER_AGENTS` | 12 UAs | for `--random-agent` |
| `SLEEP_DURATION` | 500 ms | pause-poll interval |
| `HIGH_ERROR_RATIO` | 0.90 | auto-tune/bail 403 threshold |
| `NEAR_DUPLICATE_DISTANCE` | 3 | SimHash Hamming cutoff (similarity) |
| `UNIQUE_DISTANCE` | 1 | SimHash Hamming cutoff (`--unique`) |

---

## 11. Build, test, package

- **Build:** `cargo build --release` (release profile uses LTO, single codegen unit, `panic=abort`). `build.rs` regenerates shell completions from the `parser` definition into `shell_completions/`.
- **Test:** `cargo test` — ~250 unit/integration tests live beside the code (`tests.rs` in most modules) and in `tests/`. Dev-deps: `httpmock` (mock server), `assert_cmd`/`predicates` (CLI assertions), `tempfile`.
- **Install:** `install-nix.sh`, `snapcraft.yaml` (snap), `choco_package/` (Chocolatey), `Dockerfile`, `Makefile`/`Makefile.toml`. Debian packaging metadata is in `Cargo.toml` under `[package.metadata.deb]` (installs the binary, the example config to `/etc/feroxbuster/`, and bash/fish/zsh completions).
- **Config file:** looked for in `/etc/feroxbuster/`, `$CONFIG_DIR/feroxbuster/`, the binary directory, and the CWD.

---

*Documentation generated from a source read of `epi052/feroxbuster` @ `1f595da` (v2.13.1). For usage examples see the official docs: https://epi052.github.io/feroxbuster-docs/*
