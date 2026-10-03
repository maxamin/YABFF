//! The seam between feroxml and the real `feroxbuster` binary.
//!
//! [`FeroxRunner`] is the injectable boundary: production uses [`RealRunner`]
//! (spawns feroxbuster, reads its NDJSON output file); tests inject a fake that
//! replays a captured fixture, so the whole pipeline is exercised offline.

use std::collections::HashMap;
use std::io::Write;
use std::process::Command;

use serde::Deserialize;

use crate::config::Config;
use crate::tokenize::path_segments;

/// One `type == "response"` record from feroxbuster's `--json` output.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct FeroxResponse {
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub path: String,
    #[serde(default)]
    pub status: u16,
    #[serde(default)]
    pub content_length: u64,
    #[serde(default)]
    pub word_count: u64,
    #[serde(default)]
    pub line_count: u64,
    #[serde(default)]
    pub headers: HashMap<String, String>,
}

impl FeroxResponse {
    /// Case-insensitive header lookup.
    pub fn header(&self, name: &str) -> Option<&str> {
        let want = name.to_lowercase();
        self.headers
            .iter()
            .find(|(k, _)| k.to_lowercase() == want)
            .map(|(_, v)| v.as_str())
    }

    /// Mirror of feroxbuster's own directory test: a 2xx ending in `/`, or a
    /// 3xx whose `Location` points at `url + "/"`.
    pub fn is_directory(&self) -> bool {
        if (200..300).contains(&self.status) {
            return self.url.ends_with('/');
        }
        if (300..400).contains(&self.status) {
            if let Some(loc) = self.header("location") {
                return loc == format!("{}/", self.url.trim_end_matches('/'))
                    || loc.ends_with('/');
            }
        }
        false
    }

    /// Depth = number of path segments in the URL.
    pub fn depth(&self) -> usize {
        path_segments(&self.url).len()
    }
}

/// Parse a single NDJSON line, returning `Some` only for response records.
pub fn parse_response_line(line: &str) -> Option<FeroxResponse> {
    let line = line.trim();
    if line.is_empty() {
        return None;
    }
    let value: serde_json::Value = serde_json::from_str(line).ok()?;
    if value.get("type").and_then(|t| t.as_str()) != Some("response") {
        return None;
    }
    serde_json::from_value(value).ok()
}

/// Parse a whole NDJSON blob (many lines) into response records.
pub fn parse_responses(blob: &str) -> Vec<FeroxResponse> {
    blob.lines().filter_map(parse_response_line).collect()
}

/// A single bounded feroxbuster scan request.
#[derive(Debug, Clone)]
pub struct FeroxArgs {
    pub url: String,
    pub words: Vec<String>,
    pub no_recursion: bool,
    /// When true, this is the fingerprint/probe call: do not restrict reported
    /// status codes with `-s` (the probe needs to see 500s/401s so fingerprinting
    /// isn't blind to APIs that error on their base path, e.g. Juice Shop's
    /// `/rest` → 500), and pass `--dont-filter` so feroxbuster's wildcard
    /// auto-filter doesn't drop a catch-all's uniform redirects and the
    /// discriminator headers (e.g. `JSESSIONID`) that ride on them.
    pub all_codes: bool,
    /// Per-call override to let feroxbuster extract links (used by learn mode to
    /// harvest maximum path structure). Scans leave this false so discoveries
    /// stay attributable to the ML engine unless `ferox_extract_links` is set.
    pub extract_links: bool,
}

/// The injectable boundary to feroxbuster.
pub trait FeroxRunner {
    fn run(&self, args: &FeroxArgs) -> anyhow::Result<Vec<FeroxResponse>>;
}

/// Drives the real `feroxbuster` binary.
pub struct RealRunner {
    pub cfg: Config,
}

impl RealRunner {
    pub fn new(cfg: Config) -> Self {
        Self { cfg }
    }

