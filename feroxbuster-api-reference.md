# feroxbuster — Complete API Reference

Exhaustive per-item reference for **every** struct, enum, trait, function, method, constant, static, type alias, and macro in the feroxbuster source (`epi052/feroxbuster` @ `1f595da`, v2.13.1). Generated from a full source read. Test functions are omitted (they live in each module's `tests.rs`); everything else is included verbatim with its signature, doc comment, and — for structs/enums — every field/variant.

> For architecture, runtime flow, CLI flags, and feature narrative, see [feroxbuster-documentation.md](feroxbuster-documentation.md). This file is the raw API surface.


---

## Modules

- [`config`](#module-config)
- [`event_handlers`](#module-event-handlers)
- [`scanner`](#module-scanner)
- [`scan_manager`](#module-scan-manager)
- [`extractor`](#module-extractor)
- [`filters`](#module-filters)
- [`nlp`](#module-nlp)
- [`statistics`](#module-statistics)
- [`banner`](#module-banner)
- [`sync`](#module-sync)
- [`client`](#module-client)
- [`heuristics`](#module-heuristics)
- [`lib`](#module-lib)
- [`logger`](#module-logger)
- [`macros`](#module-macros)
- [`main`](#module-main)
- [`message`](#module-message)
- [`parser`](#module-parser)
- [`progress`](#module-progress)
- [`response`](#module-response)
- [`traits`](#module-traits)
- [`url`](#module-url)
- [`utils`](#module-utils)

---


## Module `config` <a id='module-config'></a>

*120 documented items*


### `src/config/container.rs`


#### `macro update_config_if_present`  <sub>(macro)</sub>

> macro helper to abstract away repetitive configuration updates

```rust
macro_rules! update_config_if_present {
```

#### `macro update_if_not_default`  <sub>(macro)</sub>

> macro helper to abstract away repetitive if not default: update checks

```rust
macro_rules! update_if_not_default {
```

#### `macro came_from_cli`  <sub>(macro)</sub>

> macro helper to abstract away repetitive checks to see if the user has specified a value  
> for a given argument from the commandline or if we just had a default value in the parser

```rust
macro_rules! came_from_cli {
```

#### `macro update_config_with_num_type_if_present`  <sub>(macro)</sub>

> macro helper to abstract away repetitive if not default: update checks, specifically for  
> values that are number types, i.e. usize, u64, etc

```rust
macro_rules! update_config_with_num_type_if_present {
```

#### `Configuration`  <sub>(struct)</sub>

> Represents the final, global configuration of the program.  
>   
> This struct is the combination of the following:  
> - default configuration values  
> - plus overrides read from a configuration file  
> - plus command-line options  
>   
> In that order.  
>   
> Inspired by and derived from https://github.com/PhilipDaniels/rust-config-example

```rust
pub struct Configuration {
```

**Fields:**
- `pub kind: String,` — Name of this type of struct, used for serialization, i.e. `{"type":"configuration"}`
- `pub wordlist: Vec<String>,` — (deduplicated) words is fuzzed against every target
- `pub config: String,` — Path to the config file used
- `pub proxy: String,` — Proxy to use for requests (ex: http(s)://host:port, socks5(h)://host:port)
- `pub replay_proxy: String,` — Replay Proxy to use for requests (ex: http(s)://host:port, socks5(h)://host:port)
- `pub server_certs: Vec<String>,` — Path to a custom root certificate for connecting to servers with a self-signed certificate
- `pub client_cert: String,` — Path to a client's PEM encoded X509 certificate used during mutual authentication
- `pub client_key: String,` — Path to a client's PEM encoded PKSC #8 private key used during mutual authentication
- `pub target_url: String,` — The target URL
- `pub status_codes: Vec<u16>,` — Status Codes to include (allow list) (default: 200 204 301 302 307 308 401 403 405)
- `pub replay_codes: Vec<u16>,` — Status Codes to replay to the Replay Proxy (default: whatever is passed to --status-code)
- `pub filter_status: Vec<u16>,` — Status Codes to filter out (deny list)
- `pub client: Client,` — Instance of [reqwest::Client](https://docs.rs/reqwest/latest/reqwest/struct.Client.html)
- `pub replay_client: Option<Client>,` — Instance of [reqwest::Client](https://docs.rs/reqwest/latest/reqwest/struct.Client.html)
- `pub threads: usize,` — Number of concurrent threads (default: 50)
- `pub timeout: u64,` — Number of seconds before a request times out (default: 7)
- `pub verbosity: u8,` — Level of verbosity, equates to log level
- `pub silent: bool,` — Only print URLs (was --quiet in versions < 2.0.0)
- `pub quiet: bool,` — No header, no status bars
- `pub output_level: OutputLevel,` — more easily differentiate between the three states of output levels
- `pub auto_bail: bool,` — automatically bail at certain error thresholds
- `pub auto_tune: bool,` — automatically try to lower request rate in order to reduce errors
- `pub requester_policy: RequesterPolicy,` — more easily differentiate between the three requester policies
- `pub json: bool,` — Store log output as NDJSON
- `pub output: String,` — Output file to write results to (default: stdout)
- `pub debug_log: String,` — logs are written
- `pub user_agent: String,` — Sets the User-Agent (default: feroxbuster/VERSION)
- `pub random_agent: bool,` — Use random User-Agent
- `pub redirects: bool,` — Follow redirects
- `pub insecure: bool,` — Disables TLS certificate validation
- `pub extensions: Vec<String>,` — File extension(s) to search for
- `pub methods: Vec<String>,` — HTTP requests methods(s) to search for
- `pub data: Vec<u8>,` — HTTP Body data to send during request
- `pub headers: HashMap<String, String>,` — HTTP headers to be used in each request
- `pub queries: Vec<(String, String)>,` — URL query parameters
- `pub no_recursion: bool,` — Do not scan recursively
- `pub extract_links: bool,` — Extract links from html/javscript
- `pub add_slash: bool,` — Append / to each request
- `pub stdin: bool,` — Read url(s) from STDIN
- `pub cached_stdin: Vec<String>,` — Cached stdin contents to facilitate populating scope from stdin targets
- `pub depth: usize,` — Maximum recursion depth, a depth of 0 is infinite recursion
- `pub scan_limit: usize,` — Number of concurrent scans permitted; a limit of 0 means no limit is imposed
- `pub parallel: usize,` — Number of parallel scans permitted; a limit of 0 means no limit is imposed
- `pub rate_limit: usize,` — Number of requests per second permitted (per directory); a limit of 0 means no limit is imposed
- `pub filter_size: Vec<u64>,` — Filter out messages of a particular size
- `pub filter_line_count: Vec<usize>,` — Filter out messages of a particular line count
- `pub filter_word_count: Vec<usize>,` — Filter out messages of a particular word count
- `pub filter_regex: Vec<String>,` — Filter out messages by regular expression
- `pub dont_filter: bool,` — Don't auto-filter wildcard responses
- `pub resumed: bool,` — Scan started from a state file, not from CLI args
- `pub resume_from: String,` — Resume scan from this file
- `pub save_state: bool,` — Whether or not a scan's current state should be saved when user presses Ctrl+C
- `pub time_limit: String,` — non-negative integer and the next character is either s, m, h, or d (case insensitive)
- `pub filter_similar: Vec<String>,` — Filter out response bodies that meet a certain threshold of similarity
- `pub url_denylist: Vec<Url>,` — URLs that should never be scanned/recursed into
- `pub regex_denylist: Vec<Regex>,` — URLs that should never be scanned/recursed into based on a regular expression
- `pub scope: Vec<Url>,` — Allowed domains/URLs for redirects and link extraction
- `pub collect_extensions: bool,` — Automatically discover extensions and add them to --extensions (unless they're in --dont-collect)
- `pub dont_collect: Vec<String>,` — don't collect any of these extensions when --collect-extensions is used
- `pub collect_backups: bool,` — Automatically request likely backup extensions on "found" urls
- `pub backup_extensions: Vec<String>,`
- `pub collect_words: bool,` — Automatically discover important words from within responses and add them to the wordlist
- `pub force_recursion: bool,` — override recursion logic to always attempt recursion, still respects --depth
- `pub update_app: bool,` — Auto update app feature
- `pub scan_dir_listings: bool,` — whether to recurse into directory listings or not
- `pub request_file: String,` — path to a raw request file generated by burp or similar
- `pub protocol: String,` — default request protocol
- `pub limit_bars: usize,` — number of directory scan bars to show at any given time, 0 is no limit
- `pub unique: bool,` — only show unique responses based on status code and word count
- `pub response_size_limit: usize,` — Maximum size of response to read in bytes (default: 4MB to prevent OOM)


#### `impl Default for Configuration`  <sub>(impl)</sub>

```rust
impl Default for Configuration {
```

#### `default`  <sub>(fn)</sub>

> Builds the default Configuration for feroxbuster

```rust
fn default() -> Self {
```

#### `impl Configuration`  <sub>(impl)</sub>

```rust
impl Configuration {
```

#### `new`  <sub>(fn)</sub>

> Creates a [Configuration](struct.Configuration.html) object with the following  
> built-in default values  
>   
> - **timeout**: `5` seconds  
> - **redirects**: `false`  
> - **extract_links**: `true`  
> - **wordlist**: [`DEFAULT_WORDLIST`](constant.DEFAULT_WORDLIST.html)  
> - **config**: `None`  
> - **threads**: `50`  
> - **timeout**: `7` seconds  
> - **verbosity**: `0` (no logging enabled)  
> - **proxy**: `None`  
> - **status_codes**: [`DEFAULT_RESPONSE_CODES`](constant.DEFAULT_RESPONSE_CODES.html)  
> - **filter_status**: `None`  
> - **output**: `None` (print to stdout)  
> - **debug_log**: `None`  
> - **quiet**: `false`  
> - **silent**: `false`  
> - **auto_tune**: `false`  
> - **auto_bail**: `false`  
> - **save_state**: `true`  
> - **user_agent**: `feroxbuster/VERSION`  
> - **random_agent**: `false`  
> - **insecure**: `false` (don't be insecure, i.e. don't allow invalid certs)  
> - **extensions**: `None`  
> - **collect_extensions**: `false`  
> - **collect_backups**: `false`  
> - **backup_extensions**: [`DEFAULT_BACKUP_EXTENSIONS`](constant.DEFAULT_BACKUP_EXTENSIONS.html)  
> - **collect_words**: `false`  
> - **dont_collect**: [`DEFAULT_IGNORED_EXTENSIONS`](constant.DEFAULT_RESPONSE_CODES.html)  
> - **methods**: [`DEFAULT_METHOD`](constant.DEFAULT_METHOD.html)  
> - **data**: `None`  
> - **url_denylist**: `None`  
> - **regex_denylist**: `None`  
> - **scope**: `None`  
> - **filter_size**: `None`  
> - **filter_similar**: `None`  
> - **filter_regex**: `None`  
> - **filter_word_count**: `None`  
> - **filter_line_count**: `None`  
> - **headers**: `None`  
> - **queries**: `None`  
> - **no_recursion**: `false` (recursively scan enumerated sub-directories)  
> - **add_slash**: `false`  
> - **stdin**: `false`  
> - **json**: `false`  
> - **dont_filter**: `false` (auto filter wildcard responses)  
> - **depth**: `4` (maximum recursion depth)  
> - **force_recursion**: `false` (still respects recursion depth)  
> - **scan_limit**: `0` (no limit on concurrent scans imposed)  
> - **limit_bars**: `0` (no limit on number of directory scan bars shown)  
> - **parallel**: `0` (no limit on parallel scans imposed)  
> - **rate_limit**: `0` (no limit on requests per second imposed)  
> - **time_limit**: `None` (no limit on length of scan imposed)  
> - **replay_proxy**: `None` (no limit on concurrent scans imposed)  
> - **replay_codes**: [`DEFAULT_RESPONSE_CODES`](constant.DEFAULT_RESPONSE_CODES.html)  
> - **update_app**: `false`  
> - **scan_dir_listings**: `false`  
> - **request_file**: `None`  
> - **protocol**: `https`  
> - **unique**: `false`  
>   
> After which, any values defined in a  
> [ferox-config.toml](constant.DEFAULT_CONFIG_NAME.html) config file will override the  
> built-in defaults.  
>   
> `ferox-config.toml` can be placed in any of the following locations (in the order shown):  
> - `/etc/feroxbuster/`  
> - `CONFIG_DIR/ferxobuster/`  
> - The same directory as the `feroxbuster` executable  
> - The user's current working directory  
>   
> If more than one valid configuration file is found, each one overwrites the values found previously.  
>   
> Finally, any options/arguments given on the commandline will override both built-in and  
> config-file specified values.  
>   
> The resulting [Configuration](struct.Configuration.html) is a singleton with a `static`  
> lifetime.

```rust
pub fn new() -> Result<Self> {
```

#### `parse_config_files`  <sub>(fn)</sub>

> Parse all possible versions of the ferox-config.toml file, adhering to the order of  
> precedence outlined above

```rust
fn parse_config_files(config: &mut Self) -> Result<()> {
```

#### `parse_cli_args`  <sub>(fn)</sub>

> Given a set of ArgMatches read from the CLI, update and return the default Configuration  
> settings

```rust
fn parse_cli_args(args: &ArgMatches) -> Self {
```

#### `parse_url_with_no_base_correction`  <sub>(fn)</sub>

> internal helper to parse both scope urls and target urls

```rust
fn parse_url_with_no_base_correction(
    config: &Configuration,
    url: &str,
) -> Result<Url, url::ParseError> {
```

#### `try_rebuild_clients`  <sub>(fn)</sub>

> this function determines if we've gotten a Client configuration change from  
> either the config file or command line arguments; if we have, we need to rebuild  
> the client and store it in the config struct

```rust
fn try_rebuild_clients(configuration: &mut Configuration) {
```

#### `parse_and_merge_config`  <sub>(fn)</sub>

> Given a configuration file's location and an instance of `Configuration`, read in  
> the config file if found and update the current settings with the settings found therein

```rust
fn parse_and_merge_config(config_file: PathBuf, config: &mut Self) -> Result<()> {
```

#### `merge_config`  <sub>(fn)</sub>

> Given two Configurations, overwrite `settings` with the fields found in `settings_to_merge`

```rust
fn merge_config(conf: &mut Self, new: Self) {
```

#### `parse_config`  <sub>(fn)</sub>

> If present, read in `DEFAULT_CONFIG_NAME` and deserialize the specified values  
>   
> uses serde to deserialize the toml into a `Configuration` struct

```rust
pub(super) fn parse_config(config_file: PathBuf) -> Result<Self> {
```

#### `parse_data_arg`  <sub>(fn)</sub>

> Reads payload body from STDIN or file system depending on '@' and  
>   
> sets config.data according to the body's content type

```rust
fn parse_data_arg(&mut self, arg: &str, content_type: Option<ContentType>) {
```

#### `impl FeroxSerialize for Configuration`  <sub>(impl)</sub>

> Implementation of FeroxMessage

```rust
impl FeroxSerialize for Configuration {
```

#### `as_str`  <sub>(fn)</sub>

> Simple wrapper around create_report_string

```rust
fn as_str(&self) -> String {
```

#### `as_json`  <sub>(fn)</sub>

> Create an NDJSON representation of the current scan's Configuration  
>   
> (expanded for clarity)  
> ex:  
> {  
> "type":"configuration",  
> "wordlist":"test",  
> "config":"/home/epi/.config/feroxbuster/ferox-config.toml",  
> "proxy":"",  
> "replay_proxy":"",  
> "target_url":"https://localhost.com",  
> "status_codes":[  
> 200,  
> 204,  
> 301,  
> 302,  
> 307,  
> 308,  
> 401,  
> 403,  
> 405  
> ],  
> ...  
> }\n

```rust
fn as_json(&self) -> Result<String> {
```

### `src/config/utils.rs`


#### `report_and_exit`  <sub>(fn)</sub>

> simple helper to clean up some code reuse below; panics under test / exits in prod

```rust
pub(super) fn report_and_exit(err: &str) -> ! {
```

#### `serialized_type`  <sub>(fn)</sub>

> default Configuration type for use in json output

```rust
pub(super) fn serialized_type() -> String {
```

#### `timeout`  <sub>(fn)</sub>

> default timeout value

```rust
pub(super) fn timeout() -> u64 {
```

#### `save_state`  <sub>(fn)</sub>

> default save_state value

```rust
pub(super) fn save_state() -> bool {
```

#### `threads`  <sub>(fn)</sub>

> default threads value

```rust
pub(super) fn threads() -> usize {
```

#### `request_protocol`  <sub>(fn)</sub>

> default protocol value

```rust
pub(super) fn request_protocol() -> String {
```

#### `status_codes`  <sub>(fn)</sub>

> default status codes

```rust
pub(super) fn status_codes() -> Vec<u16> {
```

#### `methods`  <sub>(fn)</sub>

> default HTTP Method

```rust
pub(super) fn methods() -> Vec<String> {
```

#### `ignored_extensions`  <sub>(fn)</sub>

> default extensions to ignore while auto-collecting

```rust
pub(super) fn ignored_extensions() -> Vec<String> {
```

#### `backup_extensions`  <sub>(fn)</sub>

> default backup extensions to collect

```rust
pub(super) fn backup_extensions() -> Vec<String> {
```

#### `wordlist`  <sub>(fn)</sub>

> default wordlist

```rust
pub(super) fn wordlist() -> Vec<String> {
```

#### `deserialize_wordlist`  <sub>(fn)</sub>

> custom serde deserializer that accepts either a single string or a list of strings, producing  
> a `Vec<String>`. Used for the `wordlist` field so that legacy state files / config files that  
> store a single string still round-trip correctly.

```rust
pub(super) fn deserialize_wordlist<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: serde::Deserializer<'de>,
    {
```

#### `StringOrVec`  <sub>(struct)</sub>

```rust
struct StringOrVec;
```

#### `impl<'de> Visitor<'de> for StringOrVec`  <sub>(impl)</sub>

```rust
impl<'de> Visitor<'de> for StringOrVec {
```

#### `Value`  <sub>(type)</sub>

```rust
type Value = Vec<String>;
```

#### `expecting`  <sub>(fn)</sub>

```rust
fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
```

#### `visit_str`  <sub>(fn)</sub>

```rust
fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
where
    E: de::Error,
    {
```

#### `visit_string`  <sub>(fn)</sub>

```rust
fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
where
    E: de::Error,
    {
```

#### `visit_seq`  <sub>(fn)</sub>

```rust
fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
where
    A: de::SeqAccess<'de>,
    {
```

#### `user_agent`  <sub>(fn)</sub>

> default user-agent

```rust
pub(super) fn user_agent() -> String {
```

#### `depth`  <sub>(fn)</sub>

> default recursion depth

```rust
pub(super) fn depth() -> usize {
```

#### `extract_links`  <sub>(fn)</sub>

> default extract links

```rust
pub(super) fn extract_links() -> bool {
```

#### `response_size_limit`  <sub>(fn)</sub>

> default max response size to read (4MB to prevent OOM issues)

```rust
pub(super) fn response_size_limit() -> usize {
```

#### `OutputLevel`  <sub>(enum)</sub>

> enum representing the three possible states for informational output (not logging verbosity)

```rust
pub enum OutputLevel {
```

**Variants:**
- `Default,` — normal scan, no --quiet|--silent
- `Quiet,` — quiet scan, print some information, but not all (new in versions >= 2.0.0)
- `Silent,` — silent scan, only print urls (used to be --quiet in versions 1.x.x)
- `SilentJSON,` — silent scan, but with JSON output


#### `impl Default for OutputLevel`  <sub>(impl)</sub>

> implement a default for OutputLevel

```rust
impl Default for OutputLevel {
```

#### `default`  <sub>(fn)</sub>

> return Default

```rust
fn default() -> Self {
```

#### `determine_output_level`  <sub>(fn)</sub>

> given the current settings for quiet and silent, determine output_level (DRY helper)

```rust
pub fn determine_output_level(quiet: bool, silent: bool, json: bool) -> OutputLevel {
```

#### `RequesterPolicy`  <sub>(enum)</sub>

> represents actions the Requester should take in certain situations

```rust
pub enum RequesterPolicy {
```

**Variants:**
- `AutoTune,` — automatically try to lower request rate in order to reduce errors
- `AutoBail,` — automatically bail at certain error thresholds
- `Default,` — just let that junk run super natural


#### `impl Default for RequesterPolicy`  <sub>(impl)</sub>

> default implementation for RequesterPolicy

```rust
impl Default for RequesterPolicy {
```

#### `default`  <sub>(fn)</sub>

> Default as default

```rust
fn default() -> Self {
```

#### `determine_requester_policy`  <sub>(fn)</sub>

> given the current settings for quiet and silent, determine output_level (DRY helper)

```rust
pub fn determine_requester_policy(auto_tune: bool, auto_bail: bool) -> RequesterPolicy {
```

#### `split_query`  <sub>(fn)</sub>

> Splits a query string into a key-value pair.  
>   
> This function takes a query string in the format of `"key=value"` and splits it into  
> a tuple containing the key and value as separate strings. If the query string is  
> malformed (e.g., empty or without a key), it returns an error.  
>   
> # Arguments  
>   
> * `query` - A string slice that holds the query string to be split.  
>   
> # Returns  
>   
> * `Result<(String, String)>` - A tuple containing the key and value as `String`s,  
> or an error if the input is invalid.  
>   
> # Errors  
>   
> This function will return an error if:  
> * The input string is empty or equal to `"="`.  
> * The key part of the query string is empty (i.e., if the string starts with `"="`).

```rust
pub fn split_query(query: &str) -> Result<(String, String)> {
```

#### `split_header`  <sub>(fn)</sub>

> Splits an HTTP header string into a key-value pair.  
>   
> This function takes a header string in the format of `"Key: Value"` and splits it into  
> a tuple containing the key and value as separate strings. If the header string is  
> malformed (e.g., empty or missing a key), it returns an error.  
>   
> # Arguments  
>   
> * `header` - A string slice that holds the header string to be split.  
>   
> # Returns  
>   
> * `Result<(String, String)>` - A tuple containing the key and value as `String`s,  
> or an error if the input is invalid.  
>   
> # Errors  
>   
> This function will return an error if:  
> * The input string is empty.  
> * The key part of the header string is empty (i.e., if the string starts with `":"`).

```rust
pub fn split_header(header: &str) -> Result<(String, String)> {
```

#### `combine_cookies`  <sub>(fn)</sub>

> Combines two `Cookie` header strings into a single, unified `Cookie` header string.  
>   
> The function parses both input strings into individual key-value pairs, ensuring that each  
> key is unique. If a key appears in both input strings, the value from the second string  
> will override the value from the first string. The resulting combined `Cookie` header string  
> is returned with all key-value pairs separated by `;`.  
>   
> # Arguments  
>   
> * `cookie1` - A string slice representing the first `Cookie` header.  
> * `cookie2` - A string slice representing the second `Cookie` header.  
>   
> # Returns  
>   
> * A `String` containing the combined `Cookie` header with unique keys.  
>   
> The output string will contain all unique keys from both input strings, with the value  
> from the second string taking precedence in the case of key collisions.

```rust
pub fn combine_cookies(cookie1: &str, cookie2: &str) -> String {
```

#### `ContentType`  <sub>(enum)</sub>

> Content Types enumeration (to be complete as more header values  
> are needed)

```rust
pub enum ContentType {
```

**Variants:**
- `Json,`
- `UrlEncoded,`


#### `impl ContentType`  <sub>(impl)</sub>

> to_header_value() produces the value of the CONTENT-TYPE  
> header for each ContentType. Ideally, new content type headers  
> should be added and produced from here

```rust
impl ContentType {
```

#### `to_header_value`  <sub>(fn)</sub>

```rust
pub fn to_header_value(self: ContentType) -> String {
```

#### `parse_request_file`  <sub>(fn)</sub>

> Parses a raw HTTP request from a file and updates the provided configuration.  
>   
> This function reads an HTTP request from the file specified by `config.request_file`,  
> parses the request line, headers, and body, and updates the `config` object  
> with the parsed values. If certain elements (e.g., headers or body) are  
> already provided via the CLI, they take precedence over the parsed values.  
>   
> # Arguments  
>   
> * `config` - A mutable reference to a `Configuration` object that will be  
> updated with the parsed request data.  
>   
> # Returns  
>   
> * `Result<()>` - Returns `Ok(())` if parsing and configuration updates  
> were successful, or an error if the raw file or request is invalid.  
>   
> # Errors  
>   
> This function will return an error if:  
> * The file specified in `config.request_file` is empty.  
> * The request is malformed (e.g., missing the request line, method, or URI).  
> * Required headers are missing (e.g., `Host` when the request line URI is not a full URL).  
>   
> # Details  
>   
> * The request body is only set if it hasn't been overridden by the CLI options.  
> * The request line method is added to `config.methods` if it's not already present.  
> * Headers from the raw request are added to `config.headers`, unless overridden  
> by CLI options. Special handling is applied to `User-Agent`, `Content-Length`,  
> and `Cookie` headers.  
> * The request URI is validated and parsed. If it's not a full URL, it will be  
> combined with the `Host` header to form a full target URL.  
> * Query parameters are extracted from the URI and added to `config.queries`,  
> unless overridden by CLI options.  
> 

```rust
pub fn parse_request_file(config: &mut Configuration) -> Result<()> {
```

#### `preconfig_log`  <sub>(fn)</sub>

> Log configuration operations before main logger instantiation  
>   
> Since logging depends on config (e.g. '-vv' parsing), to log  
> conf related operations, we assemble here FeroxMessage to  
> remain iso with the rest of the app and display them on STDOUT  
>   
> # Arguments:  
>   
> * `level` - Log level of the event  
> * `message` - message to be displayed  
> 

```rust
pub fn preconfig_log(level: LevelFilter, message: String) {
```

#### `TempSetup`  <sub>(struct)</sub>

```rust
struct TempSetup {
```

**Fields:**
- `pub path: PathBuf,`
- `pub config: Configuration,`
- `pub file: File,`


#### `impl TempSetup`  <sub>(impl)</sub>

```rust
impl TempSetup {
```

#### `new`  <sub>(fn)</sub>

```rust
pub fn new() -> Self {
```

#### `cleanup`  <sub>(fn)</sub>

```rust
pub fn cleanup(self) {
```

#### `determine_output_level_returns_correct_results`  <sub>(fn)</sub>

> test determine_output_level returns higher of the two levels if both given values are true

```rust
fn determine_output_level_returns_correct_results() {
```

#### `determine_requester_policy_returns_correct_results`  <sub>(fn)</sub>

> test determine_requester_policy returns higher of the two levels if both given values are true

```rust
fn determine_requester_policy_returns_correct_results() {
```

#### `report_and_exit_panics_under_test`  <sub>(fn)</sub>

> report_and_exit should panic/exit when called

```rust
fn report_and_exit_panics_under_test() {
```

#### `test_split_query_simple`  <sub>(fn)</sub>

```rust
fn test_split_query_simple() {
```

#### `test_split_query_with_spaces`  <sub>(fn)</sub>

```rust
fn test_split_query_with_spaces() {
```

#### `test_split_query_empty_value`  <sub>(fn)</sub>

```rust
fn test_split_query_empty_value() {
```

#### `test_split_query_no_value`  <sub>(fn)</sub>

```rust
fn test_split_query_no_value() {
```

#### `test_split_query_multiple_equals`  <sub>(fn)</sub>

```rust
fn test_split_query_multiple_equals() {
```

#### `test_split_query_empty_key_and_value`  <sub>(fn)</sub>

```rust
fn test_split_query_empty_key_and_value() {
```

#### `test_split_query_empty_key`  <sub>(fn)</sub>

```rust
fn test_split_query_empty_key() {
```

#### `test_split_query_trailing_equals_in_value`  <sub>(fn)</sub>

```rust
fn test_split_query_trailing_equals_in_value() {
```

#### `test_split_query_no_equals`  <sub>(fn)</sub>

```rust
fn test_split_query_no_equals() {
```

#### `test_split_query_empty_input`  <sub>(fn)</sub>

```rust
fn test_split_query_empty_input() {
```

#### `test_split_header_simple`  <sub>(fn)</sub>

```rust
fn test_split_header_simple() -> Result<()> {
```

#### `test_split_header_with_leading_space_in_value`  <sub>(fn)</sub>

```rust
fn test_split_header_with_leading_space_in_value() -> Result<()> {
```

#### `test_split_header_with_trimmed_leading_space`  <sub>(fn)</sub>

```rust
fn test_split_header_with_trimmed_leading_space() -> Result<()> {
```

#### `test_split_header_with_multiple_colons`  <sub>(fn)</sub>

```rust
fn test_split_header_with_multiple_colons() -> Result<()> {
```

#### `test_split_header_empty_value`  <sub>(fn)</sub>

```rust
fn test_split_header_empty_value() -> Result<()> {
```

#### `test_split_header_no_value`  <sub>(fn)</sub>

```rust
fn test_split_header_no_value() -> Result<()> {
```

#### `test_split_header_no_colon`  <sub>(fn)</sub>

```rust
fn test_split_header_no_colon() -> Result<()> {
```

#### `test_split_header_empty_key`  <sub>(fn)</sub>

```rust
fn test_split_header_empty_key() {
```

#### `test_split_header_empty_key_and_value`  <sub>(fn)</sub>

```rust
fn test_split_header_empty_key_and_value() {
```

#### `test_split_header_empty_input`  <sub>(fn)</sub>

```rust
fn test_split_header_empty_input() {
```

#### `test_split_header_value_with_leading_single_space`  <sub>(fn)</sub>

```rust
fn test_split_header_value_with_leading_single_space() -> Result<()> {
```

#### `test_split_header_value_with_leading_multiple_spaces`  <sub>(fn)</sub>

```rust
fn test_split_header_value_with_leading_multiple_spaces() -> Result<()> {
```

#### `test_parse_raw_with_empty_request`  <sub>(fn)</sub>

```rust
fn test_parse_raw_with_empty_request() {
```

#### `test_parse_raw_with_empty_file`  <sub>(fn)</sub>

```rust
fn test_parse_raw_with_empty_file() -> io::Result<()> {
```

#### `test_parse_raw_without_head_body_crlf`  <sub>(fn)</sub>

```rust
fn test_parse_raw_without_head_body_crlf() -> io::Result<()> {
```

#### `test_parse_raw_with_only_head_body_crlf`  <sub>(fn)</sub>

```rust
fn test_parse_raw_with_only_head_body_crlf() -> io::Result<()> {
```

#### `test_parse_raw_body_is_overridden_by_cli`  <sub>(fn)</sub>

```rust
fn test_parse_raw_body_is_overridden_by_cli() -> io::Result<()> {
```

#### `test_parse_raw_with_empty_request_line`  <sub>(fn)</sub>

```rust
fn test_parse_raw_with_empty_request_line() -> io::Result<()> {
```

#### `test_parse_raw_with_missing_uri`  <sub>(fn)</sub>

```rust
fn test_parse_raw_with_missing_uri() -> io::Result<()> {
```

#### `test_parse_raw_with_missing_method`  <sub>(fn)</sub>

```rust
fn test_parse_raw_with_missing_method() -> io::Result<()> {
```

#### `test_parse_raw_methods_are_appended_if_unique`  <sub>(fn)</sub>

```rust
fn test_parse_raw_methods_are_appended_if_unique() -> io::Result<()> {
```

#### `test_parse_raw_methods_are_ignored_if_already_present_from_cli`  <sub>(fn)</sub>

```rust
fn test_parse_raw_methods_are_ignored_if_already_present_from_cli() -> io::Result<()> {
```

#### `test_parse_raw_headers_added_to_config_if_missing_else_overridden_from_cli`  <sub>(fn)</sub>

```rust
fn test_parse_raw_headers_added_to_config_if_missing_else_overridden_from_cli() -> io::Result<()>
    {
```

#### `test_parse_raw_with_user_agent_in_request`  <sub>(fn)</sub>

```rust
fn test_parse_raw_with_user_agent_in_request() -> io::Result<()> {
```

#### `test_parse_raw_with_user_agent_in_request_and_cli`  <sub>(fn)</sub>

```rust
fn test_parse_raw_with_user_agent_in_request_and_cli() -> io::Result<()> {
```

#### `test_parse_raw_content_length_is_always_skipped`  <sub>(fn)</sub>

```rust
fn test_parse_raw_content_length_is_always_skipped() -> io::Result<()> {
```

#### `test_parse_raw_cookie_header_appended_or_overridden`  <sub>(fn)</sub>

```rust
fn test_parse_raw_cookie_header_appended_or_overridden() -> io::Result<()> {
```

#### `test_parse_raw_with_relative_path_and_partial_host_header`  <sub>(fn)</sub>

```rust
fn test_parse_raw_with_relative_path_and_partial_host_header() -> io::Result<()> {
```

#### `test_parse_raw_with_relative_path_and_no_host_header`  <sub>(fn)</sub>

```rust
fn test_parse_raw_with_relative_path_and_no_host_header() -> io::Result<()> {
```

#### `test_parse_raw_with_full_url_and_no_host_header`  <sub>(fn)</sub>

```rust
fn test_parse_raw_with_full_url_and_no_host_header() -> io::Result<()> {
```

#### `test_parse_raw_with_full_url_and_host_header`  <sub>(fn)</sub>

```rust
fn test_parse_raw_with_full_url_and_host_header() -> io::Result<()> {
```

#### `test_parse_raw_with_partial_url_and_queries`  <sub>(fn)</sub>

```rust
fn test_parse_raw_with_partial_url_and_queries() -> io::Result<()> {
```

#### `test_parse_raw_with_full_url_and_queries`  <sub>(fn)</sub>

```rust
fn test_parse_raw_with_full_url_and_queries() -> io::Result<()> {
```

#### `test_combine_cookies`  <sub>(fn)</sub>

```rust
fn test_combine_cookies() {
```

#### `test_split_header`  <sub>(fn)</sub>

```rust
fn test_split_header() {
```

#### `test_split_query`  <sub>(fn)</sub>

```rust
fn test_split_query() {
```

#### `test_parse_raw_lf_only_request`  <sub>(fn)</sub>

```rust
fn test_parse_raw_lf_only_request() -> io::Result<()> {
```

#### `test_parse_raw_crlf_request`  <sub>(fn)</sub>

```rust
fn test_parse_raw_crlf_request() -> io::Result<()> {
```

#### `test_parse_raw_crlf_body_preserved`  <sub>(fn)</sub>

```rust
fn test_parse_raw_crlf_body_preserved() -> io::Result<()> {
```

#### `test_parse_raw_lf_headers_crlf_body`  <sub>(fn)</sub>

```rust
fn test_parse_raw_lf_headers_crlf_body() -> io::Result<()> {
```

#### `test_parse_raw_mixed_newlines_headers`  <sub>(fn)</sub>

```rust
fn test_parse_raw_mixed_newlines_headers() -> io::Result<()> {
```

#### `test_parse_raw_binary_body_preserved`  <sub>(fn)</sub>

```rust
fn test_parse_raw_binary_body_preserved() -> io::Result<()> {
```

#### `test_parse_raw_lf_headers_with_crlf_crlf_in_body`  <sub>(fn)</sub>

```rust
fn test_parse_raw_lf_headers_with_crlf_crlf_in_body() -> io::Result<()> {
```

#### `test_parse_raw_crlf_headers_with_lf_lf_in_body`  <sub>(fn)</sub>

```rust
fn test_parse_raw_crlf_headers_with_lf_lf_in_body() -> io::Result<()> {
```



## Module `event_handlers` <a id='module-event-handlers'></a>

*82 documented items*


### `src/event_handlers/command.rs`


#### `Command`  <sub>(enum)</sub>

> Protocol definition for updating an event handler via mpsc

```rust
pub enum Command {
```

**Variants:**
- `AddRequest,` — Add one to the total number of requests
- `AddError(StatError),` — Add one to the proper field(s) based on the given `StatError`
- `AddStatus(StatusCode),` — Add one to the proper field(s) based on the given `StatusCode`
- `CreateBar(u64),` — the u64 value is the offset at which to start the progress bar (can be 0)
- `AddToUsizeField(StatField, usize),` — Add to a `Stats` field that corresponds to the given `StatField` by the given `usize` value
- `SubtractFromUsizeField(StatField, usize),` — Subtract from a `Stats` field that corresponds to the given `StatField` by the given `usize` value
- `AddToF64Field(StatField, f64),` — Update a `Stats` field that corresponds to the given `StatField` by the given `f64` value
- `Save,` — Save a `Stats` object to disk using `reporter::get_cached_file_handle`
- `LoadStats(String),` — Load a `Stats` object from disk
- `AddFilter(Box<dyn FeroxFilter>),` — Add a `FeroxFilter` implementor to `FilterHandler`'s instance of `FeroxFilters`
- `RemoveFilters(Vec<usize>),` — Remove a set of `FeroxFilter` implementors from `FeroxFilters` by index
- `Report(Box<FeroxResponse>),` — Send a `FeroxResponse` to the output handler for reporting
- `ScanInitialUrls(Vec<String>),` — Send a group of urls to be scanned (only used for the urls passed in explicitly by the user)
- `ScanNewUrl(String),` — Send a single url to be scanned (presumably added from the interactive menu)
- `TryRecursion(Box<FeroxResponse>),` — Determine whether or not recursion is appropriate, given a FeroxResponse, if so start a scan
- `UpdateWordlist(Arc<Vec<String>>),` — Send a pointer to the wordlist to the recursion handler
- `JoinTasks(Sender<bool>),` — Instruct the ScanHandler to join on all known scans, use sender to notify main when done
- `Ping,` — Command used to test that a spawned task succeeded in initialization
- `Sync(Sender<bool>),` — Just receive a sender and reply, used for slowing down the main thread
- `AddDiscoveredExtension(String),` — Notify event handler that a new extension has been seen
- `WriteToDisk(Box<FeroxMessage>),` — Write an arbitrary string to disk
- `Exit,` — Break out of the (infinite) mpsc receive loop
- `AddHandles(Arc<Handles>),` — already been initialized
- `UpdateTargets(Vec<String>),` — inform the Stats object about which targets are being scanned
- `QueryOverallBarEta(Sender<Duration>),` — query the Stats handler about the position of the overall progress bar
- `AddScanPermits(usize),` — Add permits to the scan limiter (semaphore)
- `SubtractScanPermits(usize),` — Subtract permits from the scan limiter (semaphore)


### `src/event_handlers/container.rs`


#### `Tasks`  <sub>(struct)</sub>

> Simple container for multiple JoinHandles

```rust
pub struct Tasks {
```

**Fields:**
- `pub terminal: Joiner,` — JoinHandle for terminal handler
- `pub stats: Joiner,` — JoinHandle for statistics handler
- `pub filters: Joiner,` — JoinHandle for filters handler
- `pub scans: Joiner,` — JoinHandle for scans handler


#### `impl Tasks`  <sub>(impl)</sub>

> Tasks implementation

```rust
impl Tasks {
```

#### `new`  <sub>(fn)</sub>

> Given JoinHandles for terminal, statistics, and filters create a new Tasks object

```rust
pub fn new(terminal: Joiner, stats: Joiner, filters: Joiner, scans: Joiner) -> Self {
```

#### `Handles`  <sub>(struct)</sub>

> Container for the different *Handles that will be shared across modules

```rust
pub struct Handles {
```

**Fields:**
- `pub stats: StatsHandle,` — Handle for statistics
- `pub filters: FiltersHandle,` — Handle for filters
- `pub output: TermOutHandle,` — Handle for output (terminal/file)
- `pub config: Arc<Configuration>,` — Handle for Configuration
- `pub scans: RwLock<Option<ScanHandle>>,` — Handle for recursion
- `pub wordlist: Arc<Vec<String>>,` — Pointer to the list of words generated from reading in the wordlist


#### `impl Handles`  <sub>(impl)</sub>

> implementation of Handles

```rust
impl Handles {
```

#### `new`  <sub>(fn)</sub>

> Given a StatsHandle, FiltersHandle, and OutputHandle, create a Handles object

```rust
pub fn new(
    stats: StatsHandle,
    filters: FiltersHandle,
    output: TermOutHandle,
    config: Arc<Configuration>,
    wordlist: Arc<Vec<String>>,
) -> Self {
```

#### `for_testing`  <sub>(fn)</sub>

> create a Handles object suitable for unit testing (non-functional)

```rust
pub fn for_testing(
    scanned_urls: Option<Arc<FeroxScans>>,
    config: Option<Arc<Configuration>>,
) -> (Self, UnboundedReceiver<Command>) {
```

#### `set_scan_handle`  <sub>(fn)</sub>

> Set the ScanHandle object

```rust
pub fn set_scan_handle(&self, handle: ScanHandle) {
```

#### `send_scan_command`  <sub>(fn)</sub>

> Helper to easily send a Command over the (locked) underlying CommandSender object

```rust
pub fn send_scan_command(&self, command: Command) -> Result<()> {
```

#### `num_collected_extensions`  <sub>(fn)</sub>

> wrapper to reach into `FeroxScans` and yank out the length of `collected_extensions`

```rust
pub fn num_collected_extensions(&self) -> usize {
```

#### `collected_extensions`  <sub>(fn)</sub>

> wrapper to reach into `FeroxScans` and yank out the length of `collected_extensions`

```rust
pub fn collected_extensions(&self) -> HashSet<String> {
```

#### `expected_num_requests_per_dir`  <sub>(fn)</sub>

> number of words in the wordlist, multiplied by `expected_num_requests_multiplier`

```rust
pub fn expected_num_requests_per_dir(&self) -> usize {
```

#### `expected_num_requests_multiplier`  <sub>(fn)</sub>

> estimate of HTTP requests per word = (base + static extensions + collected extensions)  
> multiplied by the number of request methods

```rust
pub fn expected_num_requests_multiplier(&self) -> usize {
```

#### `ferox_scans`  <sub>(fn)</sub>

> Helper to easily get the (locked) underlying FeroxScans object

```rust
pub fn ferox_scans(&self) -> Result<Arc<FeroxScans>> {
```

### `src/event_handlers/filters.rs`


#### `FiltersHandle`  <sub>(struct)</sub>

> Container for filters transmitter and FeroxFilters object

```rust
pub struct FiltersHandle {
```

**Fields:**
- `pub data: Arc<FeroxFilters>,` — FeroxFilters object used across modules to track active filters
- `pub tx: CommandSender,` — transmitter used to update `data`


#### `impl FiltersHandle`  <sub>(impl)</sub>

> implementation of FiltersHandle

```rust
impl FiltersHandle {
```

#### `new`  <sub>(fn)</sub>

> Given an Arc-wrapped FeroxFilters and CommandSender, create a new FiltersHandle

```rust
pub fn new(data: Arc<FeroxFilters>, tx: CommandSender) -> Self {
```

#### `send`  <sub>(fn)</sub>

> Send the given Command over `tx`

```rust
pub fn send(&self, command: Command) -> Result<()> {
```

#### `sync`  <sub>(fn)</sub>

> Sync the handle with the handler

```rust
pub async fn sync(&self) -> Result<()> {
```

#### `FiltersHandler`  <sub>(struct)</sub>

> event handler for updating a single data structure of all active filters

```rust
pub struct FiltersHandler {
```

**Fields:**
- `data: Arc<FeroxFilters>,` — collection of FeroxFilters
- `receiver: UnboundedReceiver<Command>,` — Receiver half of mpsc from which `Command`s are processed


#### `impl FiltersHandler`  <sub>(impl)</sub>

> implementation of event handler for filters

```rust
impl FiltersHandler {
```

#### `new`  <sub>(fn)</sub>

> create new event handler

```rust
pub fn new(data: Arc<FeroxFilters>, receiver: UnboundedReceiver<Command>) -> Self {
```

#### `initialize`  <sub>(fn)</sub>

> Initialize new `FeroxFilters` and the sc side of an mpsc channel that is responsible for  
> updates to the aforementioned object.

```rust
pub fn initialize() -> (Joiner, FiltersHandle) {
```

#### `start`  <sub>(fn)</sub>

> Start a single consumer task (sc side of mpsc)  
>   
> The consumer simply receives `Command` and acts accordingly

```rust
pub async fn start(&mut self) -> Result<()> {
```

#### `empty_filter_skipped`  <sub>(fn)</sub>

```rust
async fn empty_filter_skipped() {
```

### `src/event_handlers/inputs.rs`


#### `SCAN_COMPLETE`  <sub>(static)</sub>

> Atomic boolean flag, used to determine whether or not the terminal input handler should exit

```rust
pub static SCAN_COMPLETE: AtomicBool = AtomicBool::new(false);
```

#### `TermInputHandler`  <sub>(struct)</sub>

> Container for filters transmitter and FeroxFilters object

```rust
pub struct TermInputHandler {
```

**Fields:**
- `handles: Arc<Handles>,` — handles to other handlers


#### `impl TermInputHandler`  <sub>(impl)</sub>

> implementation of event handler for terminal input  
>   
> kicks off the following handlers related to terminal input:  
> ctrl+c handler that saves scan state to disk  
> enter handler that listens for enter during scans to drop into interactive scan management menu

```rust
impl TermInputHandler {
```

#### `new`  <sub>(fn)</sub>

> Create new event handler

```rust
pub fn new(handles: Arc<Handles>) -> Self {
```

#### `initialize`  <sub>(fn)</sub>

> Initialize the sigint and enter handlers that are responsible for handling initial user  
> interaction during scans

```rust
pub fn initialize(handles: Arc<Handles>) {
```

#### `start`  <sub>(fn)</sub>

> wrapper around sigint_handler and enter_handler

```rust
fn start(&self) {
```

#### `sigint_handler`  <sub>(fn)</sub>

> Writes the current state of the program to disk (if save_state is true) and then exits

```rust
pub fn sigint_handler(handles: Arc<Handles>) -> Result<()> {
```

#### `enter_handler`  <sub>(fn)</sub>

> Handles specific key events triggered by the user over stdin

```rust
fn enter_handler() {
```

### `src/event_handlers/outputs.rs`


#### `ProcessResponseCall`  <sub>(enum)</sub>

> Simple enum for semantic clarity around calling expectations for `process_response`

```rust
enum ProcessResponseCall {
```

**Variants:**
- `Recursive,` — call should allow recursion
- `NotRecursive,` — call should not allow recursion


#### `TermOutHandle`  <sub>(struct)</sub>

> Container for terminal output transmitter

```rust
pub struct TermOutHandle {
```

**Fields:**
- `pub tx: CommandSender,` — Transmitter that sends to the TermOutHandler handler
- `pub tx_file: CommandSender,` — Transmitter that sends to the FileOutHandler handler


#### `impl TermOutHandle`  <sub>(impl)</sub>

> implementation of OutputHandle

```rust
impl TermOutHandle {
```

#### `new`  <sub>(fn)</sub>

> Given a CommandSender, create a new OutputHandle

```rust
pub fn new(tx: CommandSender, tx_file: CommandSender) -> Self {
```

#### `send`  <sub>(fn)</sub>

> Send the given Command over `tx`

```rust
pub fn send(&self, command: Command) -> Result<()> {
```

#### `sync`  <sub>(fn)</sub>

> Sync the handle with the handler

```rust
pub async fn sync(&self, send_to_file: bool) -> Result<()> {
```

#### `FileOutHandler`  <sub>(struct)</sub>

> Event handler for files

```rust
pub struct FileOutHandler {
```

**Fields:**
- `receiver: CommandReceiver,` — file output handler's receiver
- `config: Arc<Configuration>,` — pointer to "global" configuration struct


#### `impl FileOutHandler`  <sub>(impl)</sub>

```rust
impl FileOutHandler {
```

#### `new`  <sub>(fn)</sub>

> Given a file tx/rx pair along with a filename and awaitable task, create  
> a FileOutHandler

```rust
fn new(rx: CommandReceiver, config: Arc<Configuration>) -> Self {
```

#### `start`  <sub>(fn)</sub>

> Spawn a single consumer task (sc side of mpsc)  
>   
> The consumer simply receives responses from the terminal handler and writes them to disk

```rust
async fn start(&mut self, tx_stats: CommandSender) -> Result<()> {
```

#### `TermOutHandler`  <sub>(struct)</sub>

> Event handler for terminal

```rust
pub struct TermOutHandler {
```

**Fields:**
- `receiver: CommandReceiver,` — terminal output handler's receiver
- `tx_file: CommandSender,` — file handler
- `file_task: Option<Joiner>,` — optional file handler task
- `config: Arc<Configuration>,` — pointer to "global" configuration struct
- `handles: Option<Arc<Handles>>,` — handles instance


#### `impl TermOutHandler`  <sub>(impl)</sub>

> implementation of TermOutHandler

```rust
impl TermOutHandler {
```

#### `new`  <sub>(fn)</sub>

> Given a terminal receiver along with a file transmitter and filename, create  
> an OutputHandler

```rust
fn new(
    receiver: CommandReceiver,
    tx_file: CommandSender,
    file_task: Option<Joiner>,
    config: Arc<Configuration>,
) -> Self {
```

#### `initialize`  <sub>(fn)</sub>

> Creates all required output handlers (terminal, file) and updates the given Handles/Tasks

```rust
pub fn initialize(
    config: Arc<Configuration>,
    tx_stats: CommandSender,
) -> (Joiner, TermOutHandle) {
```

#### `start`  <sub>(fn)</sub>

> Start a single consumer task (sc side of mpsc)  
>   
> The consumer simply receives `Command` and acts accordingly

```rust
async fn start(&mut self, tx_stats: CommandSender) -> Result<()> {
```

#### `process_response`  <sub>(fn)</sub>

> upon receiving a `FeroxResponse` from the mpsc, handle printing, sending to the replay  
> proxy, checking for backups of the `FeroxResponse`'s url, and tracking the response.

```rust
fn process_response(
    &self,
    tx_stats: CommandSender,
    mut resp: Box<FeroxResponse>,
    call_type: ProcessResponseCall,
) -> BoxFuture<'_, Result<()>> {
```

#### `add_new_url_to_vec`  <sub>(fn)</sub>

> internal helper to stay DRY

```rust
fn add_new_url_to_vec(&self, url: &Url, new_name: &str, urls: &mut Vec<Url>) {
```

#### `generate_backup_urls`  <sub>(fn)</sub>

> given a `FeroxResponse`, generate either 6 or 7 urls that are likely backups of the  
> original.  
>   
> example:  
> original: LICENSE.txt  
> backups:  
> - LICENSE.txt~  
> - LICENSE.txt.bak  
> - LICENSE.txt.bak2  
> - LICENSE.txt.old  
> - LICENSE.txt.1  
> - LICENSE.bak  
> - .LICENSE.txt.swp

```rust
async fn generate_backup_urls(&self, response: &FeroxResponse) -> Vec<Url> {
```

#### `struct_fields_of_file_out_handler`  <sub>(fn)</sub>

> try to hit struct field coverage of FileOutHandler

```rust
fn struct_fields_of_file_out_handler() {
```

#### `struct_fields_of_term_out_handler`  <sub>(fn)</sub>

> try to hit struct field coverage of TermOutHandler

```rust
async fn struct_fields_of_term_out_handler() {
```

#### `generate_backup_urls_creates_correct_urls_when_extension_present`  <sub>(fn)</sub>

> when the feroxresponse's url contains an extension, there should be 7 urls returned

```rust
async fn generate_backup_urls_creates_correct_urls_when_extension_present() {
```

#### `generate_backup_urls_creates_correct_urls_when_extension_not_present`  <sub>(fn)</sub>

> when the feroxresponse's url doesn't contain an extension, there should be 6 urls returned

```rust
async fn generate_backup_urls_creates_correct_urls_when_extension_not_present() {
```

#### `generate_backup_urls_creates_correct_urls_when_not_at_root`  <sub>(fn)</sub>

> test to ensure that backups are requested from the directory in which they were found  
> re: issue #513

```rust
async fn generate_backup_urls_creates_correct_urls_when_not_at_root() {
```

### `src/event_handlers/scans.rs`


#### `ScanHandle`  <sub>(struct)</sub>

> Container for recursion transmitter and FeroxScans object

```rust
pub struct ScanHandle {
```

**Fields:**
- `pub data: Arc<FeroxScans>,` — FeroxScans object used across modules to track scans
- `pub tx: CommandSender,` — transmitter used to update `data`


#### `impl ScanHandle`  <sub>(impl)</sub>

> implementation of RecursionHandle

```rust
impl ScanHandle {
```

#### `new`  <sub>(fn)</sub>

> Given an Arc-wrapped FeroxScans and CommandSender, create a new RecursionHandle

```rust
pub fn new(data: Arc<FeroxScans>, tx: CommandSender) -> Self {
```

#### `send`  <sub>(fn)</sub>

> Send the given Command over `tx`

```rust
pub fn send(&self, command: Command) -> Result<()> {
```

#### `ScanHandler`  <sub>(struct)</sub>

> event handler for updating a single data structure of all FeroxScans

```rust
pub struct ScanHandler {
```

**Fields:**
- `data: Arc<FeroxScans>,` — collection of FeroxScans
- `handles: Arc<Handles>,` — handles to other handlers needed to kick off a scan while already past main
- `receiver: CommandReceiver,` — Receiver half of mpsc from which `Command`s are processed
- `wordlist: std::sync::Mutex<Option<Arc<Vec<String>>>>,` — wordlist (re)used for each scan
- `tasks: Vec<Arc<FeroxScan>>,` — group of scans that need to be joined
- `max_depth: usize,` — Maximum recursion depth, a depth of 0 is infinite recursion
- `depths: Vec<(String, usize)>,` — depths associated with the initial targets provided by the user
- `limiter: Arc<DynamicSemaphore>,` — Bounded semaphore used as a barrier to limit concurrent scans


#### `impl ScanHandler`  <sub>(impl)</sub>

> implementation of event handler for filters

```rust
impl ScanHandler {
```

#### `new`  <sub>(fn)</sub>

> create new event handler

```rust
pub fn new(
    data: Arc<FeroxScans>,
    handles: Arc<Handles>,
    max_depth: usize,
    receiver: CommandReceiver,
) -> Self {
```

#### `wordlist`  <sub>(fn)</sub>

> Set the wordlist

```rust
fn wordlist(&self, wordlist: Arc<Vec<String>>) {
```

#### `initialize`  <sub>(fn)</sub>

> Initialize new `FeroxScans` and the sc side of an mpsc channel that is responsible for  
> updates to the aforementioned object.

```rust
pub fn initialize(handles: Arc<Handles>) -> (Joiner, ScanHandle) {
```

#### `start`  <sub>(fn)</sub>

> Start a single consumer task (sc side of mpsc)  
>   
> The consumer simply receives `Command` and acts accordingly

```rust
pub async fn start(&mut self) -> Result<()> {
```

#### `update_all_bar_lengths`  <sub>(fn)</sub>

> update all current and future bar lengths  
>   
> updating all bar lengths correctly requires a few different actions on our part.  
> - get the current number of requests expected per scan (dynamic when --collect-extensions  
> is used)  
> - update the overall progress bar via the statistics handler (total expected)  
> - update the expected per scan value tracked in the statistics handler  
> - update progress bars on each FeroxScan (type::directory) that are running/not-started  
> - update progress bar length on FeroxScans (this is used when creating new a FeroxScan and  
> determines the new scan's progress bar length)

```rust
fn update_all_bar_lengths(&self) -> Result<()> {
```

#### `get_wordlist`  <sub>(fn)</sub>

> Helper to easily get the (locked) underlying wordlist

```rust
pub fn get_wordlist(&self, offset: usize) -> Result<Arc<Vec<String>>> {
```

#### `ordered_scan_url`  <sub>(fn)</sub>

> wrapper around scanning a url to stay DRY

```rust
async fn ordered_scan_url(&mut self, targets: Vec<String>, order: ScanOrder) -> Result<()> {
```

#### `try_recursion`  <sub>(fn)</sub>

```rust
async fn try_recursion(&mut self, response: Box<FeroxResponse>) -> Result<()> {
```

### `src/event_handlers/statistics.rs`


#### `StatsHandle`  <sub>(struct)</sub>

> Container for statistics transmitter and Stats object

```rust
pub struct StatsHandle {
```

**Fields:**
- `pub data: Arc<Stats>,` — Stats object used across modules to track statistics
- `pub tx: CommandSender,` — transmitter used to update `data`


#### `impl StatsHandle`  <sub>(impl)</sub>

> implementation of StatsHandle

```rust
impl StatsHandle {
```

#### `new`  <sub>(fn)</sub>

> Given an Arc-wrapped Stats and CommandSender, create a new StatsHandle

```rust
pub fn new(data: Arc<Stats>, tx: CommandSender) -> Self {
```

#### `send`  <sub>(fn)</sub>

> Send the given Command over `tx`

```rust
pub fn send(&self, command: Command) -> Result<()> {
```

#### `sync`  <sub>(fn)</sub>

> Sync the handle with the handler

```rust
pub async fn sync(&self) -> Result<()> {
```

#### `StatsHandler`  <sub>(struct)</sub>

> event handler struct for updating statistics

```rust
pub struct StatsHandler {
```

**Fields:**
- `bar: ProgressBar,` — overall scan's progress bar
- `receiver: UnboundedReceiver<Command>,` — Receiver half of mpsc from which `StatCommand`s are processed
- `stats: Arc<Stats>,` — data class that stores all statistics updates


#### `impl StatsHandler`  <sub>(impl)</sub>

> implementation of event handler for statistics

```rust
impl StatsHandler {
```

#### `new`  <sub>(fn)</sub>

> create new event handler

```rust
fn new(stats: Arc<Stats>, rx_stats: UnboundedReceiver<Command>) -> Self {
```

#### `start`  <sub>(fn)</sub>

> Start a single consumer task (sc side of mpsc)  
>   
> The consumer simply receives `StatCommands` and updates the given `Stats` object as appropriate

```rust
async fn start(&mut self, output_file: &str) -> Result<()> {
```

#### `increment_bar`  <sub>(fn)</sub>

> Wrapper around incrementing the overall scan's progress bar

```rust
fn increment_bar(&self) {
```

#### `initialize`  <sub>(fn)</sub>

> Initialize new `Stats` object and the sc side of an mpsc channel that is responsible for  
> updates to the aforementioned object.

```rust
pub fn initialize(config: Arc<Configuration>) -> (Joiner, StatsHandle) {
```



## Module `scanner` <a id='module-scanner'></a>

*101 documented items*


### `src/scanner/ferox_scanner.rs`


#### `ref`  <sub>(static)</sub>

> Vector of FeroxResponse objects

```rust
pub static ref RESPONSES: FeroxResponses = FeroxResponses::default();
```

#### `check_for_user_input`  <sub>(fn)</sub>

> check to see if `pause_flag` is set to true. when true; enter a busy loop that only exits  
> by setting PAUSE_SCAN back to false

```rust
async fn check_for_user_input(
    pause_flag: &AtomicBool,
    scanned_urls: Arc<FeroxScans>,
    handles: Arc<Handles>,
    limiter: Arc<DynamicSemaphore>,
) {
```

#### `FeroxScanner`  <sub>(struct)</sub>

> handles the main muscle movement of scanning a url

```rust
pub struct FeroxScanner {
```

**Fields:**
- `pub(super) handles: Arc<Handles>,` — handles to handlers and config
- `pub(super) target_url: String,` — url that will be scanned
- `order: ScanOrder,` — found via recursion
- `wordlist: Arc<Vec<String>>,` — wordlist that's already been read from disk
- `scan_limiter: Arc<DynamicSemaphore>,` — limiter that restricts the number of active FeroxScanners


#### `impl FeroxScanner`  <sub>(impl)</sub>

> FeroxScanner implementation

```rust
impl FeroxScanner {
```

#### `new`  <sub>(fn)</sub>

> create a new FeroxScanner

```rust
pub fn new(
    target_url: &str,
    order: ScanOrder,
    wordlist: Arc<Vec<String>>,
    scan_limiter: Arc<DynamicSemaphore>,
    handles: Arc<Handles>,
) -> Self {
```

#### `stream_requests`  <sub>(fn)</sub>

> produces and awaits tasks (mp of mpsc); responsible for making requests

```rust
async fn stream_requests(
    &self,
    looping_words: Arc<Vec<String>>,
    progress_bar: ProgressBar,
    scanned_urls: Arc<FeroxScans>,
    requester: Arc<Requester>,
) {
```

#### `scan_url`  <sub>(fn)</sub>

> Scan a given url using a given wordlist  
>   
> This is the primary entrypoint for the scanner

```rust
pub async fn scan_url(&self) -> Result<()> {
```

### `src/scanner/init.rs`


#### `initialize`  <sub>(fn)</sub>

> Perform steps necessary to run scans that only need to be performed once (warming up the  
> engine, as it were)

```rust
pub async fn initialize(num_words: usize, handles: Arc<Handles>) -> Result<()> {
```

### `src/scanner/limit_heap.rs`


#### `LimitHeap`  <sub>(struct)</sub>

> bespoke variation on an array-backed max-heap  
>   
> 255 possible values generated from the initial requests/second  
>   
> when no additional errors are encountered, the left child is taken (increasing req/sec)  
> if errors have increased since the last interval, the right child is taken (decreasing req/sec)  
>   
> formula for each child:  
> - left: (|parent - current|) / 2 + current  
> - right: current - ((|parent - current|) / 2)

```rust
pub(super) struct LimitHeap {
```

**Fields:**
- `pub(super) inner: [i32; 255],` — backing array, 255 nodes == height of 7 ( 2^(h+1) -1 nodes )
- `pub(super) original: i32,` — original # of requests / second
- `pub(super) current: usize,` — current position w/in the backing array


#### `impl Default for LimitHeap`  <sub>(impl)</sub>

> default implementation of a LimitHeap

```rust
impl Default for LimitHeap {
```

#### `default`  <sub>(fn)</sub>

> zero-initialize the backing array

```rust
fn default() -> Self {
```

#### `impl Debug for LimitHeap`  <sub>(impl)</sub>

> Debug implementation of a LimitHeap

```rust
impl Debug for LimitHeap {
```

#### `fmt`  <sub>(fn)</sub>

> return debug representation that conforms to <32 elements in array

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result {
```

#### `impl LimitHeap`  <sub>(impl)</sub>

> implementation of a LimitHeap

```rust
impl LimitHeap {
```

#### `move_right`  <sub>(fn)</sub>

> move to right child, return node's index from which the move was requested

```rust
pub(super) fn move_right(&mut self) -> usize {
```

#### `move_left`  <sub>(fn)</sub>

> move to left child, return node's index from which the move was requested

```rust
pub(super) fn move_left(&mut self) -> usize {
```

#### `move_up`  <sub>(fn)</sub>

> move to parent, return node's index from which the move was requested

```rust
pub(super) fn move_up(&mut self) -> usize {
```

#### `move_to`  <sub>(fn)</sub>

> move directly to the given index

```rust
pub(super) fn move_to(&mut self, index: usize) {
```

#### `value`  <sub>(fn)</sub>

> get the current node's value

```rust
pub(super) fn value(&self) -> i32 {
```

#### `set_value`  <sub>(fn)</sub>

> set the current node's value

```rust
pub(super) fn set_value(&mut self, value: i32) {
```

#### `has_parent`  <sub>(fn)</sub>

> check that this node has a parent (true for all except root)

```rust
pub(super) fn has_parent(&self) -> bool {
```

#### `parent_value`  <sub>(fn)</sub>

> get node's parent's value or self.original if at the root

```rust
pub(super) fn parent_value(&mut self) -> i32 {
```

#### `has_children`  <sub>(fn)</sub>

> check if the current node has children

```rust
pub(super) fn has_children(&self) -> bool {
```

#### `right_child_value`  <sub>(fn)</sub>

> get current node's right child's value

```rust
fn right_child_value(&mut self) -> i32 {
```

#### `set_left_child`  <sub>(fn)</sub>

> set current node's left child's value

```rust
fn set_left_child(&mut self) {
```

#### `set_right_child`  <sub>(fn)</sub>

> set current node's right child's value

```rust
fn set_right_child(&mut self) {
```

#### `clamp_to_max`  <sub>(fn)</sub>

> clamp all heap values to a maximum limit  
>   
> this is used when --rate-limit is set alongside --auto-tune to ensure  
> that no auto-tuning adjustment can exceed the user's specified rate limit.  
> only clamps non-zero values to preserve the "unset" marker (0) used during  
> heap construction.

```rust
pub(super) fn clamp_to_max(&mut self, max: i32) {
```

#### `build`  <sub>(fn)</sub>

> iterate over the backing array, filling in each child's value based on the original value

```rust
pub(super) fn build(&mut self) {
```

### `src/scanner/policy_data.rs`


#### `PolicyData`  <sub>(struct)</sub>

> data regarding policy and metadata about last enforced trigger etc...

```rust
pub struct PolicyData {
```

**Fields:**
- `pub(super) policy: RequesterPolicy,` — how to handle exceptional cases such as too many errors / 403s / 429s etc
- `pub(super) cooling_down: AtomicBool,` — whether or not we're in the middle of a cooldown period
- `pub(super) wait_time: u64,` — length of time to pause tuning after making an adjustment
- `limit: AtomicUsize,` — rate limit (at last interval)
- `pub(super) heap_initialized: AtomicBool,` — whether the heap has been initialized
- `pub(super) errors: [AtomicUsize; 3],` — number of errors (at last interval)
- `pub(super) remove_limit: AtomicBool,` — has been limited and moves back up to the point of its original scan speed
- `pub(super) heap: std::sync::RwLock<LimitHeap>,` — heap of values used for adjusting # of requests/second
- `pub(super) rate_limit: Option<usize>,` — dynamically based on the observed request rate


#### `impl PolicyData`  <sub>(impl)</sub>

> implementation of PolicyData

```rust
impl PolicyData {
```

#### `new`  <sub>(fn)</sub>

> given a RequesterPolicy, create a new PolicyData

```rust
pub fn new(policy: RequesterPolicy, timeout: u64) -> Self {
```

#### `MAX_WAIT_TIME_MS`  <sub>(const)</sub>

```rust
const MAX_WAIT_TIME_MS: u64 = 30_000;
```

#### `with_rate_limit`  <sub>(fn)</sub>

> builder for rate limit  
>   
> builder method chosen to not conflict with existing `new` api

```rust
pub fn with_rate_limit(mut self, rate_limit: usize) -> Self {
```

#### `set_reqs_sec`  <sub>(fn)</sub>

> setter for requests / second; populates the underlying heap with values from req/sec seed

```rust
pub(super) fn set_reqs_sec(&self, reqs_sec: usize) {
```

#### `set_errors`  <sub>(fn)</sub>

> setter for errors (trigger-specific)

```rust
pub(super) fn set_errors(&self, trigger: PolicyTrigger, errors: usize) {
```

#### `get_errors`  <sub>(fn)</sub>

> getter for errors (trigger-specific)

```rust
pub(super) fn get_errors(&self, trigger: PolicyTrigger) -> usize {
```

#### `heap_initialized`  <sub>(fn)</sub>

> status of heap initialization

```rust
pub(super) fn heap_initialized(&self) -> bool {
```

#### `reset_heap`  <sub>(fn)</sub>

> reset the heap and initialization flag, called when auto-tune is being disabled

```rust
pub(super) fn reset_heap(&self) {
```

#### `set_limit`  <sub>(fn)</sub>

> setter for limit

```rust
fn set_limit(&self, limit: usize) {
```

#### `get_limit`  <sub>(fn)</sub>

> getter for limit

```rust
pub(super) fn get_limit(&self) -> usize {
```

#### `adjust_up`  <sub>(fn)</sub>

> adjust the rate of requests per second up (increase rate)

```rust
pub(super) fn adjust_up(&self, streak_counter: &usize) {
```

#### `adjust_down`  <sub>(fn)</sub>

> adjust the rate of requests per second down (decrease rate)

```rust
pub(super) fn adjust_down(&self) {
```

#### `set_reqs_sec_builds_heap_and_sets_initial_value`  <sub>(fn)</sub>

> PolicyData builds and sets correct values for the inner heap when set_reqs_sec is called

```rust
fn set_reqs_sec_builds_heap_and_sets_initial_value() {
```

#### `policy_data_getters_and_setters`  <sub>(fn)</sub>

> PolicyData setters/getters tests for code coverage / sanity

```rust
fn policy_data_getters_and_setters() {
```

#### `policy_data_adjust_down_simple`  <sub>(fn)</sub>

> PolicyData adjust_down sets the limit to the correct value

```rust
fn policy_data_adjust_down_simple() {
```

#### `policy_data_adjust_down_no_children`  <sub>(fn)</sub>

> PolicyData adjust_down sets the limit to the correct value when no child nodes are present

```rust
fn policy_data_adjust_down_no_children() {
```

#### `policy_data_adjust_up_simple`  <sub>(fn)</sub>

> PolicyData adjust_up sets the limit to the correct value

```rust
fn policy_data_adjust_up_simple() {
```

#### `policy_data_adjust_up_with_streak_and_2_moves`  <sub>(fn)</sub>

> PolicyData adjust_up sets the limit to the correct value

```rust
fn policy_data_adjust_up_with_streak_and_2_moves() {
```

#### `policy_data_adjust_up_with_streak_and_2_moves_to_arrive_at_root`  <sub>(fn)</sub>

> PolicyData adjust_up sets the limit to the correct value

```rust
fn policy_data_adjust_up_with_streak_and_2_moves_to_arrive_at_root() {
```

#### `policy_data_adjust_up_with_streak_and_2_moves_to_find_less_than_current`  <sub>(fn)</sub>

> PolicyData adjust_up sets the limit to the correct value

```rust
fn policy_data_adjust_up_with_streak_and_2_moves_to_find_less_than_current() {
```

#### `policy_data_adjust_up_with_streak_and_3_moves`  <sub>(fn)</sub>

> PolicyData adjust_up sets the limit to the correct value

```rust
fn policy_data_adjust_up_with_streak_and_3_moves() {
```

#### `policy_data_adjust_up_with_no_children_2_moves`  <sub>(fn)</sub>

> PolicyData adjust_up sets the limit to the correct value

```rust
fn policy_data_adjust_up_with_no_children_2_moves() {
```

#### `policy_data_adjust_up_with_no_children_3_moves`  <sub>(fn)</sub>

> PolicyData adjust_up sets the limit to the correct value

```rust
fn policy_data_adjust_up_with_no_children_3_moves() {
```

#### `increase_limit_heap_coverage_by_hitting_edge_cases`  <sub>(fn)</sub>

> hit some of the out of the way corners of limitheap for coverage

```rust
fn increase_limit_heap_coverage_by_hitting_edge_cases() {
```

### `src/scanner/requester.rs`


#### `ref`  <sub>(static)</sub>

> make sure to note that this is a std rwlock and not tokio

```rust
pub(crate) static ref TF_IDF: Arc<sync::RwLock<TfIdf>> = Arc::new(sync::RwLock::new(TfIdf::new()));
```

#### `Requester`  <sub>(struct)</sub>

> Makes multiple requests based on the presence of extensions

```rust
pub(super) struct Requester {
```

**Fields:**
- `handles: Arc<Handles>,` — handles to handlers and config
- `target_url: String,` — url that will be scanned
- `rate_limiter: RwLock<Option<RateLimiter>>,` — limits requests per second if present
- `policy_data: PolicyData,` — data regarding policy and metadata about last enforced trigger etc...
- `ferox_scan: Arc<FeroxScan>,` — FeroxScan associated with the creation of this Requester
- `seen_links: RwLock<HashSet<String>>,` — interior mutability, similar to the tuning_lock below
- `tuning_lock: Mutex<usize>,` — the need for a counter)
- `policy_triggered: AtomicBool,`


#### `impl Requester`  <sub>(impl)</sub>

> Requester implementation

```rust
impl Requester {
```

#### `from`  <sub>(fn)</sub>

> given a FeroxScanner, create a Requester

```rust
pub fn from(scanner: &FeroxScanner, ferox_scan: Arc<FeroxScan>) -> Result<Self> {
```

#### `build_a_bucket`  <sub>(fn)</sub>

> build a RateLimiter, given a rate limit (as requests per second)

```rust
fn build_a_bucket(limit: usize) -> Result<RateLimiter> {
```

#### `cool_down`  <sub>(fn)</sub>

> sleep and set a flag that can be checked by other threads

```rust
async fn cool_down(&self) {
```

#### `limit`  <sub>(fn)</sub>

> limit the number of requests per second

```rust
pub async fn limit(&self) -> Result<()> {
```

#### `too_many_errors`  <sub>(fn)</sub>

> small function to break out different error checking mechanisms

```rust
fn too_many_errors(&self) -> bool {
```

#### `too_many_status_errors`  <sub>(fn)</sub>

> small function to break out different error checking mechanisms

```rust
fn too_many_status_errors(&self, trigger: PolicyTrigger) -> bool {
```

#### `should_enforce_policy`  <sub>(fn)</sub>

> determine whether or not a policy needs to be enforced  
>   
> criteria:  
> - number of threads (50 default) for general errors (timeouts etc)  
> - 90% of requests are 403  
> - 30% of requests are 429

```rust
fn should_enforce_policy(&self) -> Option<PolicyTrigger> {
```

#### `adjust_limit`  <sub>(fn)</sub>

> wrapper for adjust_[up,down] functions, checks error levels to determine adjustment direction

```rust
async fn adjust_limit(&self, trigger: PolicyTrigger, create_limiter: bool) -> Result<()> {
```

#### `set_rate_limiter`  <sub>(fn)</sub>

> lock the rate limiter and set its value to ta new leaky_bucket

```rust
async fn set_rate_limiter(&self, new_limit: Option<usize>) -> Result<()> {
```

#### `tune`  <sub>(fn)</sub>

> enforce auto-tune policy

```rust
async fn tune(&self, trigger: PolicyTrigger) -> Result<()> {
```

#### `bail`  <sub>(fn)</sub>

> enforce auto-bail policy

```rust
async fn bail(&self, trigger: PolicyTrigger) -> Result<()> {
```

#### `request`  <sub>(fn)</sub>

> Wrapper for make_request  
>   
> Attempts recursion when appropriate and sends Responses to the output handler for processing

```rust
pub async fn request(&self, word: &str) -> Result<()> {
```

#### `setup_requester_test`  <sub>(fn)</sub>

> helper to setup a realistic requester test

```rust
async fn setup_requester_test(config: Option<Arc<Configuration>>) -> (Arc<Handles>, Tasks) {
```

#### `increment_errors`  <sub>(fn)</sub>

> helper to stay DRY

```rust
async fn increment_errors(handles: Arc<Handles>, scan: Arc<FeroxScan>, num_errors: usize) {
```

#### `increment_scan_errors`  <sub>(fn)</sub>

> helper to stay DRY

```rust
async fn increment_scan_errors(handles: Arc<Handles>, url: &str, num_errors: usize) {
```

#### `increment_scan_status_codes`  <sub>(fn)</sub>

> helper to stay DRY

```rust
async fn increment_scan_status_codes(
    handles: Arc<Handles>,
    url: &str,
    code: StatusCode,
    num_errors: usize,
) {
```

#### `increment_status_codes`  <sub>(fn)</sub>

> helper to stay DRY

```rust
async fn increment_status_codes(
    handles: Arc<Handles>,
    scan: Arc<FeroxScan>,
    num_codes: usize,
    code: StatusCode,
) {
```

#### `create_scan`  <sub>(fn)</sub>

```rust
async fn create_scan(
    handles: Arc<Handles>,
    url: &str,
    num_errors: usize,
    trigger: PolicyTrigger,
) -> Arc<FeroxScan> {
```

#### `should_enforce_policy_returns_false_on_not_enough_requests_seen`  <sub>(fn)</sub>

> should_enforce_policy should return false when # of requests is < threads; also when < 50

```rust
async fn should_enforce_policy_returns_false_on_not_enough_requests_seen() {
```

#### `should_enforce_policy_returns_true_on_error_times_threads`  <sub>(fn)</sub>

> should_enforce_policy should return true when # of requests is >= 50 and errors >= threads * 2

```rust
async fn should_enforce_policy_returns_true_on_error_times_threads() {
```

#### `should_enforce_policy_returns_true_on_excessive_403s`  <sub>(fn)</sub>

> should_enforce_policy should return true when # of requests is >= 50 and 403s >= 45 (90%)

```rust
async fn should_enforce_policy_returns_true_on_excessive_403s() {
```

#### `should_enforce_policy_returns_true_on_excessive_429s`  <sub>(fn)</sub>

> should_enforce_policy should return true when # of requests is >= 50 and errors >= 45 (90%)

```rust
async fn should_enforce_policy_returns_true_on_excessive_429s() {
```

#### `bail_calls_abort_on_highest_errored_feroxscan`  <sub>(fn)</sub>

> bail should call abort on the scan with the most errors

```rust
async fn bail_calls_abort_on_highest_errored_feroxscan() {
```

#### `bail_returns_ok_on_no_active_scans`  <sub>(fn)</sub>

> bail is ok when no active scans are found

```rust
async fn bail_returns_ok_on_no_active_scans() {
```

#### `should_enforce_policy_returns_none_on_cooldown`  <sub>(fn)</sub>

> should_enforce should early exit when cooldown flag is set

```rust
async fn should_enforce_policy_returns_none_on_cooldown() {
```

#### `cooldown_pauses_for_wait_time`  <sub>(fn)</sub>

> cooldown should pause execution for the specified wait_time  
> note: cooling_down flag is now set by should_enforce_policy, not cool_down itself

```rust
async fn cooldown_pauses_for_wait_time() {
```

#### `adjust_limit_increments_streak_counter_on_upward_movement`  <sub>(fn)</sub>

> adjust_limit should add one to the streak counter when errors from scan equal policy and  
> increase the scan rate

```rust
async fn adjust_limit_increments_streak_counter_on_upward_movement() {
```

#### `adjust_limit_resets_streak_counter_on_downward_movement`  <sub>(fn)</sub>

> adjust_limit should reset the streak counter when errors from scan are > policy and  
> decrease the scan rate

```rust
async fn adjust_limit_resets_streak_counter_on_downward_movement() {
```

#### `adjust_limit_removes_rate_limiter`  <sub>(fn)</sub>

> adjust_limit should remove the rate limiter when remove_limit is set

```rust
async fn adjust_limit_removes_rate_limiter() {
```

#### `too_many_status_errors_returns_correct_values`  <sub>(fn)</sub>

> errors policytrigger should always be false, 403 is high ratio, and 429 is high ratio / 3

```rust
async fn too_many_status_errors_returns_correct_values() {
```

#### `set_rate_limiter_early_exit`  <sub>(fn)</sub>

> set_rate_limiter should exit early when new limit equals the current bucket's max

```rust
async fn set_rate_limiter_early_exit() {
```

#### `tune_sets_expected_values_and_then_waits`  <sub>(fn)</sub>

> tune should set req/sec and rate_limiter, adjust the limit and cooldown

```rust
async fn tune_sets_expected_values_and_then_waits() {
```

#### `build_a_bucket_handles_low_rates_correctly`  <sub>(fn)</sub>

> verify build_a_bucket produces correct rate limits for low values (1-20 req/s)  
> This test validates the fix for Bug #1 where limits < 15 collapsed to 1 req/s

```rust
fn build_a_bucket_handles_low_rates_correctly() {
```

#### `policy_triggered_reset_when_limiter_removed`  <sub>(fn)</sub>

> verify that policy_triggered flag is reset when rate limiter is removed  
> This test validates the fix for Bug #2 where auto-tune never disengaged

```rust
async fn policy_triggered_reset_when_limiter_removed() {
```

#### `should_enforce_policy_uses_per_scan_requests`  <sub>(fn)</sub>

> verify should_enforce_policy uses per-scan request counts, not global  
> This test validates the fix for Bug #4 where global counters caused false positives

```rust
async fn should_enforce_policy_uses_per_scan_requests() {
```

#### `heap_values_clamped_to_rate_limit_cap`  <sub>(fn)</sub>

> verify heap values are clamped when rate_limit cap is set

```rust
async fn heap_values_clamped_to_rate_limit_cap() {
```

#### `auto_tune_with_cap_adjusts_down_on_errors`  <sub>(fn)</sub>

> verify auto-tune with cap adjusts down correctly on errors

```rust
async fn auto_tune_with_cap_adjusts_down_on_errors() {
```

#### `auto_tune_with_cap_never_exceeds_cap_on_upward_adjustment`  <sub>(fn)</sub>

> verify auto-tune with cap never exceeds cap on upward adjustment

```rust
async fn auto_tune_with_cap_never_exceeds_cap_on_upward_adjustment() {
```

#### `remove_limit_with_cap_sets_to_cap_instead_of_removing`  <sub>(fn)</sub>

> verify remove_limit with cap sets to cap instead of removing

```rust
async fn remove_limit_with_cap_sets_to_cap_instead_of_removing() {
```

#### `initial_limiter_set_to_cap_when_both_flags_present`  <sub>(fn)</sub>

> verify initial limiter set to cap when both rate_limit and auto_tune are present

```rust
async fn initial_limiter_set_to_cap_when_both_flags_present() {
```

#### `capped_auto_tune_full_lifecycle`  <sub>(fn)</sub>

> Full lifecycle test: --rate-limit 100 --auto-tune  
> Simulates errors triggering reduction, then success allowing increase, never exceeding cap

```rust
async fn capped_auto_tune_full_lifecycle() {
```

### `src/scanner/utils.rs`


#### `PolicyTrigger`  <sub>(enum)</sub>

> represents different situations where different criteria can trigger auto-tune/bail behavior

```rust
pub enum PolicyTrigger {
```

**Variants:**
- `Status403,` — excessive 403 trigger
- `Status429,` — excessive 429 trigger
- `Errors,` — excessive general errors
- `TryAdjustUp,` — dummy error for upward rate adjustment


#### `impl PolicyTrigger`  <sub>(impl)</sub>

```rust
impl PolicyTrigger {
```

#### `as_index`  <sub>(fn)</sub>

> get the index into the `PolicyData.errors` array for this trigger

```rust
pub fn as_index(&self) -> usize {
```



## Module `scan_manager` <a id='module-scan-manager'></a>

*127 documented items*


### `src/scan_manager/menu.rs`


#### `MenuCmd`  <sub>(enum)</sub>

> Data container for a command entered by the user interactively

```rust
pub enum MenuCmd {
```

**Variants:**
- `AddUrl(String),` — user wants to add a url to be scanned
- `Cancel(Vec<usize>, bool),` — user wants to cancel one or more active scans
- `AddFilter(Box<dyn FeroxFilter>),` — user wants to create a new filter
- `RemoveFilter(Vec<usize>),` — user wants to remove one or more active filters
- `SetScanPermits(usize),` — user wants to set the number of scan permits


#### `MenuCmdResult`  <sub>(enum)</sub>

> Data container for a command result to be used internally by the ferox_scanner

```rust
pub enum MenuCmdResult {
```

**Variants:**
- `Url(String),` — Url to be added to the scan queue
- `NumCancelled(usize),` — Number of scans that were actually cancelled, can be 0
- `Filter(Box<dyn FeroxFilter>),` — Filter to be added to current list of `FeroxFilters`
- `NumPermitsToAdd(usize),` — number of permits to be added to the semaphore
- `NumPermitsToSubtract(usize),` — number of permits to be subtracted from the semaphore


#### `Menu`  <sub>(struct)</sub>

> Interactive scan cancellation menu

```rust
pub(super) struct Menu {
```

**Fields:**
- `header: String,` — header: name surrounded by separators
- `footer: String,` — footer: instructions surrounded by separators
- `longest: usize,` — length of longest displayed line (suitable for ascii/unicode)
- `border: String,` — unicode line border, matched to longest displayed line
- `pub(super) term: Term,` — target for output


#### `impl Menu`  <sub>(impl)</sub>

> Implementation of Menu

```rust
impl Menu {
```

#### `new`  <sub>(fn)</sub>

> Creates new Menu

```rust
pub(super) fn new() -> Self {
```

#### `print_header`  <sub>(fn)</sub>

> print menu header

```rust
pub(super) fn print_header(&self) {
```

#### `print_border`  <sub>(fn)</sub>

> print menu unicode border line

```rust
pub(super) fn print_border(&self) {
```

#### `print_footer`  <sub>(fn)</sub>

> print menu footer

```rust
pub(super) fn print_footer(&self) {
```

#### `print_eta`  <sub>(fn)</sub>

> print time remaining in a human-readable format

```rust
pub(super) fn print_eta(&self, eta: Duration) {
```

#### `print_scan_limit`  <sub>(fn)</sub>

> print time remaining in a human-readable format

```rust
pub(super) fn print_scan_limit(&self, limiter: Arc<DynamicSemaphore>) {
```

#### `hide_progress_bars`  <sub>(fn)</sub>

> set PROGRESS_BAR bar target to hidden

```rust
pub(super) fn hide_progress_bars(&self) {
```

#### `show_progress_bars`  <sub>(fn)</sub>

> set PROGRESS_BAR bar target to hidden

```rust
pub(super) fn show_progress_bars(&self) {
```

#### `clear_screen`  <sub>(fn)</sub>

> Wrapper around console's Term::clear_screen and flush

```rust
pub(super) fn clear_screen(&self) {
```

#### `println`  <sub>(fn)</sub>

> Wrapper around console's Term::write_line

```rust
pub(super) fn println(&self, msg: &str) {
```

#### `str_to_usize`  <sub>(fn)</sub>

> Helper for parsing a usize from a str

```rust
fn str_to_usize(&self, value: &str) -> usize {
```

#### `split_to_nums`  <sub>(fn)</sub>

> split a comma delimited string into vec of usizes

```rust
pub(super) fn split_to_nums(&self, line: &str) -> Vec<usize> {
```

#### `get_command_input_from_user`  <sub>(fn)</sub>

> get input from the user and translate it to a `MenuCmd`

```rust
pub(super) fn get_command_input_from_user(&self, line: &str) -> Option<MenuCmd> {
```

#### `confirm_cancellation`  <sub>(fn)</sub>

> Given a url, confirm with user that we should cancel

```rust
pub(super) fn confirm_cancellation(&self, url: &str) -> char {
```

#### `impl Default for Menu`  <sub>(impl)</sub>

> Default implementation for Menu

```rust
impl Default for Menu {
```

#### `default`  <sub>(fn)</sub>

> return Menu::new as default

```rust
fn default() -> Menu {
```

### `src/scan_manager/order.rs`


#### `ScanOrder`  <sub>(enum)</sub>

> Simple enum to designate whether a URL was passed in by the user (Initial) or found during  
> scanning (Latest)

```rust
pub enum ScanOrder {
```

**Variants:**
- `Initial,` — Url was passed in by the user
- `Latest,` — Url was found during scanning


### `src/scan_manager/response_container.rs`


#### `FeroxResponses`  <sub>(struct)</sub>

> Container around a locked vector of `FeroxResponse`s, adds wrappers for insertion and search

```rust
pub struct FeroxResponses {
```

**Fields:**
- `pub responses: Arc<RwLock<Vec<FeroxResponse>>>,` — Internal structure: locked hashset of `FeroxScan`s


#### `impl Serialize for FeroxResponses`  <sub>(impl)</sub>

> Serialize implementation for FeroxResponses

```rust
impl Serialize for FeroxResponses {
```

#### `serialize`  <sub>(fn)</sub>

> Function that handles serialization of FeroxResponses

```rust
fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    {
```

#### `impl FeroxResponses`  <sub>(impl)</sub>

> Implementation of `FeroxResponses`

```rust
impl FeroxResponses {
```

#### `insert`  <sub>(fn)</sub>

> Add a `FeroxResponse` to the internal container

```rust
pub fn insert(&self, response: FeroxResponse) {
```

#### `contains`  <sub>(fn)</sub>

> Simple check for whether or not a FeroxResponse is contained within the inner container

```rust
pub fn contains(&self, other: &FeroxResponse) -> bool {
```

#### `create_response_json`  <sub>(fn)</sub>

```rust
fn create_response_json(
    url: &str,
    status: u16,
    word_count: usize,
    content_length: u64,
) -> FeroxResponse {
```

#### `contains_method_works_correctly`  <sub>(fn)</sub>

> test that contains method works correctly

```rust
fn contains_method_works_correctly() {
```

### `src/scan_manager/scan.rs`


#### `Visibility`  <sub>(enum)</sub>

```rust
pub enum Visibility {
```

**Variants:**
- `Visible,` — whether a FeroxScan's progress bar is currently shown
- `Hidden,` — whether a FeroxScan's progress bar is currently hidden


#### `FeroxScan`  <sub>(struct)</sub>

> Struct to hold scan-related state  
>   
> The purpose of this container is to open up the pathway to aborting currently running tasks and  
> serialization of all scan state into a state file in order to resume scans that were cut short

```rust
pub struct FeroxScan {
```

**Fields:**
- `pub(super) id: String,` — UUID that uniquely ID's the scan
- `pub(super) url: String,` — The URL that to be scanned
- `pub(super) normalized_url: String,` — A url used solely for comparison to other URLs
- `pub scan_type: ScanType,` — The type of scan
- `pub(crate) scan_order: ScanOrder,` — The order in which the scan was received
- `pub(super) num_requests: u64,` — Number of requests to populate the progress bar with
- `pub(super) requests_made_so_far: u64,` — deserialization: sets self.requests_made_so_far to this field
- `pub status: Mutex<ScanStatus>,` — Status of this scan
- `pub(super) task: sync::Mutex<Option<JoinHandle<()>>>,` — The spawned tokio task performing this scan (uses tokio::sync::Mutex)
- `pub progress_bar: Mutex<Option<ProgressBar>>,` — The progress bar associated with this scan
- `pub(super) output_level: OutputLevel,` — whether or not the user passed --silent|--quiet on the command line
- `pub(super) status_403s: AtomicUsize,` — tracker for overall number of 403s seen by the FeroxScan instance
- `pub(super) status_429s: AtomicUsize,` — tracker for overall number of 429s seen by the FeroxScan instance
- `pub(super) errors: AtomicUsize,` — tracker for total number of errors encountered by the FeroxScan instance
- `pub(super) start_time: Mutex<Instant>,` — tracker for the time at which this scan was started
- `pub(super) visible: AtomicBool,` — whether the progress bar is currently visible or hidden
- `pub(super) handles: Option<Arc<Handles>>,` — handles object pointer


#### `impl Default for FeroxScan`  <sub>(impl)</sub>

> Default implementation for FeroxScan

```rust
impl Default for FeroxScan {
```

#### `default`  <sub>(fn)</sub>

> Create a default FeroxScan, populates ID with a new UUID

```rust
fn default() -> Self {
```

#### `impl FeroxScan`  <sub>(impl)</sub>

> Implementation of FeroxScan

```rust
impl FeroxScan {
```

#### `visible`  <sub>(fn)</sub>

> return the visibility of the scan as a boolean

```rust
pub fn visible(&self) -> bool {
```

#### `swap_visibility`  <sub>(fn)</sub>

```rust
pub fn swap_visibility(&self) {
```

#### `abort`  <sub>(fn)</sub>

> Stop a currently running scan

```rust
pub async fn abort(&self, active_bars: usize) -> Result<()> {
```

#### `url`  <sub>(fn)</sub>

> getter for url

```rust
pub fn url(&self) -> &str {
```

#### `requests_made_so_far`  <sub>(fn)</sub>

> getter for number of requests made during previously saved scans (i.e. --resume-from used)

```rust
pub fn requests_made_so_far(&self) -> u64 {
```

#### `set_task`  <sub>(fn)</sub>

> small wrapper to set the JoinHandle

```rust
pub async fn set_task(&self, task: JoinHandle<()>) -> Result<()> {
```

#### `set_status`  <sub>(fn)</sub>

> small wrapper to set ScanStatus

```rust
pub fn set_status(&self, status: ScanStatus) -> Result<()> {
```

#### `set_start_time`  <sub>(fn)</sub>

> small wrapper to set `start_time`

```rust
pub fn set_start_time(&self, start_time: Instant) -> Result<()> {
```

#### `stop_progress_bar`  <sub>(fn)</sub>

> Simple helper to call .finish on the scan's progress bar

```rust
pub(super) fn stop_progress_bar(&self, active_bars: usize) {
```

#### `progress_bar`  <sub>(fn)</sub>

> Simple helper get a progress bar

```rust
pub fn progress_bar(&self) -> ProgressBar {
```

#### `new`  <sub>(fn)</sub>

> Given a URL and ProgressBar, create a new FeroxScan, wrap it in an Arc and return it

```rust
pub fn new(
    url: &str,
    scan_type: ScanType,
    scan_order: ScanOrder,
    num_requests: u64,
    output_level: OutputLevel,
    pb: Option<ProgressBar>,
    visibility: bool,
    handles: Arc<Handles>,
) -> Arc<Self> {
```

#### `finish`  <sub>(fn)</sub>

> Mark the scan as complete and stop the scan's progress bar

```rust
pub fn finish(&self, active_bars: usize) -> Result<()> {
```

#### `is_active`  <sub>(fn)</sub>

> small wrapper to inspect ScanType and ScanStatus to see if a Directory scan is running or  
> in the queue to be run

```rust
pub fn is_active(&self) -> bool {
```

#### `is_complete`  <sub>(fn)</sub>

> small wrapper to inspect ScanStatus and see if it's Complete

```rust
pub fn is_complete(&self) -> bool {
```

#### `is_cancelled`  <sub>(fn)</sub>

> small wrapper to inspect ScanStatus and see if it's Cancelled

```rust
pub fn is_cancelled(&self) -> bool {
```

#### `is_running`  <sub>(fn)</sub>

> small wrapper to inspect ScanStatus and see if it's Running

```rust
pub fn is_running(&self) -> bool {
```

#### `is_not_started`  <sub>(fn)</sub>

> small wrapper to inspect ScanStatus and see if it's NotStarted

```rust
pub fn is_not_started(&self) -> bool {
```

#### `join`  <sub>(fn)</sub>

> await a task's completion, similar to a thread's join; perform necessary bookkeeping

```rust
pub async fn join(&self) {
```

#### `add_403`  <sub>(fn)</sub>

> increment the value in question by 1

```rust
pub(crate) fn add_403(&self) {
```

#### `add_429`  <sub>(fn)</sub>

> increment the value in question by 1

```rust
pub(crate) fn add_429(&self) {
```

#### `add_error`  <sub>(fn)</sub>

> increment the value in question by 1

```rust
pub(crate) fn add_error(&self) {
```

#### `num_errors`  <sub>(fn)</sub>

> simple wrapper to call the appropriate getter based on the given PolicyTrigger

```rust
pub fn num_errors(&self, trigger: PolicyTrigger) -> usize {
```

#### `errors`  <sub>(fn)</sub>

> return the number of errors seen by this scan

```rust
fn errors(&self) -> usize {
```

#### `status_403s`  <sub>(fn)</sub>

> return the number of 403s seen by this scan

```rust
fn status_403s(&self) -> usize {
```

#### `status_429s`  <sub>(fn)</sub>

> return the number of 429s seen by this scan

```rust
fn status_429s(&self) -> usize {
```

#### `requests_per_second`  <sub>(fn)</sub>

> return the number of requests per second performed by this scan's scanner

```rust
pub fn requests_per_second(&self) -> u64 {
```

#### `requests`  <sub>(fn)</sub>

> return the number of requests performed by this scan's scanner

```rust
pub fn requests(&self) -> u64 {
```

#### `impl fmt::Display for FeroxScan`  <sub>(impl)</sub>

> Display implementation

```rust
impl fmt::Display for FeroxScan {
```

#### `fmt`  <sub>(fn)</sub>

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
```

#### `impl PartialEq for FeroxScan`  <sub>(impl)</sub>

> PartialEq implementation; uses FeroxScan.id for comparison

```rust
impl PartialEq for FeroxScan {
```

#### `eq`  <sub>(fn)</sub>

```rust
fn eq(&self, other: &Self) -> bool {
```

#### `impl Serialize for FeroxScan`  <sub>(impl)</sub>

> Serialize implementation for FeroxScan

```rust
impl Serialize for FeroxScan {
```

#### `serialize`  <sub>(fn)</sub>

> Function that handles serialization of a FeroxScan

```rust
fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    {
```

#### `impl<'de> Deserialize<'de> for FeroxScan`  <sub>(impl)</sub>

> Deserialize implementation for FeroxScan

```rust
impl<'de> Deserialize<'de> for FeroxScan {
```

#### `deserialize`  <sub>(fn)</sub>

> Deserialize a FeroxScan from a serde_json::Value

```rust
fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
where
    D: Deserializer<'de>,
    {
```

#### `ScanType`  <sub>(enum)</sub>

> Simple enum used to flag a `FeroxScan` as likely a directory or file

```rust
pub enum ScanType {
```

**Variants:**
- `File,` — Just a file being requested
- `Directory,` — A an entire directory that might be scanned


#### `impl Default for ScanType`  <sub>(impl)</sub>

> Default implementation for ScanType

```rust
impl Default for ScanType {
```

#### `default`  <sub>(fn)</sub>

> Return ScanType::File as default

```rust
fn default() -> Self {
```

#### `ScanStatus`  <sub>(enum)</sub>

> Simple enum to represent a scan's current status ([in]complete, cancelled)

```rust
pub enum ScanStatus {
```

**Variants:**
- `NotStarted,` — Scan hasn't started yet
- `Complete,` — Scan finished normally
- `Cancelled,` — Scan was cancelled by the user
- `Running,` — Scan has started, but hasn't finished, nor been cancelled
- `Waiting,` — Scan is waiting to be started due to max concurrent scan limit


#### `impl Default for ScanStatus`  <sub>(impl)</sub>

> Default implementation for ScanStatus

```rust
impl Default for ScanStatus {
```

#### `default`  <sub>(fn)</sub>

> Default variant for ScanStatus is NotStarted

```rust
fn default() -> Self {
```

#### `num_errors_returns_correct_values`  <sub>(fn)</sub>

> ensure that num_errors returns the correct values for the given PolicyTrigger  
>   
> covers tests for add_[403,429,error] and the related getters in addition to num_errors

```rust
fn num_errors_returns_correct_values() {
```

#### `requests_per_second_returns_correct_values`  <sub>(fn)</sub>

> ensure that requests_per_second returns the correct values

```rust
fn requests_per_second_returns_correct_values() {
```

#### `test_swap_visibility`  <sub>(fn)</sub>

```rust
fn test_swap_visibility() {
```

#### `test_is_running`  <sub>(fn)</sub>

> test for is_running method

```rust
fn test_is_running() {
```

### `src/scan_manager/scan_container.rs`


#### `INTERACTIVE_BARRIER`  <sub>(static)</sub>

> Single atomic number that gets incremented once, used to track first thread to interact with  
> when pausing a scan

```rust
static INTERACTIVE_BARRIER: AtomicUsize = AtomicUsize::new(0);
```

#### `PAUSE_SCAN`  <sub>(static)</sub>

> Atomic boolean flag, used to determine whether or not a scan should pause or resume

```rust
pub static PAUSE_SCAN: AtomicBool = AtomicBool::new(false);
```

#### `FeroxScans`  <sub>(struct)</sub>

> Container around a locked hashset of `FeroxScan`s, adds wrappers for insertion and searching

```rust
pub struct FeroxScans {
```

**Fields:**
- `pub scans: RwLock<Vec<Arc<FeroxScan>>>,` — Internal structure: locked hashset of `FeroxScan`s
- `menu: Menu,` — menu used for providing a way for users to cancel a scan
- `bar_length: Mutex<u64>,` — progress bars and feroxscans
- `output_level: OutputLevel,` — whether or not the user passed --silent|--quiet on the command line
- `pub(crate) collected_extensions: RwLock<HashSet<String>>,` — vector of extensions discovered and collected during scans
- `bar_limit: usize,` — stored value for Configuration.limit_bars


#### `impl Serialize for FeroxScans`  <sub>(impl)</sub>

> Serialize implementation for FeroxScans  
>   
> purposefully skips menu attribute

```rust
impl Serialize for FeroxScans {
```

#### `serialize`  <sub>(fn)</sub>

> Function that handles serialization of FeroxScans

```rust
fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    {
```

#### `impl FeroxScans`  <sub>(impl)</sub>

> Implementation of `FeroxScans`

```rust
impl FeroxScans {
```

#### `new`  <sub>(fn)</sub>

> given an OutputLevel, create a new FeroxScans object

```rust
pub fn new(output_level: OutputLevel, bar_limit: usize) -> Self {
```

#### `insert`  <sub>(fn)</sub>

> Add a `FeroxScan` to the internal container  
>   
> If the internal container did NOT contain the scan, true is returned; else false

```rust
pub fn insert(&self, scan: Arc<FeroxScan>) -> bool {
```

#### `add_serialized_scans`  <sub>(fn)</sub>

> load serialized FeroxScan(s) and any previously collected extensions into this FeroxScans

```rust
pub fn add_serialized_scans(&self, filename: &str, handles: Arc<Handles>) -> Result<()> {
```

#### `contains`  <sub>(fn)</sub>

> Simple check for whether or not a FeroxScan is contained within the inner container based  
> on the given URL

```rust
pub fn contains(&self, url: &str) -> bool {
```

#### `get_scan_by_url`  <sub>(fn)</sub>

> Find and return a `FeroxScan` based on the given URL

```rust
pub fn get_scan_by_url(&self, url: &str) -> Option<Arc<FeroxScan>> {
```

#### `get_base_scan_by_url`  <sub>(fn)</sub>

```rust
pub fn get_base_scan_by_url(&self, url: &str) -> Option<Arc<FeroxScan>> {
```

#### `increment_status_code`  <sub>(fn)</sub>

> add one to either 403 or 429 tracker in the scan related to the given url

```rust
pub fn increment_status_code(&self, url: &str, code: StatusCode) {
```

#### `increment_error`  <sub>(fn)</sub>

> add one to either 403 or 429 tracker in the scan related to the given url

```rust
pub fn increment_error(&self, url: &str) {
```

#### `display_scans`  <sub>(fn)</sub>

> Print all FeroxScans of type Directory  
>   
> Example:  
> 0: complete   https://10.129.45.20  
> 9: complete   https://10.129.45.20/images  
> 10: complete   https://10.129.45.20/assets

```rust
pub async fn display_scans(&self) {
```

#### `cancel_scans`  <sub>(fn)</sub>

> Given a list of indexes, cancel their associated FeroxScans

```rust
async fn cancel_scans(&self, indexes: Vec<usize>, force: bool) -> usize {
```

#### `display_filters`  <sub>(fn)</sub>

```rust
fn display_filters(&self, handles: Arc<Handles>) {
```

#### `interactive_menu`  <sub>(fn)</sub>

> CLI menu that allows for interactive cancellation of recursed-into directories

```rust
async fn interactive_menu(
    &self,
    handles: Arc<Handles>,
    limiter: Arc<DynamicSemaphore>,
) -> Option<MenuCmdResult> {
```

#### `print_known_responses`  <sub>(fn)</sub>

> prints all known responses that the scanner has already seen

```rust
pub fn print_known_responses(&self) {
```

#### `print_completed_bars`  <sub>(fn)</sub>

> if a resumed scan is already complete, display a completed progress bar to the user

```rust
pub fn print_completed_bars(&self, bar_length: usize) -> Result<()> {
```

#### `pause`  <sub>(fn)</sub>

> Forced the calling thread into a busy loop  
>   
> Every `SLEEP_DURATION` milliseconds, the function examines the result stored in `PAUSE_SCAN`  
>   
> When the value stored in `PAUSE_SCAN` becomes `false`, the function returns, exiting the busy  
> loop

```rust
pub async fn pause(
    &self,
    get_user_input: bool,
    handles: Arc<Handles>,
    limiter: Arc<DynamicSemaphore>,
) -> Option<MenuCmdResult> {
```

#### `set_bar_length`  <sub>(fn)</sub>

> set the bar length of FeroxScans

```rust
pub fn set_bar_length(&self, bar_length: u64) {
```

#### `add_scan`  <sub>(fn)</sub>

> Given a url, create a new `FeroxScan` and add it to `FeroxScans`  
>   
> If `FeroxScans` did not already contain the scan, return true; otherwise return false  
>   
> Also return a reference to the new `FeroxScan`

```rust
pub(super) fn add_scan(
    &self,
    url: &str,
    scan_type: ScanType,
    scan_order: ScanOrder,
    handles: Arc<Handles>,
) -> (bool, Arc<FeroxScan>) {
```

#### `add_directory_scan`  <sub>(fn)</sub>

> Given a url, create a new `FeroxScan` and add it to `FeroxScans` as a Directory Scan  
>   
> If `FeroxScans` did not already contain the scan, return true; otherwise return false  
>   
> Also return a reference to the new `FeroxScan`

```rust
pub fn add_directory_scan(
    &self,
    url: &str,
    scan_order: ScanOrder,
    handles: Arc<Handles>,
) -> (bool, Arc<FeroxScan>) {
```

#### `add_file_scan`  <sub>(fn)</sub>

> Given a url, create a new `FeroxScan` and add it to `FeroxScans` as a File Scan  
>   
> If `FeroxScans` did not already contain the scan, return true; otherwise return false  
>   
> Also return a reference to the new `FeroxScan`

```rust
pub fn add_file_scan(
    &self,
    url: &str,
    scan_order: ScanOrder,
    handles: Arc<Handles>,
) -> (bool, Arc<FeroxScan>) {
```

#### `number_of_bars`  <sub>(fn)</sub>

> returns the number of active AND visible scans; supports --limit-bars functionality

```rust
pub fn number_of_bars(&self) -> usize {
```

#### `make_visible`  <sub>(fn)</sub>

> make one hidden bar visible; supports --limit-bars functionality

```rust
pub fn make_visible(&self) {
```

#### `has_active_scans`  <sub>(fn)</sub>

> small helper to determine whether any scans are active or not

```rust
pub fn has_active_scans(&self) -> bool {
```

#### `get_active_scans`  <sub>(fn)</sub>

> Retrieve all active scans

```rust
pub fn get_active_scans(&self) -> Vec<Arc<FeroxScan>> {
```

#### `add_discovered_extension`  <sub>(fn)</sub>

> given an extension, add it to `collected_extensions` if all constraints are met  
> returns `true` if an extension was added, `false` otherwise

```rust
pub fn add_discovered_extension(&self, extension: String) -> bool {
```

#### `unknown_extension_is_added_to_collected_extensions`  <sub>(fn)</sub>

> unknown extension should be added to collected_extensions

```rust
fn unknown_extension_is_added_to_collected_extensions() {
```

#### `known_extension_is_added_to_collected_extensions`  <sub>(fn)</sub>

> known extension should not be added to collected_extensions

```rust
fn known_extension_is_added_to_collected_extensions() {
```

### `src/scan_manager/state.rs`


#### `FeroxState`  <sub>(struct)</sub>

> Data container for (de)?serialization of multiple items

```rust
pub struct FeroxState {
```

**Fields:**
- `scans: Arc<FeroxScans>,` — Known scans
- `config: Arc<Configuration>,` — Current running config
- `responses: &'static FeroxResponses,` — Known responses
- `statistics: Arc<Stats>,` — Gathered statistics
- `collected_extensions: HashSet<String>,` — collected extensions
- `filters: Arc<FeroxFilters>,` — runtime filters, as they may differ from original config


#### `impl FeroxState`  <sub>(impl)</sub>

> implementation of FeroxState

```rust
impl FeroxState {
```

#### `new`  <sub>(fn)</sub>

> create new FeroxState object

```rust
pub fn new(
    scans: Arc<FeroxScans>,
    config: Arc<Configuration>,
    responses: &'static FeroxResponses,
    statistics: Arc<Stats>,
    filters: Arc<FeroxFilters>,
) -> Self {
```

#### `impl FeroxSerialize for FeroxState`  <sub>(impl)</sub>

> FeroxSerialize implementation for FeroxState

```rust
impl FeroxSerialize for FeroxState {
```

#### `as_str`  <sub>(fn)</sub>

> Simply return debug format of FeroxState to satisfy as_str

```rust
fn as_str(&self) -> String {
```

#### `as_json`  <sub>(fn)</sub>

> Simple call to produce a JSON string using the given FeroxState

```rust
fn as_json(&self) -> Result<String> {
```

### `src/scan_manager/utils.rs`


#### `start_max_time_thread`  <sub>(fn)</sub>

> Given a string representing some number of seconds, minutes, hours, or days, convert  
> that representation to seconds and then wait for those seconds to elapse.  Once that period  
> of time has elapsed, kill all currently running scans and dump a state file to disk that can  
> be used to resume any unfinished scan.

```rust
pub async fn start_max_time_thread(handles: Arc<Handles>) {
```

#### `resume_scan`  <sub>(fn)</sub>

> Primary logic used to load a Configuration from disk and populate the appropriate data  
> structures

```rust
pub fn resume_scan(filename: &str) -> Configuration {
```

#### `determine_bar_type`  <sub>(fn)</sub>

> determine the type of progress bar to display  
> takes both --limit-bars and output-level (--quiet|--silent|etc)  
> into account to arrive at a `BarType`

```rust
pub fn determine_bar_type(
    bar_limit: usize,
    number_of_bars: usize,
    output_level: OutputLevel,
) -> BarType {
```

#### `test_no_limit_visible`  <sub>(fn)</sub>

```rust
fn test_no_limit_visible() {
```

#### `test_limit_exceeded_hidden`  <sub>(fn)</sub>

```rust
fn test_limit_exceeded_hidden() {
```

#### `test_limit_not_exceeded_visible`  <sub>(fn)</sub>

```rust
fn test_limit_not_exceeded_visible() {
```

#### `test_quiet_visible`  <sub>(fn)</sub>

```rust
fn test_quiet_visible() {
```

#### `test_quiet_hidden`  <sub>(fn)</sub>

```rust
fn test_quiet_hidden() {
```

#### `test_silent_hidden`  <sub>(fn)</sub>

```rust
fn test_silent_hidden() {
```

#### `test_silent_json_hidden`  <sub>(fn)</sub>

```rust
fn test_silent_json_hidden() {
```



## Module `extractor` <a id='module-extractor'></a>

*32 documented items*


### `src/extractor/builder.rs`


#### `LINKFINDER_REGEX`  <sub>(const)</sub>

> Regular expression used in [LinkFinder](https://github.com/GerbenJavado/LinkFinder)  
>   
> updated on 8 August 2025 to commit 1debac5dace4724fd6187c06f133578dae51c86f  
>   
> NOTE: the ` ? or # mark with parameters` lines need to have the # character escaped as `\#`  
> to avoid being interpreted as a comment by the Rust compiler

```rust
pub(super) const LINKFINDER_REGEX: &str = r#"(?x)
```

#### `ROBOTS_TXT_REGEX`  <sub>(const)</sub>

> Regular expression to pull url paths from robots.txt  
>   
> ref: https://developers.google.com/search/reference/robots_txt

```rust
pub(super) const ROBOTS_TXT_REGEX: &str =
    r#"(?m)^[ \t]*(?i)(allow|disallow)[ \t]*:[ \t]*(?P<url_path>[^ \t\r\n#$]*)?[ \t]*\$?(?:#.*)?$"#; // multi-line (?m), case-insensitive (?i)
```

#### `URL_CHARS_REGEX`  <sub>(const)</sub>

> Regular expression to filter bad characters from extracted url paths  
>   
> ref: https://www.rfc-editor.org/rfc/rfc3986#section-2

```rust
pub(super) const URL_CHARS_REGEX: &str = r#"["<>\\^`{|} ]"#;
```

#### `ExtractionTarget`  <sub>(enum)</sub>

> Which type of extraction should be performed

```rust
pub enum ExtractionTarget {
```

**Variants:**
- `ResponseBody,` — Examine a response body and extract javascript and html links (multiple tags)
- `RobotsTxt,` — Examine robots.txt (specifically) and extract links
- `DirectoryListing,` — Extract all <a> tags from a page


#### `ExtractorBuilder`  <sub>(struct)</sub>

> responsible for building an `Extractor`

```rust
pub struct ExtractorBuilder<'a> {
```

**Fields:**
- `response: Option<&'a FeroxResponse>,` — Response from which to extract links
- `url: String,` — URL of where to extract links
- `handles: Option<Arc<Handles>>,` — Handles object to house the underlying mpsc transmitters
- `target: ExtractionTarget,` — type of extraction to be performed


#### `impl<'a> Default for ExtractorBuilder<'a>`  <sub>(impl)</sub>

> ExtractorBuilder implementation

```rust
impl<'a> Default for ExtractorBuilder<'a> {
```

#### `default`  <sub>(fn)</sub>

```rust
fn default() -> Self {
```

#### `impl<'a> ExtractorBuilder<'a>`  <sub>(impl)</sub>

> ExtractorBuilder implementation

```rust
impl<'a> ExtractorBuilder<'a> {
```

#### `handles`  <sub>(fn)</sub>

> builder call to set `handles`

```rust
pub fn handles(&mut self, handles: Arc<Handles>) -> &mut Self {
```

#### `url`  <sub>(fn)</sub>

> builder call to set `url`

```rust
pub fn url(&mut self, url: &str) -> &mut Self {
```

#### `target`  <sub>(fn)</sub>

> builder call to set `target`

```rust
pub fn target(&mut self, target: ExtractionTarget) -> &mut Self {
```

#### `response`  <sub>(fn)</sub>

> builder call to set `response`

```rust
pub fn response(&mut self, response: &'a FeroxResponse) -> &mut Self {
```

#### `build`  <sub>(fn)</sub>

> finalize configuration of `ExtractorBuilder` and return an `Extractor`  
>   
> requires either `with_url` or `with_response` to have been used in the build process

```rust
pub fn build(&self) -> Result<Extractor<'a>> {
```

### `src/extractor/container.rs`


#### `request_link`  <sub>(fn)</sub>

> Wrapper around link extraction logic  
> - create a new Url object based on cli options/args  
> - check if the new Url has already been seen/scanned -> None  
> - make a request to the new Url ? -> Some(response) : None

```rust
pub(super) async fn request_link(url: &str, handles: Arc<Handles>) -> Result<Response> {
```

#### `RecursionStatus`  <sub>(enum)</sub>

> Whether an active scan is recursive or not

```rust
enum RecursionStatus {
```

**Variants:**
- `Recursive,` — Scan is recursive
- `NotRecursive,` — Scan is not recursive


#### `Extractor`  <sub>(struct)</sub>

> Handles all logic related to extracting links from requested source code

```rust
pub struct Extractor<'a> {
```

**Fields:**
- `pub(super) links_regex: Regex,` — `LINKFINDER_REGEX` as a regex::Regex type
- `pub(super) robots_regex: Regex,` — `ROBOTS_TXT_REGEX` as a regex::Regex type
- `pub(super) url_regex: Regex,` — regex to validate a url
- `pub(super) response: Option<&'a FeroxResponse>,` — Response from which to extract links
- `pub(super) url: String,` — URL of where to extract links
- `pub(super) handles: Arc<Handles>,` — Handles object to house the underlying mpsc transmitters
- `pub(super) target: ExtractionTarget,` — type of extraction to be performed


#### `impl<'a> Extractor<'a>`  <sub>(impl)</sub>

> Extractor implementation

```rust
impl<'a> Extractor<'a> {
```

#### `extract`  <sub>(fn)</sub>

> perform extraction from the given target and return any links found

```rust
pub async fn extract(&self) -> Result<ExtractionResult> {
```

#### `parse_url_and_add_subpaths`  <sub>(fn)</sub>

> wrapper around logic that performs the following:  
> - parses `url_to_parse`  
> - bails if the parsed url doesn't belong to the list of in-scope urls  
> - otherwise, calls `add_all_sub_paths` with the parsed result

```rust
fn parse_url_and_add_subpaths(
    &self,
    url_to_parse: &str,
    links: &mut HashSet<String>,
) -> Result<()> {
```

#### `request_links`  <sub>(fn)</sub>

> given a set of links from a normal http body response, task the request handler to make  
> the requests

```rust
pub async fn request_links(
    &mut self,
    links: HashSet<String>,
) -> Result<Option<tokio::task::JoinHandle<()>>> {
```

#### `extract_all_links_from_html_tags`  <sub>(fn)</sub>

> wrapper around link extraction via html attributes

```rust
fn extract_all_links_from_html_tags(
    &self,
    resp_url: &Url,
    links: &mut HashSet<String>,
    html: &Html,
) {
```

#### `extract_all_links_from_javascript`  <sub>(fn)</sub>

> Given the body of a `reqwest::Response`, perform the following actions  
> - parse the body for links using the linkfinder regex  
> - for every link found take its url path and parse each sub-path  
> - example: Response contains a link fragment `homepage/assets/img/icons/handshake.svg`  
> with a base url of http://localhost, the following urls would be returned:  
> - homepage/assets/img/icons/handshake.svg  
> - homepage/assets/img/icons/  
> - homepage/assets/img/  
> - homepage/assets/  
> - homepage/

```rust
fn extract_all_links_from_javascript(
    &self,
    response_body: &str,
    response_url: &Url,
    links: &mut HashSet<String>,
) {
```

#### `add_all_sub_paths`  <sub>(fn)</sub>

> take a url fragment like homepage/assets/img/icons/handshake.svg and  
> incrementally add  
> - homepage/assets/img/icons/  
> - homepage/assets/img/  
> - homepage/assets/  
> - homepage/

```rust
fn add_all_sub_paths(&self, url_path: &str, links: &mut HashSet<String>) -> Result<()> {
```

#### `normalize_url_path`  <sub>(fn)</sub>

> given a url path, trim whitespace, remove slashes, and queries/fragments; return the  
> normalized string

```rust
pub(super) fn normalize_url_path(&self, path: &str) -> String {
```

#### `get_sub_paths_from_path`  <sub>(fn)</sub>

> Iterate over a given path, return a list of every sub-path found  
>   
> example: `path` contains a link fragment `homepage/assets/img/icons/handshake.svg`  
> the following fragments would be returned:  
> - homepage/assets/img/icons/handshake.svg  
> - homepage/assets/img/icons/  
> - homepage/assets/img/  
> - homepage/assets/  
> - homepage/

```rust
pub(super) fn get_sub_paths_from_path(&self, path: &str) -> Vec<String> {
```

#### `add_link_to_set_of_links`  <sub>(fn)</sub>

> simple helper to stay DRY, tries to join a url + fragment and add it to the `links` HashSet

```rust
pub(super) fn add_link_to_set_of_links(
    &self,
    link: &str,
    links: &mut HashSet<String>,
) -> Result<()> {
```

#### `extract_from_robots`  <sub>(fn)</sub>

> Entry point to perform link extraction from robots.txt  
>   
> `base_url` can have paths and subpaths, however robots.txt will be requested from the  
> root of the url  
> given the url:  
> http://localhost/stuff/things  
> this function requests:  
> http://localhost/robots.txt

```rust
pub(super) async fn extract_from_robots(&self) -> Result<ExtractionResult> {
```

#### `extract_from_body`  <sub>(fn)</sub>

> outer-most wrapper for parsing html response bodies in search of additional content.  
> performs the following high-level steps:  
> - requests the page, if necessary  
> - checks the page to see if directory listing is enabled and sucks up all the links, if so  
> - uses the linkfinder regex to grab links from embedded javascript/javascript files  
> - extracts many different types of link sources from the html itself

```rust
pub(super) async fn extract_from_body(&self) -> Result<ExtractionResult> {
```

#### `extract_from_dir_listing`  <sub>(fn)</sub>

> parses html response bodies in search of <a> tags.  
>   
> the assumption is that directory listing is turned on and this extraction target simply  
> scoops up all the links for the given directory. The test to detect a directory listing  
> is located in `HeuristicTests`

```rust
pub async fn extract_from_dir_listing(&self) -> Result<ExtractionResult> {
```

#### `extract_links_by_attr`  <sub>(fn)</sub>

> simple helper to get html links by tag/attribute and add it to the `links` HashSet

```rust
fn extract_links_by_attr(
    &self,
    resp_url: &Url,
    links: &mut HashSet<String>,
    html: &Html,
    html_tag: &str,
    html_attr: &str,
) {
```

#### `make_extract_request`  <sub>(fn)</sub>

> helper function that simply requests at <location> on the given url's base url  
>   
> example:  
> http://localhost/api/users -> http://localhost/<location>

```rust
pub(super) async fn make_extract_request(&self, location: &str) -> Result<FeroxResponse> {
```

#### `update_stats`  <sub>(fn)</sub>

> update total number of links extracted and expected responses

```rust
fn update_stats(&self, num_links: usize) -> Result<()> {
```



## Module `filters` <a id='module-filters'></a>

*66 documented items*


### `src/filters/container.rs`


#### `FeroxFilters`  <sub>(struct)</sub>

> Container around a collection of `FeroxFilters`s

```rust
pub struct FeroxFilters {
```

**Fields:**
- `pub filters: RwLock<Vec<Box<dyn FeroxFilter>>>,` — collection of `FeroxFilters`


#### `impl FeroxFilters`  <sub>(impl)</sub>

> implementation of FeroxFilter collection

```rust
impl FeroxFilters {
```

#### `push`  <sub>(fn)</sub>

> add a single FeroxFilter to the collection

```rust
pub fn push(&self, filter: Box<dyn FeroxFilter>) -> Result<()> {
```

#### `remove`  <sub>(fn)</sub>

> remove items from the underlying collection by their index  
>   
> note: indexes passed in should be index-to-remove+1. This is built for the scan mgt menu  
> so indexes aren't 0-based whehn the user enters them.  
> 

```rust
pub fn remove(&self, indices: &mut [usize]) {
```

#### `should_filter_response`  <sub>(fn)</sub>

> Simple helper to stay DRY; determines whether or not a given `FeroxResponse` should be reported  
> to the user or not.

```rust
pub fn should_filter_response(
    &self,
    response: &FeroxResponse,
    tx_stats: CommandSender,
) -> bool {
```

#### `impl Serialize for FeroxFilters`  <sub>(impl)</sub>

```rust
impl Serialize for FeroxFilters {
```

#### `serialize`  <sub>(fn)</sub>

```rust
fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    {
```

### `src/filters/empty.rs`


#### `EmptyFilter`  <sub>(struct)</sub>

> Dummy filter for internal shenanigans

```rust
pub struct EmptyFilter {}
    
    impl FeroxFilter for EmptyFilter {
```

**Fields:**
- `fn should_filter_response(&self, _response: &FeroxResponse) -> bool {` — `EmptyFilter` always returns false
- `false`
- `}`
- `fn box_eq(&self, other: &dyn Any) -> bool {` — Compare one EmptyFilter to another
- `other.downcast_ref::<Self>() == Some(self)`
- `}`
- `fn as_any(&self) -> &dyn Any {` — Return self as Any for dynamic dispatch purposes
- `self`
- `}`


#### `should_filter_response`  <sub>(fn)</sub>

> `EmptyFilter` always returns false

```rust
fn should_filter_response(&self, _response: &FeroxResponse) -> bool {
```

#### `box_eq`  <sub>(fn)</sub>

> Compare one EmptyFilter to another

```rust
fn box_eq(&self, other: &dyn Any) -> bool {
```

#### `as_any`  <sub>(fn)</sub>

> Return self as Any for dynamic dispatch purposes

```rust
fn as_any(&self) -> &dyn Any {
```

### `src/filters/init.rs`


#### `initialize`  <sub>(fn)</sub>

> add all user-supplied filters to the (already started) filters handler

```rust
pub async fn initialize(handles: Arc<Handles>) -> Result<()> {
```

### `src/filters/lines.rs`


#### `LinesFilter`  <sub>(struct)</sub>

> Simple implementor of FeroxFilter; used to filter out responses based on the number of lines  
> in a Response body; specified using -N|--filter-lines

```rust
pub struct LinesFilter {
```

**Fields:**
- `pub line_count: usize,` — Number of lines in a Response's body that should be filtered


#### `impl FeroxFilter for LinesFilter`  <sub>(impl)</sub>

> implementation of FeroxFilter for LinesFilter

```rust
impl FeroxFilter for LinesFilter {
```

#### `should_filter_response`  <sub>(fn)</sub>

> Check `line_count` against what was passed in via -N|--filter-lines

```rust
fn should_filter_response(&self, response: &FeroxResponse) -> bool {
```

#### `box_eq`  <sub>(fn)</sub>

> Compare one LinesFilter to another

```rust
fn box_eq(&self, other: &dyn Any) -> bool {
```

#### `as_any`  <sub>(fn)</sub>

> Return self as Any for dynamic dispatch purposes

```rust
fn as_any(&self) -> &dyn Any {
```

### `src/filters/regex.rs`


#### `RegexFilter`  <sub>(struct)</sub>

> Simple implementor of FeroxFilter; used to filter out responses based on a given regular  
> expression; specified using -X|--filter-regex

```rust
pub struct RegexFilter {
```

**Fields:**
- `pub compiled: Regex,` — Regular expression to be applied to the response body for filtering, compiled
- `pub raw_string: String,` — Regular expression as passed in on the command line, not compiled


#### `impl Default for RegexFilter`  <sub>(impl)</sub>

```rust
impl Default for RegexFilter {
```

#### `default`  <sub>(fn)</sub>

```rust
fn default() -> Self {
```

#### `impl FeroxFilter for RegexFilter`  <sub>(impl)</sub>

> implementation of FeroxFilter for RegexFilter

```rust
impl FeroxFilter for RegexFilter {
```

#### `should_filter_response`  <sub>(fn)</sub>

> Check `expression` against the response body, if the expression matches, the response  
> should be filtered out

```rust
fn should_filter_response(&self, response: &FeroxResponse) -> bool {
```

#### `box_eq`  <sub>(fn)</sub>

> Compare one SizeFilter to another

```rust
fn box_eq(&self, other: &dyn Any) -> bool {
```

#### `as_any`  <sub>(fn)</sub>

> Return self as Any for dynamic dispatch purposes

```rust
fn as_any(&self) -> &dyn Any {
```

#### `impl PartialEq for RegexFilter`  <sub>(impl)</sub>

> PartialEq implementation for RegexFilter

```rust
impl PartialEq for RegexFilter {
```

#### `eq`  <sub>(fn)</sub>

> Simple comparison of the raw string passed in via the command line

```rust
fn eq(&self, other: &RegexFilter) -> bool {
```

### `src/filters/similarity.rs`


#### `ref`  <sub>(static)</sub>

> single instance of the sip hasher used in similarity filtering

```rust
pub static ref SIM_HASHER: SimHash<SimSipHasher64, u64, 64> =
    SimHash::<SimSipHasher64, u64, 64>::new(SimSipHasher64::new(1, 2));
```

#### `SimilarityFilter`  <sub>(struct)</sub>

> Simple implementor of FeroxFilter; used to filter out responses based on the similarity of a  
> Response body with a known response; specified using --filter-similar-to

```rust
pub struct SimilarityFilter {
```

**Fields:**
- `pub hash: u64,` — Hash of Response's body to be used during similarity comparison
- `pub original_url: String,` — Url originally requested for the similarity filter
- `pub cutoff: usize,` — Maximum hamming distance allowed between two signatures


#### `impl SimilarityFilter`  <sub>(impl)</sub>

```rust
impl SimilarityFilter {
```

#### `new`  <sub>(fn)</sub>

> Create a new SimilarityFilter

```rust
pub fn new(hash: u64, original_url: String, cutoff: usize) -> Self {
```

#### `impl From<&FeroxResponse> for SimilarityFilter`  <sub>(impl)</sub>

```rust
impl From<&FeroxResponse> for SimilarityFilter {
```

#### `from`  <sub>(fn)</sub>

```rust
fn from(response: &FeroxResponse) -> Self {
```

#### `impl FeroxFilter for SimilarityFilter`  <sub>(impl)</sub>

> implementation of FeroxFilter for SimilarityFilter

```rust
impl FeroxFilter for SimilarityFilter {
```

#### `should_filter_response`  <sub>(fn)</sub>

> Check `FeroxResponse::text` against what was requested from the site passed in via  
> --filter-similar-to

```rust
fn should_filter_response(&self, response: &FeroxResponse) -> bool {
```

#### `box_eq`  <sub>(fn)</sub>

> Compare one SimilarityFilter to another

```rust
fn box_eq(&self, other: &dyn Any) -> bool {
```

#### `as_any`  <sub>(fn)</sub>

> Return self as Any for dynamic dispatch purposes

```rust
fn as_any(&self) -> &dyn Any {
```

### `src/filters/size.rs`


#### `SizeFilter`  <sub>(struct)</sub>

> Simple implementor of FeroxFilter; used to filter out responses based on the length of a  
> Response body; specified using -S|--filter-size

```rust
pub struct SizeFilter {
```

**Fields:**
- `pub content_length: u64,` — Overall length of a Response's body that should be filtered


#### `impl FeroxFilter for SizeFilter`  <sub>(impl)</sub>

> implementation of FeroxFilter for SizeFilter

```rust
impl FeroxFilter for SizeFilter {
```

#### `should_filter_response`  <sub>(fn)</sub>

> Check `content_length` against what was passed in via -S|--filter-size

```rust
fn should_filter_response(&self, response: &FeroxResponse) -> bool {
```

#### `box_eq`  <sub>(fn)</sub>

> Compare one SizeFilter to another

```rust
fn box_eq(&self, other: &dyn Any) -> bool {
```

#### `as_any`  <sub>(fn)</sub>

> Return self as Any for dynamic dispatch purposes

```rust
fn as_any(&self) -> &dyn Any {
```

### `src/filters/status_code.rs`


#### `StatusCodeFilter`  <sub>(struct)</sub>

> Simple implementor of FeroxFilter; used to filter out status codes specified using  
> -C|--filter-status

```rust
pub struct StatusCodeFilter {
```

**Fields:**
- `pub filter_code: u16,` — Status code that should not be displayed to the user


#### `impl FeroxFilter for StatusCodeFilter`  <sub>(impl)</sub>

> implementation of FeroxFilter for StatusCodeFilter

```rust
impl FeroxFilter for StatusCodeFilter {
```

#### `should_filter_response`  <sub>(fn)</sub>

> Check `filter_code` against what was passed in via -C|--filter-status

```rust
fn should_filter_response(&self, response: &FeroxResponse) -> bool {
```

#### `box_eq`  <sub>(fn)</sub>

> Compare one StatusCodeFilter to another

```rust
fn box_eq(&self, other: &dyn Any) -> bool {
```

#### `as_any`  <sub>(fn)</sub>

> Return self as Any for dynamic dispatch purposes

```rust
fn as_any(&self) -> &dyn Any {
```

### `src/filters/utils.rs`


#### `create_similarity_filter`  <sub>(fn)</sub>

> wrapper around logic necessary to create a SimilarityFilter  
>   
> - parses given url  
> - makes request to the parsed url  
> - gathers extensions from the url, if configured to do so  
> - computes hash of response body  
> - creates filter with hash

```rust
pub(crate) async fn create_similarity_filter(
    similarity_filter: &str,
    handles: Arc<Handles>,
) -> Result<SimilarityFilter> {
```

#### `filter_lookup`  <sub>(fn)</sub>

> used in conjunction with the Scan Management Menu  
>   
> when a user uses the n[ew-filter] command in the menu, the two params are passed here for  
> processing.  
>   
> an example command may be `new-filter lines 40`. `lines` and `40` are passed here as &str's  
>   
> once here, the type and value are used to create an appropriate FeroxFilter. If anything  
> goes wrong during creation, None is returned.

```rust
pub(crate) fn filter_lookup(filter_type: &str, filter_value: &str) -> Option<Box<dyn FeroxFilter>> {
```

#### `filter_lookup_returns_correct_filters`  <sub>(fn)</sub>

> filter_lookup returns correct filters

```rust
fn filter_lookup_returns_correct_filters() {
```

#### `create_similarity_filter_is_correct`  <sub>(fn)</sub>

> ensure create_similarity_filter correctness of return value and side-effects

```rust
async fn create_similarity_filter_is_correct() {
```

### `src/filters/wildcard.rs`


#### `WildcardFilter`  <sub>(struct)</sub>

> Data holder for all relevant data needed when auto-filtering out wildcard responses

```rust
pub struct WildcardFilter {
```

**Fields:**
- `pub content_length: Option<u64>,` — The content-length of this response, if known
- `pub line_count: Option<usize>,` — The number of lines contained in the body of this response, if known
- `pub word_count: Option<usize>,` — The number of words contained in the body of this response, if known
- `pub method: String,` — method used in request that should be included with filters passed via runtime configuration
- `pub status_code: u16,` — the status code returned in the response
- `pub dont_filter: bool,` — whether or not the user passed -D on the command line


#### `impl WildcardFilter`  <sub>(impl)</sub>

> implementation of WildcardFilter

```rust
impl WildcardFilter {
```

#### `new`  <sub>(fn)</sub>

> given a boolean representing whether -D was used or not, create a new WildcardFilter

```rust
pub fn new(dont_filter: bool) -> Self {
```

#### `impl Default for WildcardFilter`  <sub>(impl)</sub>

> implement default that populates `method` with its default value

```rust
impl Default for WildcardFilter {
```

#### `default`  <sub>(fn)</sub>

```rust
fn default() -> Self {
```

#### `impl FeroxFilter for WildcardFilter`  <sub>(impl)</sub>

> implementation of FeroxFilter for WildcardFilter

```rust
impl FeroxFilter for WildcardFilter {
```

#### `should_filter_response`  <sub>(fn)</sub>

> Examine size/words/lines and method to determine whether or not the response received  
> is a wildcard response and therefore should be filtered out

```rust
fn should_filter_response(&self, response: &FeroxResponse) -> bool {
```

#### `box_eq`  <sub>(fn)</sub>

> Compare one WildcardFilter to another

```rust
fn box_eq(&self, other: &dyn Any) -> bool {
```

#### `as_any`  <sub>(fn)</sub>

> Return self as Any for dynamic dispatch purposes

```rust
fn as_any(&self) -> &dyn Any {
```

#### `impl std::fmt::Display for WildcardFilter`  <sub>(impl)</sub>

```rust
impl std::fmt::Display for WildcardFilter {
```

#### `fmt`  <sub>(fn)</sub>

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
```

### `src/filters/words.rs`


#### `WordsFilter`  <sub>(struct)</sub>

> Simple implementor of FeroxFilter; used to filter out responses based on the number of words  
> in a Response body; specified using -W|--filter-words

```rust
pub struct WordsFilter {
```

**Fields:**
- `pub word_count: usize,` — Number of words in a Response's body that should be filtered


#### `impl FeroxFilter for WordsFilter`  <sub>(impl)</sub>

> implementation of FeroxFilter for WordsFilter

```rust
impl FeroxFilter for WordsFilter {
```

#### `should_filter_response`  <sub>(fn)</sub>

> Check `word_count` against what was passed in via -W|--filter-words

```rust
fn should_filter_response(&self, response: &FeroxResponse) -> bool {
```

#### `box_eq`  <sub>(fn)</sub>

> Compare one WordsFilter to another

```rust
fn box_eq(&self, other: &dyn Any) -> bool {
```

#### `as_any`  <sub>(fn)</sub>

> Return self as Any for dynamic dispatch purposes

```rust
fn as_any(&self) -> &dyn Any {
```



## Module `nlp` <a id='module-nlp'></a>

*53 documented items*


### `src/nlp/constants.rs`


#### `ref`  <sub>(static)</sub>

> regular expression to match on words with numbers, underscores, and hyphens

```rust
pub(super) static ref BOUNDED_WORD_REGEX: Regex = Regex::new(r"\b[a-zA-Z0-9_-]+\b").unwrap();
```

#### `STOP_WORDS`  <sub>(static)</sub>

> collection of stop words from spaCy with small modifications

```rust
pub(super) static STOP_WORDS: [&str; 323] = [
    "'d",
    "'ll",
    "'m",
    "'re",
    "'s",
    "'ve",
    "a",
    "about",
    "above",
    "across",
    "after",
    "afterwards",
    "again",
    "against",
    "almost",
    "alone",
    "along",
    "already",
    "also",
    "although",
    "always",
    "am",
    "among",
    "amongst",
    "amount",
    "an",
```

### `src/nlp/document.rs`


#### `Document`  <sub>(struct)</sub>

> data container representing a single document, in the nlp sense

```rust
pub(crate) struct Document {
```

**Fields:**
- `terms: HashMap<Term, TermMetaData>,` — collection of `Term`s and their associated metadata
- `number_of_terms: usize,` — number of terms contained within the document


#### `impl Document`  <sub>(impl)</sub>

```rust
impl Document {
```

#### `new`  <sub>(fn)</sub>

> create a new `Document` from the given string

```rust
pub(super) fn new(text: &str) -> Self {
```

#### `add_term`  <sub>(fn)</sub>

> add a `Term` to the document if it's not already tracked, otherwise increment the number  
> of times the term has been seen

```rust
fn add_term(&mut self, word: &str) {
```

#### `from_html`  <sub>(fn)</sub>

> create a new `Document` from the given HTML string

```rust
pub(crate) fn from_html(raw_html: &str) -> Option<Self> {
```

#### `term_frequency`  <sub>(fn)</sub>

> Log normalized weighting scheme for term frequency

```rust
pub(super) fn term_frequency(&self, term: &Term) -> f32 {
```

#### `terms`  <sub>(fn)</sub>

> immutable reference to the collection of terms and their metadata

```rust
pub(super) fn terms(&self) -> &HashMap<Term, TermMetaData> {
```

#### `number_of_terms`  <sub>(fn)</sub>

> number of terms the current document knows about

```rust
fn number_of_terms(&self) -> usize {
```

#### `nlp_document_creation_from_text`  <sub>(fn)</sub>

> `Document::new` should preprocess text and generate a hashmap of `Term, TermMetadata`

```rust
fn nlp_document_creation_from_text() {
```

#### `nlp_document_creation_from_html`  <sub>(fn)</sub>

> `Document::new` should preprocess html and generate a hashmap of `Term, TermMetadata`

```rust
fn nlp_document_creation_from_html() {
```

#### `term_frequency_validation`  <sub>(fn)</sub>

> simple check of the `term_frequency` function's return value

```rust
fn term_frequency_validation() {
```

#### `document_accessor_test`  <sub>(fn)</sub>

> test accessors for correctness

```rust
fn document_accessor_test() {
```

#### `document_creation_skips_script_and_style_tags`  <sub>(fn)</sub>

> ensure words in script/style tags aren't processed

```rust
fn document_creation_skips_script_and_style_tags() {
```

### `src/nlp/model.rs`


#### `TfIdf`  <sub>(struct)</sub>

> data container for the TF-IDF model

```rust
pub(crate) struct TfIdf {
```

**Fields:**
- `terms: HashMap<Term, TermMetaData>,` — collection of `Term`s and their associated metadata
- `num_documents: usize,` — number of documents processed by the model


#### `impl TfIdf`  <sub>(impl)</sub>

```rust
impl TfIdf {
```

#### `new`  <sub>(fn)</sub>

> create an empty TF-IDF model; must be populated with `add_document` prior to use

```rust
pub(crate) fn new() -> Self {
```

#### `terms`  <sub>(fn)</sub>

> accessor method for the collection of `Term`s and `TermMetaData`

```rust
fn terms(&self) -> &HashMap<Term, TermMetaData> {
```

#### `num_documents`  <sub>(fn)</sub>

> accessor method for the number of `Document`s the model has processed

```rust
pub(crate) fn num_documents(&self) -> usize {
```

#### `add_document`  <sub>(fn)</sub>

> add a `Document` to the model

```rust
pub(crate) fn add_document(&mut self, document: Document) {
```

#### `calculate_tf_idf_scores`  <sub>(fn)</sub>

> (re)-calculate tf-idf scores for all terms, given the current number of documents  
>   
> # Notes  
>   
> old tf-idf scores are removed during calculations to keep new `Term`s at the same relative  
> level as new ones WRT corpus size

```rust
pub(crate) fn calculate_tf_idf_scores(&mut self) {
```

#### `all_words`  <sub>(fn)</sub>

> select all terms with a non-zero tf-idf score

```rust
pub(crate) fn all_words(&self) -> Vec<String> {
```

#### `get_score`  <sub>(fn)</sub>

> helper for this test suite

```rust
fn get_score(word: &str, model: &TfIdf) -> f32 {
```

#### `model_generates_expected_tf_idf_scores`  <sub>(fn)</sub>

> given the example data at https://remykarem.github.io/tfidf-demo/, ensure the model  
> produces the same results

```rust
fn model_generates_expected_tf_idf_scores() {
```

#### `select_n_words_grabs_correct_words`  <sub>(fn)</sub>

> given the example data at https://remykarem.github.io/tfidf-demo/, ensure the model  
> produces the same results

```rust
fn select_n_words_grabs_correct_words() {
```

### `src/nlp/term.rs`


#### `Term`  <sub>(struct)</sub>

> single word term for text processing

```rust
pub(crate) struct Term {
```

**Fields:**
- `raw: String,` — underlying string that the term represents


#### `impl Term`  <sub>(impl)</sub>

```rust
impl Term {
```

#### `new`  <sub>(fn)</sub>

> given a word, create a new `Term`

```rust
pub(super) fn new(word: &str) -> Self {
```

#### `raw`  <sub>(fn)</sub>

> return a reference to the underlying string

```rust
pub(super) fn raw(&self) -> &str {
```

#### `TermMetaData`  <sub>(struct)</sub>

> metadata to be associated with a `Term`  
>   
> # Design Note  
>   
> The `count` field represents the number of times a term appeared in a **single document**  
> and is only meaningful in the per-document context (i.e., within a `Document`).  
>   
> When `TermMetaData` is stored in the global `TfIdf` model, the `count` field becomes stale  
> and is not used. Instead, the model relies on `term_frequencies` (which tracks the term  
> frequency for each document the term appears in) and calculates TF-IDF scores from those.

```rust
pub(super) struct TermMetaData {
```

**Fields:**
- `count: u32,` — number of times the associated `Term` was seen in a single document
- `term_frequencies: Vec<f32>,` — collection of term frequencies for the associated `Term`
- `tf_idf_score: f32,` — tf-idf score for the associated `Term`


#### `impl TermMetaData`  <sub>(impl)</sub>

```rust
impl TermMetaData {
```

#### `document_frequency`  <sub>(fn)</sub>

> number of times a `Term` has appeared in any `Document` within the corpus

```rust
pub(super) fn document_frequency(&self) -> usize {
```

#### `term_frequencies_mut`  <sub>(fn)</sub>

> mutable reference to the collection of term frequencies

```rust
pub(super) fn term_frequencies_mut(&mut self) -> &mut Vec<f32> {
```

#### `term_frequencies`  <sub>(fn)</sub>

> immutable reference to the collection of term frequencies

```rust
pub(super) fn term_frequencies(&self) -> &[f32] {
```

#### `count_mut`  <sub>(fn)</sub>

> mutable reference to the number of times a `Term` was seen in a particular `Document`

```rust
pub(super) fn count_mut(&mut self) -> &mut u32 {
```

#### `count`  <sub>(fn)</sub>

> number of times a `Term` was seen in a particular `Document`

```rust
pub(super) fn count(&self) -> u32 {
```

#### `tf_idf_score_mut`  <sub>(fn)</sub>

> mutable reference to the term's tf-idf score

```rust
pub(super) fn tf_idf_score_mut(&mut self) -> &mut f32 {
```

#### `tf_idf_score`  <sub>(fn)</sub>

> immutable reference to the term's tf-idf score

```rust
pub(super) fn tf_idf_score(&self) -> f32 {
```

#### `nlp_term_accessor_test`  <sub>(fn)</sub>

> test accessors for correctness

```rust
fn nlp_term_accessor_test() {
```

#### `nlp_term_metadata_accessor_test`  <sub>(fn)</sub>

> test accessors for correctness

```rust
fn nlp_term_metadata_accessor_test() {
```

### `src/nlp/utils.rs`


#### `preprocess`  <sub>(fn)</sub>

> pre-processing pipeline wrapper that removes punctuation, normalizes word case (utf-8 included)  
> to lowercase, and remove stop words

```rust
pub(crate) fn preprocess(text: &str) -> Vec<String> {
```

#### `normalize_case`  <sub>(fn)</sub>

> optimized version of `str::to_lowercase`

```rust
fn normalize_case<'a, S: Into<Cow<'a, str>>>(input: S) -> Cow<'a, str> {
```

#### `remove_punctuation`  <sub>(fn)</sub>

> replace ascii and some utf-8 punctuation characters with ' ' (space) in the given string

```rust
fn remove_punctuation(text: &str) -> String {
```

#### `remove_stop_words`  <sub>(fn)</sub>

> remove stop words from the given string

```rust
fn remove_stop_words(text: &str) -> String {
```

#### `inverse_document_frequency`  <sub>(fn)</sub>

> calculate inverse document frequency

```rust
pub(super) fn inverse_document_frequency(num_docs: f32, doc_frequency: f32) -> f32 {
```

#### `tf_idf_score`  <sub>(fn)</sub>

> calculate term frequency-inverse document frequency (tf-idf)

```rust
pub(super) fn tf_idf_score(term_frequency: f32, idf: f32) -> f32 {
```

#### `test_remove_punctuation`  <sub>(fn)</sub>

> ensure all expected punctuation characters are removed

```rust
fn test_remove_punctuation() {
```

#### `test_normalize_case`  <sub>(fn)</sub>

> ensure uppercase characters are swapped to lowercase

```rust
fn test_normalize_case() {
```

#### `test_remove_stopwords`  <sub>(fn)</sub>

> ensure all stop words are removed from the list of stopwords ... intestuous

```rust
fn test_remove_stopwords() {
```

#### `test_preprocess_results`  <sub>(fn)</sub>

> ensure preprocess

```rust
fn test_preprocess_results() {
```

#### `idf_returns_expected_value`  <sub>(fn)</sub>

> ensure our calculations conform to the example provided at the link below  
>   
> https://www.kaggle.com/paulrohan2020/tf-idf-tutorial/notebook#TF-IDF-Model  
>   
> Consider a document containing 100 words wherein the word cat appears 3 times.  
> The term frequency (i.e., tf) for cat is then (3 / 100) = 0.03. Now, assume we have 10  
> million documents and the word cat appears in one thousand of these. Then, the inverse  
> document frequency (i.e., idf) is calculated as log(10,000,000 / 1,000) = 4. Thus, the  
> Tf-idf weight is the product of these quantities: 0.03 * 4 = 0.12.

```rust
fn idf_returns_expected_value() {
```

#### `tf_idf_returns_expected_value`  <sub>(fn)</sub>

> ensure our calculations conform to the example provided at the link below  
>   
> https://www.kaggle.com/paulrohan2020/tf-idf-tutorial/notebook#TF-IDF-Model  
>   
> Consider a document containing 100 words wherein the word cat appears 3 times.  
> The term frequency (i.e., tf) for cat is then (3 / 100) = 0.03. Now, assume we have 10  
> million documents and the word cat appears in one thousand of these. Then, the inverse  
> document frequency (i.e., idf) is calculated as log(10,000,000 / 1,000) = 4. Thus, the  
> Tf-idf weight is the product of these quantities: 0.03 * 4 = 0.12.

```rust
fn tf_idf_returns_expected_value() {
```



## Module `statistics` <a id='module-statistics'></a>

*42 documented items*


### `src/statistics/container.rs`


#### `Stats`  <sub>(struct)</sub>

> Data collection of statistics related to a scan

```rust
pub struct Stats {
```

**Fields:**
- `kind: String,` — Name of this type of struct, used for serialization, i.e. `{"type":"statistics"}`
- `timeouts: AtomicUsize,` — tracker for number of timeouts seen by the client
- `pub(crate) requests: AtomicUsize,` — tracker for total number of requests sent by the client
- `expected_per_scan: AtomicUsize,` — indicative of the current expectation at any given time, but is a moving target.
- `total_expected: AtomicUsize,` — this value should increase by `expected_requests`
- `pub(crate) errors: AtomicUsize,` — tracker for total number of errors encountered by the client
- `successes: AtomicUsize,` — tracker for overall number of 2xx status codes seen by the client
- `redirects: AtomicUsize,` — tracker for overall number of 3xx status codes seen by the client
- `client_errors: AtomicUsize,` — tracker for overall number of 4xx status codes seen by the client
- `server_errors: AtomicUsize,` — tracker for overall number of 5xx status codes seen by the client
- `pub(crate) total_scans: AtomicUsize,` — recursed into and affects the total number of expected requests
- `initial_targets: AtomicUsize,` — tracker for initial number of requested targets
- `links_extracted: AtomicUsize,` — response bodies and robots.txt as of v1.11.0
- `extensions_collected: AtomicUsize,` — are response bodies
- `status_200s: AtomicUsize,` — tracker for overall number of 200s seen by the client
- `status_301s: AtomicUsize,` — tracker for overall number of 301s seen by the client
- `status_302s: AtomicUsize,` — tracker for overall number of 302s seen by the client
- `status_401s: AtomicUsize,` — tracker for overall number of 401s seen by the client
- `pub(crate) status_403s: AtomicUsize,` — tracker for overall number of 403s seen by the client
- `pub(crate) status_429s: AtomicUsize,` — tracker for overall number of 429s seen by the client
- `status_500s: AtomicUsize,` — tracker for overall number of 500s seen by the client
- `status_503s: AtomicUsize,` — tracker for overall number of 503s seen by the client
- `status_504s: AtomicUsize,` — tracker for overall number of 504s seen by the client
- `status_508s: AtomicUsize,` — tracker for overall number of 508s seen by the client
- `wildcards_filtered: AtomicUsize,` — tracker for overall number of wildcard urls filtered out by the client
- `responses_filtered: AtomicUsize,` — tracker for overall number of all filtered responses
- `resources_discovered: AtomicUsize,` — tracker for number of files found
- `url_format_errors: AtomicUsize,` — tracker for number of errors triggered during URL formatting
- `redirection_errors: AtomicUsize,` — tracker for number of errors triggered by the `reqwest::RedirectPolicy`
- `connection_errors: AtomicUsize,` — tracker for number of errors related to the connecting
- `request_errors: AtomicUsize,` — tracker for number of errors related to the request used
- `certificate_errors: AtomicUsize,` — tracker for number of certificate/TLS/SSL errors
- `directory_scan_times: Mutex<Vec<f64>>,` — tracker for each directory's total scan time in seconds as a float
- `total_runtime: Mutex<Vec<f64>>,` — tracker for total runtime
- `json: bool,` — tracker for whether to use json during serialization or not
- `targets: Mutex<Vec<String>>,` — tracker for the initial targets that were passed in to the scan


#### `impl FeroxSerialize for Stats`  <sub>(impl)</sub>

> FeroxSerialize implementation for Stats

```rust
impl FeroxSerialize for Stats {
```

#### `as_str`  <sub>(fn)</sub>

> Simply return empty string here to disable serializing this to the output file as a string  
> due to it looking like garbage

```rust
fn as_str(&self) -> String {
```

#### `as_json`  <sub>(fn)</sub>

> Simple call to produce a JSON string using the given Stats object

```rust
fn as_json(&self) -> Result<String> {
```

#### `impl Serialize for Stats`  <sub>(impl)</sub>

> Serialize implementation for Stats

```rust
impl Serialize for Stats {
```

#### `serialize`  <sub>(fn)</sub>

> Function that handles serialization of Stats

```rust
fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    {
```

#### `impl<'a> Deserialize<'a> for Stats`  <sub>(impl)</sub>

> Deserialize implementation for Stats

```rust
impl<'a> Deserialize<'a> for Stats {
```

#### `deserialize`  <sub>(fn)</sub>

> Deserialize a Stats object from a serde_json::Value

```rust
fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
where
    D: Deserializer<'a>,
    {
```

#### `impl Stats`  <sub>(impl)</sub>

> implementation of statistics data collection struct

```rust
impl Stats {
```

#### `new`  <sub>(fn)</sub>

> Small wrapper for default to set `kind` to "statistics" and `total_runtime` to have at least  
> one value

```rust
pub fn new(is_json: bool) -> Self {
```

#### `expected_per_scan`  <sub>(fn)</sub>

> public getter for expected_per_scan

```rust
pub fn expected_per_scan(&self) -> usize {
```

#### `resources_discovered`  <sub>(fn)</sub>

> public getter for resources_discovered

```rust
pub fn resources_discovered(&self) -> usize {
```

#### `errors`  <sub>(fn)</sub>

> public getter for errors

```rust
pub fn errors(&self) -> usize {
```

#### `status_403s`  <sub>(fn)</sub>

> public getter for status_403s

```rust
pub fn status_403s(&self) -> usize {
```

#### `status_429s`  <sub>(fn)</sub>

> public getter for status_429s

```rust
pub fn status_429s(&self) -> usize {
```

#### `total_expected`  <sub>(fn)</sub>

> public getter for total_expected

```rust
pub fn total_expected(&self) -> usize {
```

#### `initial_targets`  <sub>(fn)</sub>

> public getter for initial_targets

```rust
pub fn initial_targets(&self) -> usize {
```

#### `add_request`  <sub>(fn)</sub>

> increment `requests` field by one

```rust
pub fn add_request(&self) {
```

#### `update_runtime`  <sub>(fn)</sub>

> given an `Instant` update total runtime

```rust
fn update_runtime(&self, seconds: f64) {
```

#### `update_targets`  <sub>(fn)</sub>

> update targets with the given vector of strings

```rust
pub fn update_targets(&self, targets: Vec<String>) {
```

#### `save`  <sub>(fn)</sub>

> save an instance of `Stats` to disk after updating the total runtime for the scan

```rust
pub fn save(&self, seconds: f64, location: &str) -> Result<()> {
```

#### `add_error`  <sub>(fn)</sub>

> Inspect the given `StatError` and increment the appropriate fields  
>   
> Implies incrementing:  
> - requests  
> - errors

```rust
pub fn add_error(&self, error: StatError) {
```

#### `add_status_code`  <sub>(fn)</sub>

> Inspect the given `StatusCode` and increment the appropriate fields  
>   
> Implies incrementing:  
> - requests  
> - appropriate status_* codes  
> - errors (when code is [45]xx)

```rust
pub fn add_status_code(&self, status: StatusCode) {
```

#### `update_f64_field`  <sub>(fn)</sub>

> Update a `Stats` field of type f64

```rust
pub fn update_f64_field(&self, field: StatField, value: f64) {
```

#### `subtract_from_usize_field`  <sub>(fn)</sub>

> subtract a value from the given field

```rust
pub fn subtract_from_usize_field(&self, field: StatField, value: usize) {
```

#### `update_usize_field`  <sub>(fn)</sub>

> Update a `Stats` field of type usize

```rust
pub fn update_usize_field(&self, field: StatField, value: usize) {
```

#### `merge_from`  <sub>(fn)</sub>

> Merge a given `Stats` object from a json entry written to disk when handling a Ctrl+c  
>   
> This is only ever called when resuming a scan from disk

```rust
pub fn merge_from(&self, filename: &str) -> Result<()> {
```

#### `statistics_handler_increments_requests`  <sub>(fn)</sub>

> when sent StatCommand::AddRequest, stats object should reflect the change

```rust
async fn statistics_handler_increments_requests() -> Result<()> {
```

#### `statistics_handler_increments_403_via_status_code`  <sub>(fn)</sub>

> when sent StatCommand::AddRequest, stats object should reflect the change  
>   
> incrementing a 403 (tracked in status_403s) should also increment:  
> - requests  
> - client_errors

```rust
async fn statistics_handler_increments_403_via_status_code() {
```

#### `statistics_handler_increments_500_via_status_code`  <sub>(fn)</sub>

> when sent StatCommand::AddStatus, stats object should reflect the change  
>   
> incrementing a 500 (tracked in server_errors) should also increment:  
> - requests

```rust
async fn statistics_handler_increments_500_via_status_code() -> Result<()> {
```

#### `stats_increments_timeouts`  <sub>(fn)</sub>

> when Stats::add_error receives StatError::Timeout, it should increment the following:  
> - timeouts  
> - requests  
> - errors

```rust
fn stats_increments_timeouts() {
```

#### `stats_increments_wildcards`  <sub>(fn)</sub>

> when Stats::update_usize_field receives StatField::WildcardsFiltered, it should increment  
> the following:  
> - responses_filtered

```rust
fn stats_increments_wildcards() {
```

#### `stats_increments_responses_filtered`  <sub>(fn)</sub>

> when Stats::update_usize_field receives StatField::ResponsesFiltered, it should increment

```rust
fn stats_increments_responses_filtered() {
```

#### `stats_merge_from_alters_correct_fields`  <sub>(fn)</sub>

> Stats::merge_from should properly increment expected fields and ignore others

```rust
fn stats_merge_from_alters_correct_fields() {
```

#### `update_runtime_works`  <sub>(fn)</sub>

> ensure update runtime overwrites the default 0th entry

```rust
fn update_runtime_works() {
```

#### `status_403s_returns_correct_value`  <sub>(fn)</sub>

> ensure status_403s returns the correct value

```rust
fn status_403s_returns_correct_value() {
```

#### `status_429s_returns_correct_value`  <sub>(fn)</sub>

> ensure status_403s returns the correct value

```rust
fn status_429s_returns_correct_value() {
```

### `src/statistics/error.rs`


#### `StatError`  <sub>(enum)</sub>

> Enum variants used to inform the `StatCommand` protocol what `Stats` fields should be updated

```rust
pub enum StatError {
```

**Variants:**
- `Timeout,` — Represents a timeout error
- `UrlFormat,` — Represents a URL formatting error
- `Redirection,` — Represents an error encountered during redirection
- `Connection,` — Represents an error encountered during connection
- `Request,` — Represents an error resulting from the client's request
- `Certificate,` — Represents certificate-related errors (TLS/SSL)
- `Other,` — Represents any other error not explicitly defined above


### `src/statistics/field.rs`


#### `StatField`  <sub>(enum)</sub>

> Enum representing fields whose updates need to be performed in batches instead of one at  
> a time

```rust
pub enum StatField {
```

**Variants:**
- `ExpectedPerScan,` — the `expected_per_scan` field after initialization
- `TotalScans,` — Translates to `total_scans`
- `LinksExtracted,` — Translates to `links_extracted`
- `ExtensionsCollected,` — Translates to `extensions_collected`
- `TotalExpected,` — Translates to `total_expected`
- `WildcardsFiltered,` — Translates to `wildcards_filtered`
- `ResponsesFiltered,` — Translates to `responses_filtered`
- `ResourcesDiscovered,` — Translates to `resources_discovered`
- `InitialTargets,` — Translates to `initial_targets`
- `DirScanTimes,` — Translates to `directory_scan_times`; assumes a single append to the vector


### `src/statistics/macros.rs`


#### `macro atomic_increment`  <sub>(macro)</sub>

> Wrapper `Atomic*.fetch_add` to save me from writing Ordering::Relaxed a bajillion times  
>   
> default is to increment by 1, second arg can be used to increment by a different value

```rust
macro_rules! atomic_increment {
```

#### `macro atomic_load`  <sub>(macro)</sub>

> Wrapper around `Atomic*.load` to save me from writing Ordering::Relaxed a bajillion times

```rust
macro_rules! atomic_load {
```

#### `macro atomic_store`  <sub>(macro)</sub>

> Wrapper around `Atomic*.store` to save me from writing Ordering::Relaxed a bajillion times

```rust
macro_rules! atomic_store {
```



## Module `banner` <a id='module-banner'></a>

*17 documented items*


### `src/banner/container.rs`


#### `UPDATE_URL`  <sub>(const)</sub>

> Url used to query github's api; specifically used to look for the latest tagged release name

```rust
pub const UPDATE_URL: &str = "https://api.github.com/repos/epi052/feroxbuster/releases/latest";
```

#### `UpdateStatus`  <sub>(enum)</sub>

> Simple enum to hold three different update states

```rust
pub(super) enum UpdateStatus {
```

**Variants:**
- `UpToDate,` — this version and latest release are the same
- `OutOfDate,` — this version and latest release are not the same
- `Unknown,` — some error occurred during version check


#### `Banner`  <sub>(struct)</sub>

> Banner object, contains multiple BannerEntry's and knows how to display itself

```rust
pub struct Banner {
```

**Fields:**
- `targets: Vec<BannerEntry>,` — all live targets
- `status_codes: BannerEntry,` — represents Configuration.status_codes
- `filter_status: BannerEntry,` — represents Configuration.filter_status
- `threads: BannerEntry,` — represents Configuration.threads
- `wordlist: BannerEntry,` — represents Configuration.wordlist
- `timeout: BannerEntry,` — represents Configuration.timeout
- `user_agent: BannerEntry,` — represents Configuration.user_agent
- `random_agent: BannerEntry,` — represents Configuration.random_agent
- `config: BannerEntry,` — represents Configuration.config
- `proxy: BannerEntry,` — represents Configuration.proxy
- `client_key: BannerEntry,` — represents Configuration.client_key
- `client_cert: BannerEntry,` — represents Configuration.client_cert
- `server_certs: BannerEntry,` — represents Configuration.server_certs
- `replay_proxy: BannerEntry,` — represents Configuration.replay_proxy
- `replay_codes: BannerEntry,` — represents Configuration.replay_codes
- `headers: Vec<BannerEntry>,` — represents Configuration.headers
- `filter_size: Vec<BannerEntry>,` — represents Configuration.filter_size
- `filter_similar: Vec<BannerEntry>,` — represents Configuration.filter_similar
- `filter_word_count: Vec<BannerEntry>,` — represents Configuration.filter_word_count
- `filter_line_count: Vec<BannerEntry>,` — represents Configuration.filter_line_count
- `filter_regex: Vec<BannerEntry>,` — represents Configuration.filter_regex
- `extract_links: BannerEntry,` — represents Configuration.extract_links
- `json: BannerEntry,` — represents Configuration.json
- `output: BannerEntry,` — represents Configuration.output
- `debug_log: BannerEntry,` — represents Configuration.debug_log
- `extensions: BannerEntry,` — represents Configuration.extensions
- `methods: BannerEntry,` — represents Configuration.methods
- `data: BannerEntry,` — represents Configuration.data
- `insecure: BannerEntry,` — represents Configuration.insecure
- `redirects: BannerEntry,` — represents Configuration.redirects
- `dont_filter: BannerEntry,` — represents Configuration.dont_filter
- `queries: Vec<BannerEntry>,` — represents Configuration.queries
- `verbosity: BannerEntry,` — represents Configuration.verbosity
- `add_slash: BannerEntry,` — represents Configuration.add_slash
- `no_recursion: BannerEntry,` — represents Configuration.no_recursion
- `scan_limit: BannerEntry,` — represents Configuration.scan_limit
- `time_limit: BannerEntry,` — represents Configuration.time_limit
- `rate_limit: BannerEntry,` — represents Configuration.rate_limit
- `parallel: BannerEntry,` — represents Configuration.parallel
- `auto_tune: BannerEntry,` — represents Configuration.auto_tune
- `auto_bail: BannerEntry,` — represents Configuration.auto_bail
- `url_denylist: Vec<BannerEntry>,` — represents Configuration.url_denylist
- `scope: Vec<BannerEntry>,` — represents Configuration.scope
- `pub(super) version: String,` — current version of feroxbuster
- `pub(super) update_status: UpdateStatus,` — whether or not there is a known new version
- `collect_extensions: BannerEntry,` — represents Configuration.collect_extensions
- `dont_collect: BannerEntry,` — represents Configuration.dont_collect
- `collect_backups: BannerEntry,` — represents Configuration.collect_backups
- `collect_words: BannerEntry,` — represents Configuration.collect_words
- `force_recursion: BannerEntry,` — represents Configuration.collect_words
- `protocol: BannerEntry,` — represents Configuration.protocol
- `scan_dir_listings: BannerEntry,` — represents Configuration.scan_dir_listings
- `limit_bars: BannerEntry,` — represents Configuration.limit_bars
- `unique: BannerEntry,` — represents Configuration.unique
- `response_size_limit: BannerEntry,` — represents Configuration.response_size_limit


#### `impl Banner`  <sub>(impl)</sub>

> implementation of Banner

```rust
impl Banner {
```

#### `new`  <sub>(fn)</sub>

> Create a new Banner from a Configuration and live targets

```rust
pub fn new(tgts: &[String], config: &Configuration) -> Self {
```

#### `header`  <sub>(fn)</sub>

> get a fancy header for the banner

```rust
fn header(&self) -> String {
```

#### `footer`  <sub>(fn)</sub>

> get a fancy footer for the banner

```rust
fn footer(&self) -> String {
```

#### `check_for_updates`  <sub>(fn)</sub>

> Makes a request to the given url, expecting to receive a JSON response that contains a field  
> named `tag_name` that holds a value representing the latest tagged release of this tool.  
>   
> ex: v1.1.0

```rust
pub async fn check_for_updates(&mut self, url: &str, handles: Arc<Handles>) -> Result<()> {
```

#### `print_to`  <sub>(fn)</sub>

> display the banner on Write writer

```rust
pub fn print_to<W>(&self, mut writer: W, config: Arc<Configuration>) -> Result<()>
where
    W: Write,
    {
```

### `src/banner/entry.rs`


#### `INDENT`  <sub>(const)</sub>

> Initial visual indentation size used in formatting banner entries

```rust
const INDENT: usize = 3;
```

#### `COL_WIDTH`  <sub>(const)</sub>

> Column width used in formatting banner entries

```rust
const COL_WIDTH: usize = 22;
```

#### `BannerEntry`  <sub>(struct)</sub>

> Represents a single line on the banner

```rust
pub(super) struct BannerEntry {
```

**Fields:**
- `emoji: String,` — emoji used in the banner entry
- `title: String,` — title used in the banner entry
- `value: String,` — value passed in via config/cli/defaults


#### `impl BannerEntry`  <sub>(impl)</sub>

> implementation of a banner entry

```rust
impl BannerEntry {
```

#### `new`  <sub>(fn)</sub>

> Create a new banner entry from given fields

```rust
pub fn new(emoji: &str, title: &str, value: &str) -> Self {
```

#### `format_emoji`  <sub>(fn)</sub>

> Simple wrapper for emoji or fallback when terminal doesn't support emoji

```rust
fn format_emoji(&self) -> String {
```

#### `impl fmt::Display for BannerEntry`  <sub>(impl)</sub>

> Display implementation for a banner entry

```rust
impl fmt::Display for BannerEntry {
```

#### `fmt`  <sub>(fn)</sub>

> Display formatter for the given banner entry

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
```



## Module `sync` <a id='module-sync'></a>

*26 documented items*


### `src/sync/dynamic_semaphore.rs`


#### `DynamicSemaphore`  <sub>(struct)</sub>

> A wrapper around Tokio's [`Semaphore`] that supports dynamic capacity reduction.  
>   
> Unlike the standard Tokio semaphore, this implementation allows for reduction of the  
> effective capacity even when permits are already acquired and other tasks are waiting.  
> This is particularly useful for rate limiting scenarios where we need to dynamically  
> adjust the concurrency level based on runtime conditions.  
>   
> # Key Features  
>   
> - **Dynamic Capacity Reduction**: Can reduce capacity even when permits are in use  
> - **Queued Waiter Preservation**: Existing waiters remain in queue during capacity changes  
> - **Thread-Safe**: All operations are atomic and safe for concurrent use  
> - **Drop Safety**: Automatically manages capacity when permits are released  
>   
> # Example  
>   
> ```rust,no_run  
> use feroxbuster::sync::DynamicSemaphore;  
>   
> #[tokio::main]  
> async fn main() {  
> let semaphore = DynamicSemaphore::new(2);  
>   
> // Acquire permits  
> let _permit1 = semaphore.acquire().await.unwrap();  
> let _permit2 = semaphore.acquire().await.unwrap();  
>   
> // Reduce capacity from 2 to 1 (takes effect when permits are released)  
> semaphore.reduce_capacity(1);  
>   
> // When permits are dropped, only 1 permit will be available instead of 2  
> }  
> ```

```rust
pub struct DynamicSemaphore {
```

**Fields:**
- `inner: Arc<Semaphore>,` — The underlying Tokio semaphore that handles the actual permit management
- `max_capacity: AtomicUsize,` — available permits never exceed this capacity.
- `permits_in_use: AtomicUsize,` — We use this to track how many permits are actually in use vs the virtual capacity.


#### `DynamicSemaphorePermit`  <sub>(struct)</sub>

> A permit acquired from a [`DynamicSemaphore`].  
>   
> This permit automatically manages the dynamic capacity when dropped. If releasing  
> the permit would cause the semaphore to exceed its current capacity limit, the  
> permit is "forgotten" instead of being returned to the available pool.  
>   
> The permit provides the same guarantees as Tokio's [`SemaphorePermit`] but with  
> additional capacity management logic.

```rust
pub struct DynamicSemaphorePermit<'a> {
```

**Fields:**
- `permit: Option<SemaphorePermit<'a>>,` — capacity management in the Drop implementation.
- `semaphore: &'a DynamicSemaphore,` — Reference to the parent semaphore for capacity checking


#### `impl DynamicSemaphore`  <sub>(impl)</sub>

```rust
impl DynamicSemaphore {
```

#### `new`  <sub>(fn)</sub>

> Creates a new [`DynamicSemaphore`] with the specified number of permits.  
>   
> # Arguments  
>   
> * `permits` - The initial number of permits available in the semaphore  
>   
> # Panics  
>   
> Panics if `permits` exceeds the maximum number of permits supported by  
> the underlying Tokio semaphore implementation.  
>   
> # Examples  
>   
> ```rust,no_run  
> use feroxbuster::sync::DynamicSemaphore;  
>   
> let semaphore = DynamicSemaphore::new(10);  
> assert_eq!(semaphore.current_capacity(), 10);  
> ```

```rust
pub fn new(permits: usize) -> Self {
```

#### `acquire`  <sub>(fn)</sub>

> Acquires a permit from the semaphore.  
>   
> This method will wait until a permit becomes available. The returned permit  
> will automatically manage capacity constraints when dropped.  
>   
> # Returns  
>   
> A [`Result`] containing a [`DynamicSemaphorePermit`] on success, or an  
> [`tokio::sync::AcquireError`] if the semaphore has been closed.  
>   
> # Examples  
>   
> ```rust,no_run  
> use feroxbuster::sync::DynamicSemaphore;  
>   
> #[tokio::main]  
> async fn main() {  
> let semaphore = DynamicSemaphore::new(1);  
> let permit = semaphore.acquire().await.unwrap();  
> // permit is automatically released when dropped  
> }  
> ```

```rust
pub async fn acquire(&self) -> Result<DynamicSemaphorePermit<'_>, tokio::sync::AcquireError> {
```

#### `try_acquire`  <sub>(fn)</sub>

> Attempts to acquire a permit without waiting.  
>   
> If a permit is immediately available, it is returned. Otherwise, this method  
> returns an error indicating why the permit could not be acquired.  
>   
> # Returns  
>   
> A [`Result`] containing a [`DynamicSemaphorePermit`] if successful, or a  
> [`tokio::sync::TryAcquireError`] if no permit is available or the semaphore is closed.  
>   
> # Examples  
>   
> ```rust,no_run  
> use feroxbuster::sync::DynamicSemaphore;  
> use tokio::sync::TryAcquireError;  
>   
> let semaphore = DynamicSemaphore::new(1);  
> match semaphore.try_acquire() {  
> Ok(permit) => println!("Got permit"),  
> Err(TryAcquireError::NoPermits) => println!("No permits available"),  
> Err(TryAcquireError::Closed) => println!("Semaphore closed"),  
> };  
> ```

```rust
pub fn try_acquire(&self) -> Result<DynamicSemaphorePermit<'_>, tokio::sync::TryAcquireError> {
```

#### `reduce_capacity`  <sub>(fn)</sub>

> Reduces the maximum capacity of the semaphore.  
>   
> This method sets a new maximum capacity for the semaphore. The change takes  
> effect immediately for new permit acquisitions. If there are currently more  
> permits in use than the new capacity allows, the reduction will take effect  
> gradually as permits are released.  
>   
> # Arguments  
>   
> * `new_capacity` - The new maximum number of permits that should be available  
>   
> # Returns  
>   
> The previous capacity value before the change.  
>   
> # Notes  
>   
> - This operation is atomic and thread-safe  
> - Existing permit holders are not affected until they release their permits  
> - Queued waiters remain in the queue and will eventually be served  
> - If available permits exceed the new capacity, excess permits are immediately forgotten  
>   
> # Examples  
>   
> ```rust,no_run  
> use feroxbuster::sync::DynamicSemaphore;  
>   
> #[tokio::main]  
> async fn main() {  
> let semaphore = DynamicSemaphore::new(5);  
>   
> // Reduce capacity from 5 to 2  
> let old_capacity = semaphore.reduce_capacity(2);  
> assert_eq!(old_capacity, 5);  
> assert_eq!(semaphore.current_capacity(), 2);  
> }  
> ```

```rust
pub fn reduce_capacity(&self, new_capacity: usize) -> usize {
```

#### `increase_capacity`  <sub>(fn)</sub>

> Increases the maximum capacity of the semaphore.  
>   
> This method sets a new maximum capacity that is higher than the current one.  
> Additional permits are immediately added to the semaphore up to the new capacity.  
>   
> # Arguments  
>   
> * `new_capacity` - The new maximum number of permits that should be available  
>   
> # Returns  
>   
> The previous capacity value before the change.  
>   
> # Panics  
>   
> Panics if the new capacity would cause the semaphore to exceed its maximum  
> supported permit count.  
>   
> # Examples  
>   
> ```rust,no_run  
> use feroxbuster::sync::DynamicSemaphore;  
>   
> #[tokio::main]  
> async fn main() {  
> let semaphore = DynamicSemaphore::new(2);  
>   
> // Increase capacity from 2 to 5  
> let old_capacity = semaphore.increase_capacity(5);  
> assert_eq!(old_capacity, 2);  
> assert_eq!(semaphore.current_capacity(), 5);  
> }  
> ```

```rust
pub fn increase_capacity(&self, new_capacity: usize) -> usize {
```

#### `current_capacity`  <sub>(fn)</sub>

> Returns the current maximum capacity of the semaphore.  
>   
> This represents the maximum number of permits that can be available at any  
> given time, which may be different from the number of currently available permits.  
>   
> # Examples  
>   
> ```rust,no_run  
> use feroxbuster::sync::DynamicSemaphore;  
>   
> let semaphore = DynamicSemaphore::new(10);  
> assert_eq!(semaphore.current_capacity(), 10);  
> ```

```rust
pub fn current_capacity(&self) -> usize {
```

#### `available_permits`  <sub>(fn)</sub>

> Returns the number of permits currently available for immediate acquisition.  
>   
> This value represents permits that can be acquired without waiting. Note that  
> this number may be less than the capacity if permits are currently in use.  
>   
> # Examples  
>   
> ```rust,no_run  
> use feroxbuster::sync::DynamicSemaphore;  
>   
> #[tokio::main]  
> async fn main() {  
> let semaphore = DynamicSemaphore::new(3);  
> assert_eq!(semaphore.available_permits(), 3);  
>   
> let _permit = semaphore.acquire().await.unwrap();  
> assert_eq!(semaphore.available_permits(), 2);  
> }  
> ```

```rust
pub fn available_permits(&self) -> usize {
```

#### `close`  <sub>(fn)</sub>

> Closes the semaphore, preventing new permits from being acquired.  
>   
> This will wake up all tasks currently waiting to acquire a permit, causing  
> them to receive an [`tokio::sync::AcquireError`]. Existing permits remain  
> valid until dropped.  
>   
> # Examples  
>   
> ```rust,no_run  
> use feroxbuster::sync::DynamicSemaphore;  
>   
> #[tokio::main]  
> async fn main() {  
> let semaphore = DynamicSemaphore::new(1);  
> semaphore.close();  
>   
> // This will return an error  
> assert!(semaphore.acquire().await.is_err());  
> }  
> ```

```rust
pub fn close(&self) {
```

#### `is_closed`  <sub>(fn)</sub>

> Returns whether the semaphore has been closed.  
>   
> # Examples  
>   
> ```rust,no_run  
> use feroxbuster::sync::DynamicSemaphore;  
>   
> let semaphore = DynamicSemaphore::new(1);  
> assert!(!semaphore.is_closed());  
>   
> semaphore.close();  
> assert!(semaphore.is_closed());  
> ```

```rust
pub fn is_closed(&self) -> bool {
```

#### `permits_in_use`  <sub>(fn)</sub>

> Returns the current number of permits in use (for debugging).  
>   
> This is primarily useful for debugging and testing to understand  
> the internal state of the semaphore.  
>   
> # Examples  
>   
> ```rust,no_run  
> use feroxbuster::sync::DynamicSemaphore;  
>   
> #[tokio::main]  
> async fn main() {  
> let semaphore = DynamicSemaphore::new(3);  
> assert_eq!(semaphore.permits_in_use(), 0);  
>   
> let _permit = semaphore.acquire().await.unwrap();  
> assert_eq!(semaphore.permits_in_use(), 1);  
> }  
> ```

```rust
pub fn permits_in_use(&self) -> usize {
```

#### `impl<'a> Drop for DynamicSemaphorePermit<'a>`  <sub>(impl)</sub>

```rust
impl<'a> Drop for DynamicSemaphorePermit<'a> {
```

#### `drop`  <sub>(fn)</sub>

> Handles the automatic release of the permit with capacity management.  
>   
> This implementation uses an approach designed to avoid race conditions:  
>   
> We make the decision atomically BEFORE releasing the permit by checking if we're  
> currently over capacity. If we are, we "forget" the permit instead of releasing it.  
> If we're not over capacity, we release it normally.  
>   
> This works because:  
> 1. We decrement permits_in_use first (atomically)  
> 2. We check if permits_in_use + available_permits > capacity  
> 3. If so, we're over capacity and should forget this permit  
> 4. If not, we can safely release it  
>   
> The key insight is that permits_in_use represents permits about to be released,  
> so permits_in_use + available_permits tells us what the total would be after release.

```rust
fn drop(&mut self) {
```

#### `impl<'a> Send for DynamicSemaphorePermit<'a> }`  <sub>(impl)</sub>

```rust
unsafe impl<'a> Send for DynamicSemaphorePermit<'a> {}
    
    #[cfg(test)]
    mod tests {
```

#### `test_basic_acquire_release`  <sub>(fn)</sub>

```rust
async fn test_basic_acquire_release() {
```

#### `test_capacity_reduction`  <sub>(fn)</sub>

```rust
async fn test_capacity_reduction() {
```

#### `test_capacity_increase`  <sub>(fn)</sub>

```rust
async fn test_capacity_increase() {
```

#### `test_try_acquire`  <sub>(fn)</sub>

```rust
async fn test_try_acquire() {
```

#### `test_close`  <sub>(fn)</sub>

```rust
async fn test_close() {
```

#### `test_over_capacity_acquisition_prevention`  <sub>(fn)</sub>

> Test that reproduces the exact live site issue that was discovered

```rust
async fn test_over_capacity_acquisition_prevention() {
```

#### `test_concurrent_capacity_reduction`  <sub>(fn)</sub>

> Test concurrent operations under load to verify race condition fixes

```rust
async fn test_concurrent_capacity_reduction() {
```

#### `test_stress_concurrent_operations`  <sub>(fn)</sub>

> Stress test with continuous capacity changes and concurrent acquisitions

```rust
async fn test_stress_concurrent_operations() {
```

#### `test_feroxbuster_integration_scenario`  <sub>(fn)</sub>

> Test that demonstrates integration scenarios similar to feroxbuster usage

```rust
async fn test_feroxbuster_integration_scenario() {
```

#### `test_edge_cases`  <sub>(fn)</sub>

> Test edge cases and boundary conditions

```rust
async fn test_edge_cases() {
```



## Module `client` <a id='module-client'></a>

*11 documented items*


### `src/client.rs`


#### `ClientConfig`  <sub>(struct)</sub>

> Configuration struct for initializing a reqwest client

```rust
pub struct ClientConfig<'a, I>
where
    I: IntoIterator,
    I::Item: AsRef<Path> + std::fmt::Debug,
    {
```

**Fields:**
- `pub timeout: u64,` — The timeout for requests in seconds
- `pub user_agent: &'a str,` — The User-Agent string to use for requests
- `pub redirects: bool,` — Whether to follow redirects
- `pub insecure: bool,` — Whether to allow insecure connections
- `pub headers: &'a HashMap<String, String>,` — Headers to include in requests
- `pub proxy: Option<&'a str>,` — Proxy server to use for requests
- `pub server_certs: Option<I>,` — Server certificates to use for requests
- `pub client_cert: Option<&'a str>,` — Client certificate to use for requests
- `pub client_key: Option<&'a str>,` — Client key to use for requests
- `pub scope: &'a [Url],` — scope for redirect handling


#### `create_redirect_policy`  <sub>(fn)</sub>

> Create a redirect policy based on the provided config

```rust
fn create_redirect_policy<I>(config: &ClientConfig<'_, I>) -> Policy
where
    I: IntoIterator,
    I::Item: AsRef<Path> + std::fmt::Debug,
    {
```

#### `initialize`  <sub>(fn)</sub>

> Create and return an instance of [reqwest::Client](https://docs.rs/reqwest/latest/reqwest/struct.Client.html)  
> with optional scope-aware redirect handling

```rust
pub fn initialize<I>(config: ClientConfig<'_, I>) -> Result<Client>
where
    I: IntoIterator,
    I::Item: AsRef<Path> + std::fmt::Debug,
    {
```

#### `client_with_bad_proxy`  <sub>(fn)</sub>

> create client with a bad proxy, expect panic

```rust
fn client_with_bad_proxy() {
```

#### `client_with_good_proxy`  <sub>(fn)</sub>

> create client with a proxy, expect no error

```rust
fn client_with_good_proxy() {
```

#### `client_with_valid_server_pem`  <sub>(fn)</sub>

> create client with a server cert in pem format, expect no error

```rust
fn client_with_valid_server_pem() {
```

#### `client_with_valid_server_der`  <sub>(fn)</sub>

> create client with a server cert in der format, expect no error

```rust
fn client_with_valid_server_der() {
```

#### `client_with_valid_server_pem_and_der`  <sub>(fn)</sub>

> create client with two server certs (pem and der), expect no error

```rust
fn client_with_valid_server_pem_and_der() {
```

#### `client_with_invalid_server_cert`  <sub>(fn)</sub>

> create client with invalid certificate, expect panic

```rust
fn client_with_invalid_server_cert() {
```

#### `initialize_with_scope_creates_client`  <sub>(fn)</sub>

> test that scope-aware client can be created with valid parameters

```rust
fn initialize_with_scope_creates_client() {
```

#### `initialize_with_scope_empty_scope`  <sub>(fn)</sub>

> test that scope-aware client works without scope (should use default behavior)

```rust
fn initialize_with_scope_empty_scope() {
```



## Module `heuristics` <a id='module-heuristics'></a>

*38 documented items*


### `src/heuristics.rs`


#### `ref`  <sub>(static)</sub>

> Pre-built HashSet of file extensions for O(1) lookup in directory listing detection  
> Combines COMMON_FILE_EXTENSIONS and DEFAULT_BACKUP_EXTENSIONS

```rust
static ref FILE_EXTENSION_SET: HashSet<&'static str> = {
```

#### `DirListingType`  <sub>(enum)</sub>

> enum representing the different servers that `parse_html` can detect when directory listing is  
> enabled

```rust
pub enum DirListingType {
```

**Variants:**
- `Apache,` — apache server, detected by `Index of /`
- `TomCatOrPython,` — tomcat/python server, detected by `Directory Listing for /`
- `AspDotNet,` — ASP.NET server, detected by `Directory Listing -- /`
- `Custom,` — custom/non-standard directory listing, detected by high-signal heuristics
- `None,` — variant that represents the absence of directory listing


#### `DirListingResult`  <sub>(struct)</sub>

> Wrapper around the results of running a directory listing detection against a target web page

```rust
pub struct DirListingResult {
```

**Fields:**
- `pub dir_list_type: Option<DirListingType>,` — i.e. https://portswigger.net/kb/issues/00600100_directory-listing
- `pub response: FeroxResponse,` — the `FeroxResponse` generated during detection


#### `WildcardResult`  <sub>(enum)</sub>

> wrapper around the results of running a wildcard detection against a target web page

```rust
pub enum WildcardResult {
```

**Variants:**
- `WildcardDirectory(usize),` — variant that represents a wildcard directory
- `FourOhFourLike(usize),` — variant that represents the presence of a 404-like response


#### `HeuristicTests`  <sub>(struct)</sub>

> container for heuristics related info

```rust
pub struct HeuristicTests {
```

**Fields:**
- `handles: Arc<Handles>,` — Handles object for event handler interaction


#### `impl HeuristicTests`  <sub>(impl)</sub>

> HeuristicTests implementation

```rust
impl HeuristicTests {
```

#### `new`  <sub>(fn)</sub>

> create a new HeuristicTests struct

```rust
pub fn new(handles: Arc<Handles>) -> Self {
```

#### `unique_string`  <sub>(fn)</sub>

> Simple helper to return a uuid, formatted as lowercase without hyphens  
>   
> `length` determines the number of uuids to string together. Each uuid  
> is 32 characters long. So, a length of 1 return a 32 character string,  
> a length of 2 returns a 64 character string, and so on...

```rust
fn unique_string(&self, length: usize) -> String {
```

#### `connectivity`  <sub>(fn)</sub>

> Simply tries to connect to all given sites before starting to scan  
>   
> In the event that no sites can be reached, the program will exit.  
>   
> Any urls that are found to be alive are returned to the caller.

```rust
pub async fn connectivity(&self, target_urls: &[String]) -> Result<Vec<String>> {
```

#### `directory_listing`  <sub>(fn)</sub>

> heuristic designed to detect when a server has directory listing enabled

```rust
pub async fn directory_listing(&self, target_url: &str) -> Result<Option<DirListingResult>> {
```

#### `detect_directory_listing`  <sub>(fn)</sub>

> Directory listing heuristic detection, uses <title> tag to make its determination. When  
> the inner html of <title> matches one of the following, a `DirListingType` is returned.  
> - apache: `Index of /`  
> - tomcat/python: `Directory Listing for /`  
> - ASP.NET: `Directory Listing -- /`  
> - <host> - /: iis, azure, skipping due to loose heuristic  
> - custom: detected by combining multiple high-signal heuristics

```rust
fn detect_directory_listing(&self, html: &Html) -> Option<DirListingType> {
```

#### `has_parent_directory_link`  <sub>(fn)</sub>

> check if the HTML contains a link to the parent directory  
>   
> returns true if any anchor element has:  
> - href equals "../" or ".."  
> - visible text contains "parent directory", "to parent", or "up to parent"

```rust
fn has_parent_directory_link(&self, html: &Html) -> bool {
```

#### `has_directory_table_headers`  <sub>(fn)</sub>

> check if the HTML contains table headers typical of directory listings  
>   
> returns true if at least two of the following header categories are present:  
> - name headers: "file name", "filename", "name"  
> - size headers: "size", "file size"  
> - time headers: "date", "last modified", "modified", "last mod"

```rust
fn has_directory_table_headers(&self, html: &Html) -> bool {
```

#### `has_sorting_query_params`  <sub>(fn)</sub>

> check if the HTML contains sorting query parameters typical of auto-index pages  
>   
> returns true if any anchor href contains sorting parameters like:  
> - ?C=N (name), ?C=S (size), ?C=M (modified), ?C=D (date)  
> - optionally combined with &O=A or &O=D (ascending/descending)

```rust
fn has_sorting_query_params(&self, html: &Html) -> bool {
```

#### `has_high_link_density`  <sub>(fn)</sub>

> check if the HTML has a high density of file/directory links  
>   
> returns true if there are at least 3 links that look like files or directories:  
> - href ends with '/' (likely subdirectory)  
> - href looks like a file (common extensions)

```rust
fn has_high_link_density(&self, html: &Html) -> bool {
```

#### `MIN_LINKS`  <sub>(const)</sub>

```rust
const MIN_LINKS: usize = 3;
```

#### `detect_404_like_responses`  <sub>(fn)</sub>

> given a target's base url, attempt to automatically detect its 404 response  
> pattern(s), and then set filters that will exclude those patterns from future  
> responses

```rust
pub async fn detect_404_like_responses(
    &self,
    target_url: &str,
) -> Result<Option<WildcardResult>> {
```

#### `examine_404_like_responses`  <sub>(fn)</sub>

> for all responses, group them by status code, then examine chars/words/lines.  
> if all responses' respective lengths within a status code grouping match  
> each other, we can assume that will remain true for subsequent non-existent urls  
>   
> within a status code grouping, values are examined from most to  
> least specific (content length, word count, line count)

```rust
fn examine_404_like_responses<'a>(
    &self,
    responses: &'a [FeroxResponse],
) -> Option<(Vec<Box<WildcardFilter>>, Vec<&'a FeroxResponse>)> {
```

#### `heuristics_unique_string_returns_correct_length`  <sub>(fn)</sub>

> request a unique string of 32bytes * a value returns correct result

```rust
fn heuristics_unique_string_returns_correct_length() {
```

#### `detect_directory_listing_finds_tomcat_python`  <sub>(fn)</sub>

> `detect_directory_listing` correctly identifies tomcat/python instances

```rust
fn detect_directory_listing_finds_tomcat_python() {
```

#### `detect_directory_listing_finds_apache`  <sub>(fn)</sub>

> `detect_directory_listing` correctly identifies apache instances

```rust
fn detect_directory_listing_finds_apache() {
```

#### `detect_directory_listing_finds_asp_dot_net`  <sub>(fn)</sub>

> `detect_directory_listing` correctly identifies ASP.NET instances

```rust
fn detect_directory_listing_finds_asp_dot_net() {
```

#### `detect_directory_listing_returns_none_as_default`  <sub>(fn)</sub>

> `detect_directory_listing` returns None when heuristic doesn't match

```rust
fn detect_directory_listing_returns_none_as_default() {
```

#### `has_parent_directory_link_detects_by_href`  <sub>(fn)</sub>

> `has_parent_directory_link` detects parent directory links by href

```rust
fn has_parent_directory_link_detects_by_href() {
```

#### `has_parent_directory_link_detects_by_text`  <sub>(fn)</sub>

> `has_parent_directory_link` detects parent directory links by text

```rust
fn has_parent_directory_link_detects_by_text() {
```

#### `has_parent_directory_link_returns_false_when_absent`  <sub>(fn)</sub>

> `has_parent_directory_link` returns false when no parent link

```rust
fn has_parent_directory_link_returns_false_when_absent() {
```

#### `has_directory_table_headers_detects_name_and_size`  <sub>(fn)</sub>

> `has_directory_table_headers` detects table headers with name and size

```rust
fn has_directory_table_headers_detects_name_and_size() {
```

#### `has_directory_table_headers_detects_name_and_date`  <sub>(fn)</sub>

> `has_directory_table_headers` detects table headers with name and date

```rust
fn has_directory_table_headers_detects_name_and_date() {
```

#### `has_directory_table_headers_requires_two_categories`  <sub>(fn)</sub>

> `has_directory_table_headers` returns false with only one category

```rust
fn has_directory_table_headers_requires_two_categories() {
```

#### `has_sorting_query_params_detects_apache_style`  <sub>(fn)</sub>

> `has_sorting_query_params` detects Apache-style sorting parameters

```rust
fn has_sorting_query_params_detects_apache_style() {
```

#### `has_sorting_query_params_returns_false_when_absent`  <sub>(fn)</sub>

> `has_sorting_query_params` returns false when no sorting params

```rust
fn has_sorting_query_params_returns_false_when_absent() {
```

#### `has_high_link_density_detects_files_and_dirs`  <sub>(fn)</sub>

> `has_high_link_density` detects high density of file/directory links

```rust
fn has_high_link_density_detects_files_and_dirs() {
```

#### `has_high_link_density_requires_minimum_links`  <sub>(fn)</sub>

> `has_high_link_density` requires at least 3 links

```rust
fn has_high_link_density_requires_minimum_links() {
```

#### `has_high_link_density_ignores_parent_links`  <sub>(fn)</sub>

> `has_high_link_density` ignores parent directory links

```rust
fn has_high_link_density_ignores_parent_links() {
```

#### `detect_directory_listing_detects_custom_with_multiple_signals`  <sub>(fn)</sub>

> `detect_directory_listing` detects custom listing with 2+ signals

```rust
fn detect_directory_listing_detects_custom_with_multiple_signals() {
```

#### `detect_directory_listing_requires_two_signals`  <sub>(fn)</sub>

> `detect_directory_listing` requires at least 2 signals for custom detection

```rust
fn detect_directory_listing_requires_two_signals() {
```

#### `detect_directory_listing_detects_rootme_sample`  <sub>(fn)</sub>

> `detect_directory_listing` detects Root-Me sample page as custom

```rust
fn detect_directory_listing_detects_rootme_sample() {
```

#### `detect_directory_listing_ignores_generic_pages`  <sub>(fn)</sub>

> `detect_directory_listing` does not trigger on pages with many random links

```rust
fn detect_directory_listing_ignores_generic_pages() {
```



## Module `lib` <a id='module-lib'></a>

*24 documented items*


### `src/lib.rs`


#### `CommandSender`  <sub>(type)</sub>

> Alias for tokio::sync::mpsc::UnboundedSender<Command>

```rust
pub(crate) type CommandSender = UnboundedSender<Command>;
```

#### `CommandReceiver`  <sub>(type)</sub>

> Alias for tokio::sync::mpsc::UnboundedSender<Command>

```rust
pub(crate) type CommandReceiver = UnboundedReceiver<Command>;
```

#### `Joiner`  <sub>(type)</sub>

> Alias for tokio::task::JoinHandle<anyhow::Result<()>>

```rust
pub(crate) type Joiner = JoinHandle<Result<()>>;
```

#### `FeroxChannel`  <sub>(type)</sub>

> Generic mpsc::unbounded_channel type to tidy up some code

```rust
pub(crate) type FeroxChannel<T> = (UnboundedSender<T>, UnboundedReceiver<T>);
```

#### `ExtractionResult`  <sub>(type)</sub>

> Wrapper around the results of performing any kind of extraction against a target web page

```rust
pub(crate) type ExtractionResult = HashSet<String>;
```

#### `VERSION`  <sub>(const)</sub>

> Version pulled from Cargo.toml at compile time

```rust
pub(crate) const VERSION: &str = env!("CARGO_PKG_VERSION");
```

#### `DEFAULT_OPEN_FILE_LIMIT`  <sub>(const)</sub>

> Maximum number of file descriptors that can be opened during a scan

```rust
pub const DEFAULT_OPEN_FILE_LIMIT: u64 = 8192;
```

#### `DEFAULT_IGNORED_EXTENSIONS`  <sub>(const)</sub>

> Default set of extensions to Ignore when auto-collecting extensions during scans

```rust
pub(crate) const DEFAULT_IGNORED_EXTENSIONS: [&str; 43] = [
    "woff2", "woff", "ttf", "otf", "eot", "tif", "tiff", "ico", "cur", "bmp", "webp", "svg", "png",
    "jpg", "jpeg", "jfif", "gif", "avif", "apng", "pjpeg", "pjp", "mov", "wav", "mpg", "mpeg",
    "mp3", "mp4", "m4a", "m4p", "m4v", "ogg", "webm", "ogv", "oga", "flac", "aac", "3gp", "css",
    "zip", "xls", "xml", "gz", "tgz",
    ];
```

#### `DEFAULT_BACKUP_EXTENSIONS`  <sub>(const)</sub>

> Default set of extensions to search for when auto-collecting backups during scans

```rust
pub(crate) const DEFAULT_BACKUP_EXTENSIONS: [&str; 5] = ["~", ".bak", ".bak2", ".old", ".1"];
```

#### `COMMON_FILE_EXTENSIONS`  <sub>(const)</sub>

> list of common file extensions for link density detection in directory listings  
> based on https://www.computerhope.com/issues/ch001789.htm

```rust
pub(crate) const COMMON_FILE_EXTENSIONS: [&str; 154] = [
    // Web & Documents
    ".html",
    ".htm",
    ".php",
    ".asp",
    ".aspx",
    ".jsp",
    ".jspx",
    ".cgi",
    ".pl",
    ".py",
    ".rb",
    ".lua",
    ".txt",
    ".pdf",
    ".doc",
    ".docx",
    ".xls",
    ".xlsx",
    ".ppt",
    ".pptx",
    ".odt",
    ".ods",
    ".odp",
    ".rtf",
    ".tex",
```

#### `DEFAULT_WORDLIST`  <sub>(const)</sub>

> Default wordlist to use when `-w|--wordlist` isn't specified and not `wordlist` isn't set  
> in a [ferox-config.toml](constant.DEFAULT_CONFIG_NAME.html) config file.  
>   
> defaults to kali's default install location on linux:  
> - `/usr/share/seclists/Discovery/Web-Content/raft-medium-directories.txt`  
>   
> and to the current directory on windows  
> - `.\seclists\Discovery\Web-Content\raft-medium-directories.txt`

```rust
pub const DEFAULT_WORDLIST: &str =
    "/usr/share/seclists/Discovery/Web-Content/raft-medium-directories.txt";
```

#### `DEFAULT_WORDLIST`  <sub>(const)</sub>

```rust
pub const DEFAULT_WORDLIST: &str =
    ".\\SecLists\\Discovery\\Web-Content\\raft-medium-directories.txt";
```

#### `SECONDARY_WORDLIST`  <sub>(const)</sub>

```rust
pub const SECONDARY_WORDLIST: &str =
    "/usr/local/share/seclists/Discovery/Web-Content/raft-medium-directories.txt";
```

#### `SLEEP_DURATION`  <sub>(const)</sub>

> Number of milliseconds to wait between polls of `PAUSE_SCAN` when user pauses a scan

```rust
pub(crate) const SLEEP_DURATION: u64 = 500;
```

#### `HIGH_ERROR_RATIO`  <sub>(const)</sub>

> The percentage of requests as errors it takes to be deemed too high

```rust
pub const HIGH_ERROR_RATIO: f64 = 0.90;
```

#### `DEFAULT_STATUS_CODES`  <sub>(const)</sub>

> Default list of status codes to report (all of them)

```rust
pub const DEFAULT_STATUS_CODES: [StatusCode; 60] = [
    // all 1XX response codes
    StatusCode::CONTINUE,
    StatusCode::SWITCHING_PROTOCOLS,
    StatusCode::PROCESSING,
    // all 2XX response codes
    StatusCode::OK,
    StatusCode::CREATED,
    StatusCode::ACCEPTED,
    StatusCode::NON_AUTHORITATIVE_INFORMATION,
    StatusCode::NO_CONTENT,
    StatusCode::RESET_CONTENT,
    StatusCode::PARTIAL_CONTENT,
    StatusCode::MULTI_STATUS,
    StatusCode::ALREADY_REPORTED,
    StatusCode::IM_USED,
    // all 3XX response codes
    StatusCode::MULTIPLE_CHOICES,
    StatusCode::MOVED_PERMANENTLY,
    StatusCode::FOUND,
    StatusCode::SEE_OTHER,
    StatusCode::NOT_MODIFIED,
    StatusCode::USE_PROXY,
    StatusCode::TEMPORARY_REDIRECT,
    StatusCode::PERMANENT_REDIRECT,
    // all 4XX response codes
    StatusCode::BAD_REQUEST,
```

#### `DEFAULT_METHOD`  <sub>(const)</sub>

> Default method for requests

```rust
pub(crate) const DEFAULT_METHOD: &str = "GET";
```

#### `DEFAULT_CONFIG_NAME`  <sub>(const)</sub>

> Default filename for config file settings  
>   
> Expected location is in the same directory as the feroxbuster binary.

```rust
pub const DEFAULT_CONFIG_NAME: &str = "ferox-config.toml";
```

#### `USER_AGENTS`  <sub>(const)</sub>

> User agents to select from when random agent is being used

```rust
pub const USER_AGENTS: [&str; 12] = [
    "Mozilla/5.0 (Linux; Android 8.0.0; SM-G960F Build/R16NW) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/62.0.3202.84 Mobile Safari/537.36",
    "Mozilla/5.0 (iPhone; CPU iPhone OS 12_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/12.0 Mobile/15E148 Safari/604.1",
    "Mozilla/5.0 (Windows Phone 10.0; Android 6.0.1; Microsoft; RM-1152) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/52.0.2743.116 Mobile Safari/537.36 Edge/15.15254",
    "Mozilla/5.0 (Linux; Android 7.0; Pixel C Build/NRD90M; wv) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/52.0.2743.98 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/42.0.2311.135 Safari/537.36 Edge/12.246",
    "Mozilla/5.0 (X11; CrOS x86_64 8172.45.0) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/51.0.2704.64 Safari/537.36",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_11_2) AppleWebKit/601.3.9 (KHTML, like Gecko) Version/9.0.2 Safari/601.3.9",
    "Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/47.0.2526.111 Safari/537.36",
    "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:15.0) Gecko/20100101 Firefox/15.0.1",
    "Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)",
    "Mozilla/5.0 (compatible; bingbot/2.0; +http://www.bing.com/bingbot.htm)",
    "Mozilla/5.0 (compatible; Yahoo! Slurp; http://help.yahoo.com/help/us/ysearch/slurp)",
    ];
```

#### `NEAR_DUPLICATE_DISTANCE`  <sub>(const)</sub>

> maximum hamming distance allowed between two simhash signatures when detecting near-duplicates  
>   
> ref: https://static.googleusercontent.com/media/research.google.com/en//pubs/archive/33026.pdf  
> section: 4.1 Choice of Parameters

```rust
pub(crate) const NEAR_DUPLICATE_DISTANCE: usize = 3;
```

#### `UNIQUE_DISTANCE`  <sub>(const)</sub>

> maximum hamming distance allowed between two simhash signatures when unique'ifying responses

```rust
pub(crate) const UNIQUE_DISTANCE: usize = 1;
```

#### `default_config_name`  <sub>(fn)</sub>

> asserts default config name is correct

```rust
fn default_config_name() {
```

#### `default_wordlist`  <sub>(fn)</sub>

> asserts default wordlist is correct

```rust
fn default_wordlist() {
```

#### `default_version`  <sub>(fn)</sub>

> asserts default version is correct

```rust
fn default_version() {
```



## Module `logger` <a id='module-logger'></a>

*1 documented items*


### `src/logger.rs`


#### `initialize`  <sub>(fn)</sub>

> Create a customized instance of  
> [env_logger::Logger](https://docs.rs/env_logger/latest/env_logger/struct.Logger.html)  
> with timer offset/color and set the log level based on `verbosity`

```rust
pub fn initialize(config: Arc<Configuration>) -> Result<()> {
```



## Module `macros` <a id='module-macros'></a>

*2 documented items*


### `src/macros.rs`


#### `macro send_command`  <sub>(macro)</sub>

> wrapper to improve code readability

```rust
macro_rules! send_command {
```

#### `macro skip_fail`  <sub>(macro)</sub>

> while looping, check for a Result, if Ok return the value, if Err, continue

```rust
macro_rules! skip_fail {
```



## Module `main` <a id='module-main'></a>

*15 documented items*


### `src/main.rs`


#### `ref`  <sub>(static)</sub>

> Limits the number of parallel scans active at any given time when using --parallel

```rust
static ref PARALLEL_LIMITER: Semaphore = Semaphore::new(0);
```

#### `append_words_from_path`  <sub>(fn)</sub>

> Read a single wordlist file, appending any new (non-comment, non-empty) entries to `words`.  
> Tracks the set of words already added in `seen` so the merged result across multiple source  
> files contains every unique word exactly once.

```rust
fn append_words_from_path(
    path: &str,
    words: &mut Vec<String>,
    seen: &mut std::collections::HashSet<String>,
) -> Result<()> {
```

#### `scan`  <sub>(fn)</sub>

> Determine whether it's a single url scan or urls are coming from stdin, then scan as needed

```rust
async fn scan(targets: Vec<String>, handles: Arc<Handles>) -> Result<()> {
```

#### `get_targets`  <sub>(fn)</sub>

> Get targets from either commandline or stdin, pass them back to the caller as a Result<Vec>

```rust
async fn get_targets(handles: Arc<Handles>) -> Result<Vec<String>> {
```

#### `wrapped_main`  <sub>(fn)</sub>

> async main called from real main, broken out in this way to allow for some synchronous code  
> to be executed before bringing the tokio runtime online

```rust
async fn wrapped_main(config: Arc<Configuration>) -> Result<()> {
```

#### `clean_up`  <sub>(fn)</sub>

> Single cleanup function that handles all the necessary drops/finishes etc required to gracefully  
> shutdown the program

```rust
async fn clean_up(handles: Arc<Handles>, tasks: Tasks) -> Result<()> {
```

#### `build_update_client`  <sub>(fn)</sub>

> build a client for self_update to use in place of its own default, honoring --insecure and  
> --server-certs the same way the scanning client does; returns `None` when neither flag is in  
> play, so self_update keeps using its own default client  
> (see https://github.com/epi052/feroxbuster/issues/1148)

```rust
fn build_update_client(
    insecure: bool,
    server_certs: &[String],
) -> anyhow::Result<Option<self_update::reqwest::blocking::Client>> {
```

#### `update_app`  <sub>(fn)</sub>

```rust
async fn update_app(
    insecure: bool,
    server_certs: Vec<String>,
) -> Result<self_update::VersionStatus, Box<dyn ::std::error::Error>> {
```

#### `main`  <sub>(fn)</sub>

```rust
fn main() -> Result<()> {
```

#### `build_update_client_returns_none_when_secure_and_no_certs`  <sub>(fn)</sub>

> neither --insecure nor --server-certs given -> self_update keeps its own default client

```rust
fn build_update_client_returns_none_when_secure_and_no_certs() {
```

#### `build_update_client_returns_client_when_insecure`  <sub>(fn)</sub>

> --insecure alone is enough to force building an override client

```rust
fn build_update_client_returns_client_when_insecure() {
```

#### `build_update_client_returns_client_for_valid_pem_cert`  <sub>(fn)</sub>

> a valid pem server cert, with --insecure left off, still forces an override client

```rust
fn build_update_client_returns_client_for_valid_pem_cert() {
```

#### `build_update_client_returns_client_for_valid_der_cert`  <sub>(fn)</sub>

> a valid der server cert is also accepted

```rust
fn build_update_client_returns_client_for_valid_der_cert() {
```

#### `build_update_client_errors_on_missing_cert_file`  <sub>(fn)</sub>

> a path that doesn't exist should error out instead of silently continuing

```rust
fn build_update_client_errors_on_missing_cert_file() {
```

#### `build_update_client_accepts_non_certificate_der_until_connection_time`  <sub>(fn)</sub>

> self_update's client is rustls-backed, and rustls' `Certificate::from_pem`/`from_der`  
> only reject bytes that fail to parse as *some* DER structure - they don't check that the  
> structure is actually an X.509 certificate, so a well-formed-but-wrong DER blob (here, a  
> private key) is accepted at build time and would only fail later, during an actual TLS  
> handshake. This documents that boundary rather than asserting an eager rejection that  
> doesn't happen; feroxbuster's own scanning client (native-tls backed, see client.rs)  
> does reject this same file eagerly, so the two code paths differ here.

```rust
fn build_update_client_accepts_non_certificate_der_until_connection_time() {
```



## Module `message` <a id='module-message'></a>

*8 documented items*


### `src/message.rs`


#### `FeroxMessage`  <sub>(struct)</sub>

> Representation of a log entry, can be represented as a human readable string or JSON

```rust
pub struct FeroxMessage {
```

**Fields:**
- `pub(crate) kind: String,` — Name of this type of struct, used for serialization, i.e. `{"type":"log"}`
- `pub(crate) message: String,` — The log message
- `pub(crate) level: String,` — The log level
- `pub(crate) time_offset: f32,` — The number of seconds elapsed since the scan started
- `pub(crate) module: String,` — The module from which log::* was called


#### `impl FeroxSerialize for FeroxMessage`  <sub>(impl)</sub>

> Implementation of FeroxMessage

```rust
impl FeroxSerialize for FeroxMessage {
```

#### `as_str`  <sub>(fn)</sub>

> Create a string representation of the log message  
>   
> ex:  301       10l       16w      173c https://localhost/api

```rust
fn as_str(&self) -> String {
```

#### `as_json`  <sub>(fn)</sub>

> Create an NDJSON representation of the log message  
>   
> (expanded for clarity)  
> ex:  
> {  
> "type": "log",  
> "message": "Sent https://localhost/api to file handler",  
> "level": "DEBUG",  
> "time_offset": 0.86333454,  
> "module": "feroxbuster::reporter"  
> }\n

```rust
fn as_json(&self) -> anyhow::Result<String> {
```

#### `ferox_message_as_str_returns_string_with_newline`  <sub>(fn)</sub>

> test as_str method of FeroxMessage

```rust
fn ferox_message_as_str_returns_string_with_newline() {
```

#### `ferox_message_as_json_returns_json_representation_of_ferox_message_with_newline`  <sub>(fn)</sub>

> test as_json method of FeroxMessage

```rust
fn ferox_message_as_json_returns_json_representation_of_ferox_message_with_newline() {
```

#### `message_defaults`  <sub>(fn)</sub>

> test defaults for coverage

```rust
fn message_defaults() {
```

#### `message_as_str_edges`  <sub>(fn)</sub>

> ensure WILDCARD messages serialize to WLD and anything not known to UNK

```rust
fn message_as_str_edges() {
```



## Module `parser` <a id='module-parser'></a>

*13 documented items*


### `src/parser.rs`


#### `ref`  <sub>(static)</sub>

> Regex used to validate values passed to --time-limit  
>   
> Examples of expected values that will this regex will match:  
> - 30s  
> - 20m  
> - 1h  
> - 1d

```rust
pub static ref TIMESPEC_REGEX: Regex =
    Regex::new(r"^(?i)(?P<n>\d+)(?P<m>[smdh])$").expect("Could not compile regex");
```

#### `ref`  <sub>(static)</sub>

> help string for user agent, your guess is as good as mine as to why this is required...

```rust
static ref DEFAULT_USER_AGENT: String = format!(
    "Sets the User-Agent (default: feroxbuster/{})",
    crate_version!()
);
```

#### `initialize`  <sub>(fn)</sub>

> Create and return an instance of [clap::App](https://docs.rs/clap/latest/clap/struct.App.html), i.e. the Command Line Interface's configuration

```rust
pub fn initialize() -> Command {
```

#### `valid_time_spec`  <sub>(fn)</sub>

> Validate that a string is formatted as a number followed by s, m, h, or d (10d, 30s, etc...)

```rust
fn valid_time_spec(time_spec: &str) -> Result<String, String> {
```

#### `EPILOGUE`  <sub>(const)</sub>

```rust
const EPILOGUE: &str = r#"NOTE:
    Options that take multiple values are very flexible.  Consider the following ways of specifying
    extensions:
    ./feroxbuster -u http://127.1 -x pdf -x js,html -x php txt json,docx
    
    The command above adds .pdf, .js, .html, .php, .txt, .json, and .docx to each url
    
    All of the methods above (multiple flags, space separated, comma separated, etc...) are valid
    and interchangeable.  The same goes for urls, headers, status codes, queries, and size filters.
    
    EXAMPLES:
    Multiple headers:
    ./feroxbuster -u http://127.1 -H Accept:application/json "Authorization: Bearer {token}"
    
    IPv6, non-recursive scan with INFO-level logging enabled:
    ./feroxbuster -u http://[::1] --no-recursion -vv
    
    Read urls from STDIN; pipe only resulting urls out to another tool
    cat targets | ./feroxbuster --stdin --silent -s 200 301 302 --redirects -x js | fff -s 200 -o js-files
    
    Proxy traffic through Burp
    ./feroxbuster -u http://127.1 --burp
    
    Proxy traffic through a SOCKS proxy
    ./feroxbuster -u http://127.1 --proxy socks5://127.0.0.1:9050
    
    Pass auth token via query parameter
```

#### `parser_initialize_gives_defaults`  <sub>(fn)</sub>

> initialize parser, expect a clap::App returned

```rust
fn parser_initialize_gives_defaults() {
```

#### `validate_valid_time_spec_validation`  <sub>(fn)</sub>

> sanity checks that valid_time_spec correctly checks and rejects a given string  
>   
> instead of having a bunch of single tests here, they're all quick and are mostly checking  
> that i didn't hose up the regex.  Going to consolidate them into a single test

```rust
fn validate_valid_time_spec_validation() {
```

#### `update_alone_is_allowed`  <sub>(fn)</sub>

> --update alone should parse fine (baseline exclusivity behavior, unchanged)

```rust
fn update_alone_is_allowed() {
```

#### `update_allows_insecure`  <sub>(fn)</sub>

> --update combined with -k/--insecure should parse fine now; this is the crux of the  
> #1148 fix, so this is a real clap-conflict-table assertion, not just a smoke test

```rust
fn update_allows_insecure() {
```

#### `update_allows_server_certs`  <sub>(fn)</sub>

> --update combined with --server-certs should also parse fine

```rust
fn update_allows_server_certs() {
```

#### `update_allows_insecure_reversed_order`  <sub>(fn)</sub>

> order shouldn't matter for the two allowed flags

```rust
fn update_allows_insecure_reversed_order() {
```

#### `update_still_conflicts_with_url`  <sub>(fn)</sub>

> --update should still conflict with --url, same as every other unrelated flag

```rust
fn update_still_conflicts_with_url() {
```

#### `update_still_conflicts_with_verbosity`  <sub>(fn)</sub>

> --update should still conflict with flags that have nothing to do with the client,  
> confirming the allowlist didn't accidentally widen beyond --insecure/--server-certs

```rust
fn update_still_conflicts_with_verbosity() {
```



## Module `progress` <a id='module-progress'></a>

*6 documented items*


### `src/progress.rs`


#### `ref`  <sub>(static)</sub>

> Global progress bar that houses other progress bars

```rust
pub static ref PROGRESS_BAR: MultiProgress = MultiProgress::with_draw_target(ProgressDrawTarget::stdout());
```

#### `ref`  <sub>(static)</sub>

> Global progress bar that is only used for printing messages that don't jack up other bars

```rust
pub static ref PROGRESS_PRINTER: ProgressBar = add_bar("", 0, BarType::Hidden);
```

#### `BarType`  <sub>(enum)</sub>

> Types of ProgressBars that can be added to `PROGRESS_BAR`

```rust
pub enum BarType {
```

**Variants:**
- `Hidden,` — no template used / not visible
- `Default,` — normal directory status bar (reqs/sec shown)
- `Message,` — similar to `Default`, except `-` is used in place of line/word/char count
- `Total,` — bar used to show overall scan metrics
- `Quiet,` — simpler output bar that shows only the directory being scanned (no updating info)


#### `add_bar`  <sub>(fn)</sub>

> Add an [indicatif::ProgressBar](https://docs.rs/indicatif/latest/indicatif/struct.ProgressBar.html)  
> to the global [PROGRESS_BAR](../config/struct.PROGRESS_BAR.html)

```rust
pub fn add_bar(prefix: &str, length: u64, bar_type: BarType) -> ProgressBar {
```

#### `update_style`  <sub>(fn)</sub>

> Update the style of a progress bar based on the `BarType`

```rust
pub fn update_style(bar: &ProgressBar, bar_type: BarType) {
```

#### `add_bar_with_all_configurations`  <sub>(fn)</sub>

> hit all code branches for add_bar

```rust
fn add_bar_with_all_configurations() {
```



## Module `response` <a id='module-response'></a>

*43 documented items*


### `src/response.rs`


#### `FeroxResponse`  <sub>(struct)</sub>

> A `FeroxResponse`, derived from a `Response` to a submitted `Request`

```rust
pub struct FeroxResponse {
```

**Fields:**
- `url: Url,` — The final `Url` of this `FeroxResponse`
- `original_url: String,` — The original url from which the final `Url` was derived
- `status: StatusCode,` — The `StatusCode` of this `FeroxResponse`
- `method: Method,` — The HTTP Request `Method` of this `FeroxResponse`
- `text: String,` — The full response text
- `content_length: u64,` — The content-length of this response, if known
- `line_count: usize,` — The number of lines contained in the body of this response, if known
- `word_count: usize,` — The number of words contained in the body of this response, if known
- `headers: HeaderMap,` — The `Headers` of this `FeroxResponse`
- `wildcard: bool,` — Wildcard response status
- `pub(crate) output_level: OutputLevel,` — whether the user passed --quiet|--silent on the command line
- `pub(crate) extension: Option<String>,` — Url's file extension, if one exists
- `truncated: bool,` — Whether the response body was truncated due to size limits
- `timestamp: f64,` — Timestamp of when this response was received


#### `impl Default for FeroxResponse`  <sub>(impl)</sub>

> implement Default trait for FeroxResponse

```rust
impl Default for FeroxResponse {
```

#### `default`  <sub>(fn)</sub>

> return a default reqwest::Url and then normal defaults after that

```rust
fn default() -> Self {
```

#### `impl fmt::Display for FeroxResponse`  <sub>(impl)</sub>

> Implement Display for FeroxResponse

```rust
impl fmt::Display for FeroxResponse {
```

#### `fmt`  <sub>(fn)</sub>

> formatter for Display

```rust
fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
```

#### `impl FeroxResponse`  <sub>(impl)</sub>

> `FeroxResponse` implementation

```rust
impl FeroxResponse {
```

#### `status`  <sub>(fn)</sub>

> Get the `StatusCode` of this `FeroxResponse`

```rust
pub fn status(&self) -> &StatusCode {
```

#### `method`  <sub>(fn)</sub>

> Get the `Method` of this `FeroxResponse`

```rust
pub fn method(&self) -> &Method {
```

#### `wildcard`  <sub>(fn)</sub>

> Get the `wildcard` of this `FeroxResponse`

```rust
pub fn wildcard(&self) -> bool {
```

#### `url`  <sub>(fn)</sub>

> Get the final `Url` of this `FeroxResponse`.

```rust
pub fn url(&self) -> &Url {
```

#### `text`  <sub>(fn)</sub>

> Get the full response text

```rust
pub fn text(&self) -> &str {
```

#### `headers`  <sub>(fn)</sub>

> Get the `Headers` of this `FeroxResponse`

```rust
pub fn headers(&self) -> &HeaderMap {
```

#### `content_length`  <sub>(fn)</sub>

> Get the content-length of this response, if known

```rust
pub fn content_length(&self) -> u64 {
```

#### `timestamp`  <sub>(fn)</sub>

> Get the timestamp of this response

```rust
pub fn timestamp(&self) -> f64 {
```

#### `truncated`  <sub>(fn)</sub>

> Get whether this response was truncated due to size limits

```rust
pub fn truncated(&self) -> bool {
```

#### `set_url`  <sub>(fn)</sub>

> Set `FeroxResponse`'s `url` attribute, has no affect if an error occurs

```rust
pub fn set_url(&mut self, url: &str) {
```

#### `set_wildcard`  <sub>(fn)</sub>

> set `wildcard` attribute

```rust
pub fn set_wildcard(&mut self, is_wildcard: bool) {
```

#### `set_text`  <sub>(fn)</sub>

> set `text` attribute; update words/lines/content_length

```rust
pub fn set_text(&mut self, text: &str) {
```

#### `drop_text`  <sub>(fn)</sub>

> free the `text` data, reducing memory usage

```rust
pub fn drop_text(&mut self) {
```

#### `line_count`  <sub>(fn)</sub>

> Returns line count of the response text.

```rust
pub fn line_count(&self) -> usize {
```

#### `word_count`  <sub>(fn)</sub>

> Returns word count of the response text.

```rust
pub fn word_count(&self) -> usize {
```

#### `from`  <sub>(fn)</sub>

> Create a new `FeroxResponse` from the given `Response`

```rust
pub async fn from(
    mut response: Response,
    original_url: &str,
    method: &str,
    output_level: OutputLevel,
    max_size_read: usize,
) -> Self {
```

#### `parse_extension`  <sub>(fn)</sub>

> if --collect-extensions is used, examine the response's url and grab the file's extension  
> if one is available to be grabbed. If an extension is found, send it to the ScanHandler  
> for further processing

```rust
pub(crate) fn parse_extension(&mut self, handles: Arc<Handles>) -> Result<()> {
```

#### `reached_max_depth`  <sub>(fn)</sub>

> Helper function that determines if the configured maximum recursion depth has been reached  
>   
> Essentially looks at the Url path and determines how many directories are present in the  
> given Url

```rust
pub(crate) fn reached_max_depth(
    &self,
    base_depth: usize,
    max_depth: usize,
    handles: Arc<Handles>,
) -> bool {
```

#### `is_directory`  <sub>(fn)</sub>

> Helper function to determine suitability for recursion  
>   
> handles 2xx and 3xx responses by either checking if the url ends with a / (2xx)  
> or if the Location header is present and matches the base url + / (3xx)

```rust
pub fn is_directory(&self) -> bool {
```

#### `send_report`  <sub>(fn)</sub>

> Simple helper to send a `FeroxResponse` over the tx side of an `mpsc::unbounded_channel`

```rust
pub fn send_report(self, report_sender: CommandSender) -> Result<()> {
```

#### `impl FeroxSerialize for FeroxResponse`  <sub>(impl)</sub>

> Implement FeroxSerialize for FeroxResponse

```rust
impl FeroxSerialize for FeroxResponse {
```

#### `as_str`  <sub>(fn)</sub>

> Simple wrapper around create_report_string

```rust
fn as_str(&self) -> String {
```

#### `as_json`  <sub>(fn)</sub>

> Create an NDJSON representation of the FeroxResponse  
>   
> (expanded for clarity)  
> ex:  
> {  
> "type":"response",  
> "url":"https://localhost.com/images",  
> "path":"/images",  
> "status":301,  
> "content_length":179,  
> "line_count":10,  
> "word_count":16,  
> "headers":{  
> "x-content-type-options":"nosniff",  
> "strict-transport-security":"max-age=31536000; includeSubDomains",  
> "x-frame-options":"SAMEORIGIN",  
> "connection":"keep-alive",  
> "server":"nginx/1.16.1",  
> "content-type":"text/html; charset=UTF-8",  
> "referrer-policy":"origin-when-cross-origin",  
> "content-security-policy":"default-src 'none'",  
> "access-control-allow-headers":"X-Requested-With",  
> "x-xss-protection":"1; mode=block",  
> "content-length":"179",  
> "date":"Mon, 23 Nov 2020 15:33:24 GMT",  
> "location":"/images/",  
> "access-control-allow-origin":"https://localhost.com"  
> }  
> }\n

```rust
fn as_json(&self) -> anyhow::Result<String> {
```

#### `impl Serialize for FeroxResponse`  <sub>(impl)</sub>

> Serialize implementation for FeroxResponse

```rust
impl Serialize for FeroxResponse {
```

#### `serialize`  <sub>(fn)</sub>

> Function that handles serialization of a FeroxResponse to NDJSON

```rust
fn serialize<S>(&self, serializer: S) -> anyhow::Result<S::Ok, S::Error>
where
    S: Serializer,
    {
```

#### `impl<'de> Deserialize<'de> for FeroxResponse`  <sub>(impl)</sub>

> Deserialize implementation for FeroxResponse

```rust
impl<'de> Deserialize<'de> for FeroxResponse {
```

#### `deserialize`  <sub>(fn)</sub>

> Deserialize a FeroxResponse from a serde_json::Value

```rust
fn deserialize<D>(deserializer: D) -> anyhow::Result<Self, D::Error>
where
    D: Deserializer<'de>,
    {
```

#### `reached_max_depth_returns_early_on_zero`  <sub>(fn)</sub>

> call reached_max_depth with max depth of zero, which is infinite recursion, expect false

```rust
fn reached_max_depth_returns_early_on_zero() {
```

#### `reached_max_depth_current_depth_equals_max`  <sub>(fn)</sub>

> call reached_max_depth with url depth equal to max depth, expect true

```rust
fn reached_max_depth_current_depth_equals_max() {
```

#### `reached_max_depth_current_depth_less_than_max`  <sub>(fn)</sub>

> call reached_max_depth with url dpeth less than max depth, expect false

```rust
fn reached_max_depth_current_depth_less_than_max() {
```

#### `reached_max_depth_base_depth_equals_max_depth`  <sub>(fn)</sub>

> call reached_max_depth with url of 2, base depth of 2, and max depth of 2, expect false

```rust
fn reached_max_depth_base_depth_equals_max_depth() {
```

#### `reached_max_depth_current_greater_than_max`  <sub>(fn)</sub>

> call reached_max_depth with url depth greater than max depth, expect true

```rust
fn reached_max_depth_current_greater_than_max() {
```

#### `parse_extension_finds_simple_extension`  <sub>(fn)</sub>

> simple case of a single extension gets parsed correctly and stored on the `FeroxResponse`

```rust
fn parse_extension_finds_simple_extension() {
```

#### `parse_extension_ignores_hidden_files`  <sub>(fn)</sub>

> hidden files shouldn't be parsed as extensions, i.e. `/.bash_history`

```rust
fn parse_extension_ignores_hidden_files() {
```

#### `parse_extension_early_returns_based_on_config`  <sub>(fn)</sub>

> `parse_extension` should return immediately if `--collect-extensions` isn't used

```rust
fn parse_extension_early_returns_based_on_config() {
```

#### `truncated_getter_returns_correct_value`  <sub>(fn)</sub>

> test that the truncated getter returns the correct value

```rust
fn truncated_getter_returns_correct_value() {
```

#### `truncated_response_shows_in_url_display`  <sub>(fn)</sub>

> test that truncated responses show [TRUNCATED] in URL display

```rust
fn truncated_response_shows_in_url_display() {
```



## Module `traits` <a id='module-traits'></a>

*11 documented items*


### `src/traits.rs`


#### `FeroxFilter`  <sub>(trait)</sub>

> FeroxFilter trait; represents different types of possible filters that can be applied to  
> responses

```rust
pub trait FeroxFilter: Debug + Send + Sync {
```

#### `should_filter_response`  <sub>(fn)</sub>

> Determine whether or not this particular filter should be applied or not

```rust
fn should_filter_response(&self, response: &FeroxResponse) -> bool;
```

#### `box_eq`  <sub>(fn)</sub>

> delegates to the FeroxFilter-implementing type which gives us the actual type of self

```rust
fn box_eq(&self, other: &dyn Any) -> bool;
```

#### `as_any`  <sub>(fn)</sub>

> gives us `other` as Any in box_eq

```rust
fn as_any(&self) -> &dyn Any;
```

#### `impl Display for dyn FeroxFilter`  <sub>(impl)</sub>

```rust
impl Display for dyn FeroxFilter {
```

#### `fmt`  <sub>(fn)</sub>

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
```

#### `impl PartialEq for Box<dyn FeroxFilter>`  <sub>(impl)</sub>

> implementation of PartialEq, necessary long-form due to "trait cannot be made into an object"  
> error when attempting to derive PartialEq on the trait itself

```rust
impl PartialEq for Box<dyn FeroxFilter> {
```

#### `eq`  <sub>(fn)</sub>

> Perform a comparison of two implementors of the FeroxFilter trait

```rust
fn eq(&self, other: &Box<dyn FeroxFilter>) -> bool {
```

#### `FeroxSerialize`  <sub>(trait)</sub>

> FeroxSerialize trait; represents different types that are Serialize and also implement  
> as_str / as_json methods

```rust
pub trait FeroxSerialize: Serialize {
```

#### `as_str`  <sub>(fn)</sub>

> Return a String representation of the object, generally the human readable version of the  
> implementor

```rust
fn as_str(&self) -> String;
```

#### `as_json`  <sub>(fn)</sub>

> Return an NDJSON representation of the object

```rust
fn as_json(&self) -> Result<String>;
```



## Module `url` <a id='module-url'></a>

*53 documented items*


### `src/url.rs`


#### `UrlExt`  <sub>(trait)</sub>

> Trait extension for reqwest::Url to add scope checking functionality

```rust
pub trait UrlExt {
```

#### `is_in_scope`  <sub>(fn)</sub>

> Check if this URL is allowed based on scope configuration  
>   
> A URL is considered in-scope if:  
> 1. It belongs to the same domain as an in-scope url, OR  
> 2. It belongs to a subdomain of an in-scope url  
>   
> note: the scope list passed in is populated from either --url or --stdin  
> as well as --scope. This means we don't have to worry about checking  
> against the original target url, as that is already in the scope list

```rust
fn is_in_scope(&self, scope: &[Url]) -> bool;
```

#### `is_subdomain_of`  <sub>(fn)</sub>

> Check if this URL is a subdomain of the given parent domain

```rust
fn is_subdomain_of(&self, parent_url: &Url) -> bool;
```

#### `impl UrlExt for Url`  <sub>(impl)</sub>

```rust
impl UrlExt for Url {
```

#### `is_in_scope`  <sub>(fn)</sub>

```rust
fn is_in_scope(&self, scope: &[Url]) -> bool {
```

#### `is_subdomain_of`  <sub>(fn)</sub>

```rust
fn is_subdomain_of(&self, parent_url: &Url) -> bool {
```

#### `FeroxUrl`  <sub>(struct)</sub>

> abstraction around target urls; collects all Url related shenanigans in one place

```rust
pub struct FeroxUrl {
```

**Fields:**
- `pub target: String,` — string representation of the target url
- `handles: Arc<Handles>,` — Handles object for grabbing config values


#### `impl FeroxUrl`  <sub>(impl)</sub>

> implementation of FeroxUrl

```rust
impl FeroxUrl {
```

#### `from_string`  <sub>(fn)</sub>

> Create new FeroxUrl given a target url as a string

```rust
pub fn from_string(target: &str, handles: Arc<Handles>) -> Self {
```

#### `from_url`  <sub>(fn)</sub>

> Create new FeroxUrl given a target url as a reqwest::Url

```rust
pub fn from_url(target: &Url, handles: Arc<Handles>) -> Self {
```

#### `formatted_urls`  <sub>(fn)</sub>

> Creates a vector of formatted Urls  
>   
> At least one value will be returned (base_url + word)  
>   
> If any extensions were passed to the program, each extension will add a  
> (base_url + word + ext) Url to the vector

```rust
pub fn formatted_urls(
    &self,
    word: &str,
    collected_extensions: HashSet<String>,
) -> Result<Vec<Url>> {
```

#### `format`  <sub>(fn)</sub>

> Simple helper to generate a `Url`  
>   
> Errors during parsing `url` or joining `word` are propagated up the call stack

```rust
pub fn format(&self, word: &str, extension: Option<&str>) -> Result<Url> {
```

#### `normalize`  <sub>(fn)</sub>

> Simple helper to abstract away adding a forward-slash to a url if not present  
>   
> used mostly for deduplication purposes and url state tracking

```rust
pub fn normalize(&self) -> String {
```

#### `depth`  <sub>(fn)</sub>

> Helper function that determines the current depth of a given url  
>   
> Essentially looks at the Url path and determines how many directories are present in the  
> given Url  
>   
> http://localhost -> 1  
> http://localhost/ -> 1  
> http://localhost/stuff -> 2  
> ...  
>   
> returns 0 on error and relative urls

```rust
pub fn depth(&self) -> Result<usize> {
```

#### `impl fmt::Display for FeroxUrl`  <sub>(impl)</sub>

> Display implementation for a FeroxUrl

```rust
impl fmt::Display for FeroxUrl {
```

#### `fmt`  <sub>(fn)</sub>

> formatter for FeroxUrl

```rust
fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
```

#### `formatted_urls_no_extension_returns_base_url_with_word`  <sub>(fn)</sub>

> sending url + word without any extensions should get back one url with the joined word

```rust
fn formatted_urls_no_extension_returns_base_url_with_word() {
```

#### `formatted_urls_one_extension_returns_two_urls`  <sub>(fn)</sub>

> sending url + word + 1 extension should get back two urls, one base and one with extension

```rust
fn formatted_urls_one_extension_returns_two_urls() {
```

#### `formatted_urls_multiple_extensions_returns_n_plus_one_urls`  <sub>(fn)</sub>

> sending url + word + multiple extensions should get back n+1 urls

```rust
fn formatted_urls_multiple_extensions_returns_n_plus_one_urls() {
```

#### `depth_base_url_returns_1`  <sub>(fn)</sub>

> base url returns 1

```rust
fn depth_base_url_returns_1() {
```

#### `depth_base_url_with_slash_returns_1`  <sub>(fn)</sub>

> base url with slash returns 1

```rust
fn depth_base_url_with_slash_returns_1() {
```

#### `depth_one_dir_returns_2`  <sub>(fn)</sub>

> base url + 1 dir returns 2

```rust
fn depth_one_dir_returns_2() {
```

#### `depth_one_dir_with_slash_returns_2`  <sub>(fn)</sub>

> base url + 1 dir and slash returns 2

```rust
fn depth_one_dir_with_slash_returns_2() {
```

#### `format_url_normal`  <sub>(fn)</sub>

> base url + 1 word + no slash + no extension

```rust
fn format_url_normal() {
```

#### `format_url_no_word`  <sub>(fn)</sub>

> base url + no word + no slash + no extension

```rust
fn format_url_no_word() {
```

#### `format_url_joins_queries`  <sub>(fn)</sub>

> base url + word + no slash + no extension + queries

```rust
fn format_url_joins_queries() {
```

#### `format_url_without_word_joins_queries`  <sub>(fn)</sub>

> base url + no word + no slash + no extension + queries

```rust
fn format_url_without_word_joins_queries() {
```

#### `format_url_no_url`  <sub>(fn)</sub>

> no base url is an error

```rust
fn format_url_no_url() {
```

#### `format_url_word_with_preslash`  <sub>(fn)</sub>

> word prepended with slash is adjusted for correctness

```rust
fn format_url_word_with_preslash() {
```

#### `format_url_word_with_postslash`  <sub>(fn)</sub>

> word with appended slash allows the slash to persist

```rust
fn format_url_word_with_postslash() {
```

#### `format_url_word_with_two_prepended_slashes`  <sub>(fn)</sub>

> word with two prepended slashes doesn't discard the entire domain

```rust
fn format_url_word_with_two_prepended_slashes() {
```

#### `format_url_word_with_two_prepended_slashes_and_extensions`  <sub>(fn)</sub>

> word with two prepended slashes and extensions doesn't discard the entire domain

```rust
fn format_url_word_with_two_prepended_slashes_and_extensions() {
```

#### `format_url_word_that_is_a_url`  <sub>(fn)</sub>

> word that is a fully formed url, should return an error

```rust
fn format_url_word_that_is_a_url() {
```

#### `formatted_urls_with_postslash_and_extensions`  <sub>(fn)</sub>

> sending url + word with both an extension and add-slash should get back  
> two urls, one with '/' appended to the word, and the other with the extension  
> appended

```rust
fn formatted_urls_with_postslash_and_extensions() {
```

#### `test_is_in_scope`  <sub>(fn)</sub>

> test is_in_scope function to ensure that it checks for presence within scope list

```rust
fn test_is_in_scope() {
```

#### `test_is_in_scope_subdomain`  <sub>(fn)</sub>

> test is_in_scope function to ensure that it checks that a subdomain of a domain within  
> the scope list returns true

```rust
fn test_is_in_scope_subdomain() {
```

#### `test_is_in_scope_not_in_scope`  <sub>(fn)</sub>

> test is_in_scope returns false when url is not in scope

```rust
fn test_is_in_scope_not_in_scope() {
```

#### `test_is_in_scope_empty_scope`  <sub>(fn)</sub>

> test is_in_scope with empty scope returns false

```rust
fn test_is_in_scope_empty_scope() {
```

#### `test_is_in_scope_domain_only_scope`  <sub>(fn)</sub>

> test is_in_scope with domain-only scope entry (not a URL)

```rust
fn test_is_in_scope_domain_only_scope() {
```

#### `test_is_in_scope_subdomain_domain_only_scope`  <sub>(fn)</sub>

> test is_in_scope with subdomain and domain-only scope entry

```rust
fn test_is_in_scope_subdomain_domain_only_scope() {
```

#### `test_is_in_scope_no_domain`  <sub>(fn)</sub>

> test is_in_scope with URL that has no domain

```rust
fn test_is_in_scope_no_domain() {
```

#### `test_is_subdomain_of_true`  <sub>(fn)</sub>

> test is_subdomain_of basic functionality

```rust
fn test_is_subdomain_of_true() {
```

#### `test_is_subdomain_of_same_domain`  <sub>(fn)</sub>

> test is_subdomain_of returns false for same domain

```rust
fn test_is_subdomain_of_same_domain() {
```

#### `test_is_subdomain_of_different_domain`  <sub>(fn)</sub>

> test is_subdomain_of returns false for different domain

```rust
fn test_is_subdomain_of_different_domain() {
```

#### `test_is_subdomain_of_multi_level`  <sub>(fn)</sub>

> test is_subdomain_of with multi-level subdomain

```rust
fn test_is_subdomain_of_multi_level() {
```

#### `test_is_subdomain_of_no_domain`  <sub>(fn)</sub>

> test is_subdomain_of with URLs that have no domain

```rust
fn test_is_subdomain_of_no_domain() {
```

#### `test_is_subdomain_of_parent_no_domain`  <sub>(fn)</sub>

> test is_subdomain_of where parent has no domain

```rust
fn test_is_subdomain_of_parent_no_domain() {
```

#### `test_is_not_in_empty_scope`  <sub>(fn)</sub>

> test is_in_scope with same domain/host

```rust
fn test_is_not_in_empty_scope() {
```

#### `test_is_in_scope_subdomain_with_empty_scope`  <sub>(fn)</sub>

> test is_in_scope with subdomain

```rust
fn test_is_in_scope_subdomain_with_empty_scope() {
```

#### `test_is_in_scope_scope_match`  <sub>(fn)</sub>

> test is_in_scope with scope match

```rust
fn test_is_in_scope_scope_match() {
```

#### `test_is_in_scope_not_allowed`  <sub>(fn)</sub>

> test is_in_scope returns false when not in scope

```rust
fn test_is_in_scope_not_allowed() {
```

#### `test_is_in_scope_empty_scope_different_domain`  <sub>(fn)</sub>

> test is_in_scope with empty scope and different domain

```rust
fn test_is_in_scope_empty_scope_different_domain() {
```

#### `test_is_in_scope_subdomain_in_scope`  <sub>(fn)</sub>

> test is_in_scope with subdomain in scope

```rust
fn test_is_in_scope_subdomain_in_scope() {
```



## Module `utils` <a id='module-utils'></a>

*43 documented items*


### `src/utils.rs`


#### `mut`  <sub>(static)</sub>

> simple counter for grabbing 'random' user agents

```rust
static mut USER_AGENT_CTR: usize = 0;
```

#### `is_certificate_error`  <sub>(fn)</sub>

> detects certificate-related errors by analyzing the error chain

```rust
fn is_certificate_error(error: &reqwest::Error) -> bool {
```

#### `open_file`  <sub>(fn)</sub>

> Given the path to a file, open the file in append mode (create it if it doesn't exist) and  
> return a reference to the buffered file

```rust
pub fn open_file(filename: &str) -> Result<BufWriter<fs::File>> {
```

#### `status_colorizer`  <sub>(fn)</sub>

> Takes in a string and examines the first character to return a color version of the same string

```rust
pub fn status_colorizer(status: &str) -> String {
```

#### `fmt_err`  <sub>(fn)</sub>

> simple wrapper to stay DRY

```rust
pub fn fmt_err(msg: &str) -> String {
```

#### `timestamp`  <sub>(fn)</sub>

> simple wrapper to get the current system time as  
> time elapsed from unix epoch

```rust
pub fn timestamp() -> f64 {
```

#### `send_try_recursion_command`  <sub>(fn)</sub>

> given a FeroxResponse, send a TryRecursion command  
>   
> moved to utils to allow for calls from extractor and scanner

```rust
pub(crate) async fn send_try_recursion_command(
    handles: Arc<Handles>,
    response: FeroxResponse,
) -> Result<()> {
```

#### `module_colorizer`  <sub>(fn)</sub>

> Takes in a string and colors it using console::style  
>   
> mainly putting this here in case i want to change the color later, making any changes easy

```rust
pub fn module_colorizer(modname: &str) -> String {
```

#### `ferox_print`  <sub>(fn)</sub>

> Simple helper to abstract away the check for an attached terminal.  
>   
> If a terminal is attached, progress bars are visible and the progress bar is used to print  
> to stderr. The progress bar must be used when bars are visible in order to not jack up any  
> progress bar output (the bar knows how to print above itself)  
>   
> If a terminal is not attached, `msg` is printed to stdout, with its ansi  
> color codes stripped.  
>   
> additionally, provides a location for future printing options (no color, etc) to be handled

```rust
pub fn ferox_print(msg: &str, bar: &ProgressBar) {
```

#### `logged_request`  <sub>(fn)</sub>

> wrapper for make_request used to pass error/response codes to FeroxScans for per-scan stats  
> tracking of information related to auto-tune/bail

```rust
pub async fn logged_request(
    url: &Url,
    method: &str,
    data: Option<&[u8]>,
    handles: Arc<Handles>,
) -> Result<Response> {
```

#### `make_request`  <sub>(fn)</sub>

> Initiate request to the given `Url` using `Client`

```rust
pub async fn make_request(
    client: &Client,
    url: &Url,
    method: &str,
    mut data: Option<&[u8]>,
    output_level: OutputLevel,
    config: &Configuration,
    tx_stats: UnboundedSender<Command>,
) -> Result<Response> {
```

#### `create_report_string`  <sub>(fn)</sub>

> Helper to create the standard line for output to file/terminal  
>   
> example output:  
> 200      127l      283w     4134c http://localhost/faq

```rust
pub fn create_report_string(
    status: &str,
    method: &str,
    line_count: &str,
    word_count: &str,
    content_length: &str,
    url: &str,
    output_level: OutputLevel,
) -> String {
```

#### `set_open_file_limit`  <sub>(fn)</sub>

> Attempts to set the soft limit for the RLIMIT_NOFILE resource  
>   
> RLIMIT_NOFILE is the maximum number of file descriptors that can be opened by this process  
>   
> The soft limit is the value that the kernel enforces for the corresponding resource.  
> The hard limit acts as a ceiling for the soft limit: an unprivileged process may set only its  
> soft limit to a value in the range from 0 up to the hard limit, and (irreversibly) lower its  
> hard limit.  
>   
> A child process created via fork(2) inherits its parent's resource limits. Resource limits are  
> per-process attributes that are shared by all of the threads in a process.  
>   
> Based on the above information, no attempt is made to restore the limit to its pre-scan value  
> as the adjustment made here is only valid for the scan itself (and any child processes, of which  
> there are none).

```rust
pub fn set_open_file_limit(limit: u64) -> bool {
```

#### `write_to`  <sub>(fn)</sub>

> Given a string and a reference to a locked buffered file, write the contents and flush  
> the buffer to disk.

```rust
pub fn write_to<T>(
    value: &T,
    file: &mut io::BufWriter<fs::File>,
    convert_to_json: bool,
) -> Result<()>
where
    T: FeroxSerialize,
    {
```

#### `should_deny_absolute`  <sub>(fn)</sub>

> determine if a url should be denied based on the given absolute url

```rust
fn should_deny_absolute(url_to_test: &Url, denier: &Url, handles: Arc<Handles>) -> Result<bool> {
```

#### `should_deny_regex`  <sub>(fn)</sub>

> determine if a url should be denied based on the given regular expression  
>   
> the regex ONLY matches against the PATH of the url (not the scheme, host, port, etc)

```rust
fn should_deny_regex(url_to_test: &Url, denier: &Regex) -> bool {
```

#### `should_deny_url`  <sub>(fn)</sub>

> determines whether or not a given url should be denied based on the user-supplied --dont-scan  
> flag

```rust
pub fn should_deny_url(url: &Url, handles: Arc<Handles>) -> Result<bool> {
```

#### `slugify_filename`  <sub>(fn)</sub>

> given a url and filename-suffix, return a unique filename comprised of the slugified url,  
> current unix timestamp and suffix  
>   
> ex: ferox-http_telsa_com-1606947491.state

```rust
pub fn slugify_filename(url: &str, prefix: &str, suffix: &str) -> String {
```

#### `parse_url_with_raw_path`  <sub>(fn)</sub>

> This function takes a url string and returns a `url::Url`  
>   
> It is primarily used to detect url paths that `url::Url::parse` will  
> silently transform, such as /path/../file.html -> /file.html  
>   
> # Warning  
>   
> In the instance of a url with encoded path traversal strings, such as  
> /path/%2e%2e/file.html, the underlying `url::Url::parse` will  
> further encode the %-signs and return /path/%252e%252e/file.html

```rust
pub fn parse_url_with_raw_path(url: &str) -> Result<Url> {
```

#### `utils_parse_url_with_raw_path_javascript`  <sub>(fn)</sub>

> parse_url_with_raw_path with javascript:// should not throw an unimplemented! error

```rust
fn utils_parse_url_with_raw_path_javascript() {
```

#### `utils_parse_url_with_raw_path`  <sub>(fn)</sub>

> multiple tests for parse_url_with_raw_path

```rust
fn utils_parse_url_with_raw_path() {
```

#### `utils_set_open_file_limit_with_low_requested_limit`  <sub>(fn)</sub>

> set_open_file_limit with a low requested limit succeeds

```rust
fn utils_set_open_file_limit_with_low_requested_limit() {
```

#### `utils_set_open_file_limit_with_high_requested_limit`  <sub>(fn)</sub>

> set_open_file_limit with a high requested limit succeeds

```rust
fn utils_set_open_file_limit_with_high_requested_limit() {
```

#### `utils_set_open_file_limit_with_fails_when_both_limits_are_equal`  <sub>(fn)</sub>

> set_open_file_limit should fail when hard == soft

```rust
fn utils_set_open_file_limit_with_fails_when_both_limits_are_equal() {
```

#### `status_colorizer_uses_red_for_500s`  <sub>(fn)</sub>

> status colorizer uses red for 500s

```rust
fn status_colorizer_uses_red_for_500s() {
```

#### `status_colorizer_uses_red_for_400s`  <sub>(fn)</sub>

> status colorizer uses red for 400s

```rust
fn status_colorizer_uses_red_for_400s() {
```

#### `status_colorizer_uses_red_for_errors`  <sub>(fn)</sub>

> status colorizer uses red for errors

```rust
fn status_colorizer_uses_red_for_errors() {
```

#### `status_colorizer_uses_cyan_for_wildcards`  <sub>(fn)</sub>

> status colorizer uses cyan for wildcards

```rust
fn status_colorizer_uses_cyan_for_wildcards() {
```

#### `status_colorizer_uses_blue_for_100s`  <sub>(fn)</sub>

> status colorizer uses blue for 100s

```rust
fn status_colorizer_uses_blue_for_100s() {
```

#### `status_colorizer_uses_green_for_200s`  <sub>(fn)</sub>

> status colorizer uses green for 200s

```rust
fn status_colorizer_uses_green_for_200s() {
```

#### `status_colorizer_uses_yellow_for_300s`  <sub>(fn)</sub>

> status colorizer uses yellow for 300s

```rust
fn status_colorizer_uses_yellow_for_300s() {
```

#### `status_colorizer_returns_as_is`  <sub>(fn)</sub>

> status colorizer doesnt color anything else

```rust
fn status_colorizer_returns_as_is() {
```

#### `should_deny_url_blocks_when_denier_is_exact_match`  <sub>(fn)</sub>

> provide a url that should be blocked where the denier is an exact match for the tested url  
> expect true

```rust
fn should_deny_url_blocks_when_denier_is_exact_match() {
```

#### `should_deny_url_doesnt_compare_mismatched_domains`  <sub>(fn)</sub>

> provide a url that has a different host than the denier but the same path, expect false

```rust
fn should_deny_url_doesnt_compare_mismatched_domains() {
```

#### `should_deny_url_doesnt_compare_non_domains`  <sub>(fn)</sub>

> provide a denier from which we can't check a host, which results in no comparison, expect false

```rust
fn should_deny_url_doesnt_compare_non_domains() {
```

#### `should_deny_url_doesnt_compare_mismatched_domains_in_scanned`  <sub>(fn)</sub>

> provide a url that has a different host than the denier but the same path, expect false  
> because the denier is a parent to the tested, even tho the scanned doesn't compare, it  
> still returns true

```rust
fn should_deny_url_doesnt_compare_mismatched_domains_in_scanned() {
```

#### `should_deny_url_doesnt_compare_non_domains_in_scanned`  <sub>(fn)</sub>

> provide a denier from which we can't check a host, which results in no comparison, expect false  
> because the denier is a parent to the tested, even tho the scanned doesn't compare, it  
> still returns true  
>   
> note: adding parse_url_with_raw_path changed the behavior of this test, it used to return  
> true, now it returns false. see my note in should_deny_absolute and the unreachable!  
> call block to see why  
>   
> leaving this test here to document the behavior change and to catch regressions in the  
> new expected behavior

```rust
fn should_deny_url_doesnt_compare_non_domains_in_scanned() {
```

#### `should_deny_url_blocks_child`  <sub>(fn)</sub>

> provide a denier where the tested url is a sub-path and the scanned url is not, expect true

```rust
fn should_deny_url_blocks_child() {
```

#### `should_deny_url_doesnt_block_non_child`  <sub>(fn)</sub>

> provide a denier where the tested url is not a sub-path and the scanned url is not, expect false

```rust
fn should_deny_url_doesnt_block_non_child() {
```

#### `should_deny_url_blocks_child_when_scan_url_isnt_parent`  <sub>(fn)</sub>

> provide a denier where the tested url is a sub-path and the scanned url is not, expect true

```rust
fn should_deny_url_blocks_child_when_scan_url_isnt_parent() {
```

#### `should_deny_url_doesnt_block_child_when_scan_url_is_parent`  <sub>(fn)</sub>

> provide a denier where the tested url is not a sub-path and the scanned url is not, expect false

```rust
fn should_deny_url_doesnt_block_child_when_scan_url_is_parent() {
```

#### `should_deny_url_blocks_urls_based_on_regex_in_path`  <sub>(fn)</sub>

> provide a denier where the tested url is matched against a regular expression in the path  
> of the url

```rust
fn should_deny_url_blocks_urls_based_on_regex_in_path() {
```

#### `should_deny_url_blocks_urls_based_on_regex_in_scheme`  <sub>(fn)</sub>

> provide a denier where the tested url is matched against a regular expression in the scheme  
> of the url

```rust
fn should_deny_url_blocks_urls_based_on_regex_in_scheme() {
```