    /// Build the full feroxbuster argument list (everything after the binary
    /// name) for one scan. Pure and deterministic given `(cfg, args, wl, out)`, so
    /// the flag logic is unit-testable without launching the subprocess.
    fn cli_args(
        &self,
        args: &FeroxArgs,
        wl: &std::path::Path,
        out: &std::path::Path,
    ) -> Vec<String> {
        let mut a: Vec<String> = vec![
            "-u".into(), args.url.clone(),
            "-w".into(), wl.to_string_lossy().into_owned(),
            "--json".into(),
            "-o".into(), out.to_string_lossy().into_owned(),
            "-q".into(),
        ];
        if args.no_recursion {
            a.push("-n".into());
        }
        // discoveries should come from feroxml's engine, not feroxbuster's crawler
        // (learn mode overrides per-call to harvest maximum structure)
        if !self.cfg.ferox_extract_links && !args.extract_links {
            a.push("--dont-extract-links".into());
        }
        // The fingerprint/probe call (all_codes) must see the RAW responses:
        // feroxbuster's wildcard auto-filter otherwise drops a catch-all's uniform
        // redirects, and with them the discriminator headers that ride on those
        // redirects (e.g. an auth-walled Spring app that 302s every path to /login
        // with a JSESSIONID cookie). feroxml does its own soft-404 handling on the
        // probe (learn_soft_404 / apply_soft_404), so keeping the raw stream here
        // surfaces signal without polluting results. Normal scans keep the filter.
        if args.all_codes {
            a.push("--dont-filter".into());
        }
        a.push("-t".into());
        a.push(self.cfg.threads.to_string());
        if self.cfg.rate_limit > 0 {
            a.push("--rate-limit".into());
            a.push(self.cfg.rate_limit.to_string());
        }
        if !self.cfg.tls_verify {
            a.push("-k".into());
        }
        if !self.cfg.scan_time_limit.is_empty() {
            a.push("--time-limit".into());
            a.push(self.cfg.scan_time_limit.clone());
        }
        for ext in &self.cfg.extensions {
            a.push("-x".into());
            a.push(ext.clone());
        }
        if !args.all_codes && !self.cfg.success_codes.is_empty() {
            a.push("-s".into());
            for c in &self.cfg.success_codes {
                a.push(c.to_string());
            }
        }
        a
    }

    fn write_wordlist(&self, words: &[String]) -> anyhow::Result<std::path::PathBuf> {
        std::fs::create_dir_all(&self.cfg.state_dir)?;
        let unique = format!(
            "wl-{}-{}.txt",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or(0)
        );
        let path = std::path::Path::new(&self.cfg.state_dir).join(unique);
        let mut f = std::fs::File::create(&path)?;
        for w in words {
            writeln!(f, "{w}")?;
        }
        Ok(path)
    }
}

impl FeroxRunner for RealRunner {
    fn run(&self, args: &FeroxArgs) -> anyhow::Result<Vec<FeroxResponse>> {
        if args.words.is_empty() {
            return Ok(vec![]);
        }
        let wl = self.write_wordlist(&args.words)?;
        let out = std::path::Path::new(&self.cfg.state_dir).join(format!(
            "out-{}-{}.json",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or(0)
        ));

        let mut cmd = Command::new(&self.cfg.ferox_binary);
        for a in self.cli_args(args, &wl, &out) {
            cmd.arg(a);
        }

        let status = cmd.status();
        let result = match status {
            Ok(_) => {
                let blob = std::fs::read_to_string(&out).unwrap_or_default();
                Ok(parse_responses(&blob))
            }
            Err(e) => Err(anyhow::anyhow!(
                "failed to launch feroxbuster ('{}'): {e}",
                self.cfg.ferox_binary
            )),
        };
        // best-effort cleanup
        let _ = std::fs::remove_file(&wl);
        let _ = std::fs::remove_file(&out);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE: &str = r#"{"type":"configuration","wordlist":"x"}
{"type":"response","url":"https://x.test/api/","path":"/api/","status":301,"content_length":0,"word_count":0,"line_count":0,"headers":{"location":"https://x.test/api/"}}
{"type":"response","url":"https://x.test/api/v1","path":"/api/v1","status":200,"content_length":51,"word_count":4,"line_count":1,"headers":{"content-type":"application/json"}}
{"type":"statistics","requests":100}
garbage line that is not json
"#;

    #[test]
    fn parses_only_response_records() {
        let r = parse_responses(FIXTURE);
        assert_eq!(r.len(), 2);
        assert_eq!(r[0].status, 301);
        assert_eq!(r[1].url, "https://x.test/api/v1");
        assert_eq!(r[1].header("content-type"), Some("application/json"));
    }

    #[test]
    fn directory_detection() {
        let r = parse_responses(FIXTURE);
        assert!(r[0].is_directory(), "301 with trailing-slash location is a dir");
        assert!(!r[1].is_directory());
    }

    fn args_for(all_codes: bool) -> Vec<String> {
        let mut cfg = Config::default();
        cfg.success_codes = vec![200, 301];
        let runner = RealRunner::new(cfg);
        let a = FeroxArgs {
            url: "https://x.test/".into(),
            words: vec!["a".into()],
            no_recursion: true,
            all_codes,
            extract_links: false,
        };
        runner.cli_args(&a, std::path::Path::new("/tmp/wl"), std::path::Path::new("/tmp/out"))
    }

    #[test]
    fn probe_passes_dont_filter_and_no_status_restriction() {
        let probe = args_for(true);
        assert!(probe.iter().any(|x| x == "--dont-filter"), "probe must disable the wildcard filter: {probe:?}");
        assert!(!probe.iter().any(|x| x == "-s"), "probe must not restrict status codes: {probe:?}");
    }

    #[test]
    fn normal_scan_keeps_filter_and_restricts_status() {
        let scan = args_for(false);
        assert!(!scan.iter().any(|x| x == "--dont-filter"), "scans keep feroxbuster's wildcard filter: {scan:?}");
        assert!(scan.iter().any(|x| x == "-s"), "scans restrict to success codes: {scan:?}");
    }
}
