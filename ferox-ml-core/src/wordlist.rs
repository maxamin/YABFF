//! Per-round wordlist construction and cross-round de-duplication.

use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::io::{BufRead, Write};

use crate::tokenize::path_segments;

/// Turn ranked `(token, score)` predictions into a de-duplicated word list,
/// capped at `max` entries. Order is preserved (already ranked).
pub fn build_round_wordlist(preds: &[(String, f64)], max: usize) -> Vec<String> {
    let mut seen = HashSet::new();
    let mut out = Vec::new();
    for (tok, _) in preds {
        let t = tok.trim().trim_matches('/').to_string();
        if t.is_empty() || !seen.insert(t.clone()) {
            continue;
        }
        out.push(t);
        if out.len() >= max {
            break;
        }
    }
    out
}

/// Append up to `extra` unique entries from a base `seed` list onto `primary`
/// (the Markov predictions), skipping anything already present. Keeps prediction
/// first so the ML ordering is preserved; the seed list only fills spare budget.
pub fn merge_seed(primary: &[String], seed: &[String], extra: usize) -> Vec<String> {
    let mut out = primary.to_vec();
    let mut seen: HashSet<String> = primary
        .iter()
        .map(|s| s.trim().trim_matches('/').to_string())
        .collect();
    let mut added = 0;
    for s in seed {
        if added >= extra {
            break;
        }
        let t = s.trim().trim_matches('/').to_string();
        if t.is_empty() || t.starts_with('#') || !seen.insert(t.clone()) {
            continue;
        }
        out.push(t);
        added += 1;
    }
    out
}

/// Read a wordlist file into non-empty, non-comment entries (bounded to `cap`).
pub fn load_wordlist(path: &str, cap: usize) -> std::io::Result<Vec<String>> {
    let text = std::fs::read_to_string(path)?;
    Ok(text
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .take(cap)
        .map(|l| l.to_string())
        .collect())
}

/// Load every file in `dir` **recursively** (walking the whole directory tree,
/// e.g. all of SecLists' `Discovery/Web-Content`) and merge them into one
/// de-duplicated pool **ranked by signal**: entries are ordered by *document
/// frequency* (how many of the files contain them) descending, then shorter first,
/// then lexicographically — so universally-common tokens (`admin`, `api`,
/// `index.php`, `images`, …) come first and one-off esoteric entries (a specific
/// vendor path, a fuzzing header name) come last. Lines are trimmed; empty lines
/// and `#` comments are skipped. `cap` keeps the highest-signal entries:
/// **`cap == 0` means unlimited** (every entry in the tree); otherwise the top
/// `cap` by signal. Files that can't be read (e.g. binary) are skipped rather than
/// aborting the whole load. The result is deterministic.
pub fn load_list_dir(dir: &str, cap: usize) -> std::io::Result<Vec<String>> {
    Ok(capped(rank_entries(&collect_files(dir)), cap))
}

/// Recursively collect every file under `dir`, in sorted full-path order.
fn collect_files(dir: &str) -> Vec<std::path::PathBuf> {
    let mut files: Vec<std::path::PathBuf> = Vec::new();
    let mut stack = vec![std::path::PathBuf::from(dir)];
    while let Some(d) = stack.pop() {
        let Ok(rd) = std::fs::read_dir(&d) else {
            continue; // unreadable directory: skip
        };
        for p in rd.filter_map(|e| e.ok().map(|e| e.path())) {
            if p.is_dir() {
                stack.push(p);
            } else if p.is_file() {
                files.push(p);
            }
        }
    }
    files.sort();
    files
}

/// Rank every entry across `files` by document frequency (files containing it)
/// descending, then shorter, then lexicographic. Returns the full ranked pool.
fn rank_entries(files: &[std::path::PathBuf]) -> Vec<String> {
    let mut df: HashMap<String, u32> = HashMap::new();
    for f in files {
        let Ok(text) = std::fs::read_to_string(f) else {
            continue; // skip unreadable/binary files
        };
        let mut in_file: HashSet<String> = HashSet::new();
        for line in text.lines() {
            let w = line.trim();
            if w.is_empty() || w.starts_with('#') {
                continue;
            }
            if in_file.insert(w.to_string()) {
                *df.entry(w.to_string()).or_insert(0) += 1;
            }
        }
    }
    let mut scored: Vec<(u32, String)> = df.into_iter().map(|(w, n)| (n, w)).collect();
    scored.sort_by(|(na, a), (nb, b)| {
        nb.cmp(na)
            .then_with(|| a.len().cmp(&b.len()))
            .then_with(|| a.as_str().cmp(b.as_str()))
    });
    scored.into_iter().map(|(_, w)| w).collect()
}

fn capped(mut pool: Vec<String>, cap: usize) -> Vec<String> {
    if cap != 0 && pool.len() > cap {
        pool.truncate(cap); // keep the top `cap` highest-signal entries
    }
    pool
}

/// Signature of the directory tree: file count plus each file's path, length and
/// modified-time. Changes whenever a file is added, removed, resized or rewritten,
/// so a stale cache is never used.
fn tree_signature(files: &[std::path::PathBuf]) -> u64 {
    let mut h = std::collections::hash_map::DefaultHasher::new();
    files.len().hash(&mut h);
    for f in files {
        f.to_string_lossy().as_bytes().hash(&mut h);
        if let Ok(m) = std::fs::metadata(f) {
            m.len().hash(&mut h);
            if let Some(d) = m
                .modified()
                .ok()
                .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            {
                d.as_secs().hash(&mut h);
                d.subsec_nanos().hash(&mut h);
            }
        }
    }
    h.finish()
}

/// Cache file path for `dir` inside `cache_dir` (keyed by the absolute dir path).
fn cache_file(cache_dir: &str, dir: &str) -> std::path::PathBuf {
    let mut h = std::collections::hash_map::DefaultHasher::new();
    let abs = std::fs::canonicalize(dir).unwrap_or_else(|_| std::path::PathBuf::from(dir));
    abs.to_string_lossy().as_bytes().hash(&mut h);
    std::path::Path::new(cache_dir).join(format!("feroxml-listpool-{:016x}.cache", h.finish()))
}

/// Load the ranked pool with an on-disk cache in `cache_dir`, keyed by `dir` and a
/// [`tree_signature`] of its files. On a cache **hit** the ranked pool is read back
/// directly — skipping the recursive walk and document-frequency ranking (the
/// expensive part for a multi-million-entry tree like full SecLists). On a **miss**
/// (no cache, or the tree changed) it ranks fresh and writes the cache (best-effort;
/// a read-only `cache_dir` simply means no speed-up). An **empty `cache_dir` disables
/// caching** entirely (rank fresh, write nothing). The cache file's first line is the
/// signature; the rest are the ranked entries. Returns `(pool, from_cache)`.
pub fn load_list_dir_cached(
    dir: &str,
    cap: usize,
    cache_dir: &str,
) -> std::io::Result<(Vec<String>, bool)> {
    let files = collect_files(dir);
    // an empty cache_dir disables caching (rank fresh every time, write nothing)
    if cache_dir.is_empty() {
        return Ok((capped(rank_entries(&files), cap), false));
    }
    let sig = tree_signature(&files);
    let cache = cache_file(cache_dir, dir);

    if let Ok(f) = std::fs::File::open(&cache) {
        let mut rd = std::io::BufReader::new(f);
        let mut first = String::new();
        if rd.read_line(&mut first).is_ok() && first.trim_end() == sig.to_string() {
            let pool: Vec<String> = rd.lines().map_while(Result::ok).collect();
            return Ok((capped(pool, cap), true));
        }
    }

    let full = rank_entries(&files);
    // best-effort cache write (signature line, then one entry per line)
    if let Some(parent) = cache.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if let Ok(f) = std::fs::File::create(&cache) {
        let mut w = std::io::BufWriter::new(f);
        if writeln!(w, "{sig}").is_ok() {
            for e in &full {
                if writeln!(w, "{e}").is_err() {
                    break;
                }
            }
        }
        let _ = w.flush();
    }
    Ok((capped(full, cap), false))
}

/// A merged list pool that is **re-applied to every directory**: each arm (a
/// discovered directory) gets its own forward cursor over the whole pool, so the
/// same word is tried under every directory — like feroxbuster's recursion — rather
/// than being consumed once globally. Each arm's cursor stops at the end of the pool
/// (no wrap-around).
pub struct ListPool {
    pool: Vec<String>,
    /// per-arm walk position into `pool`
    pos: HashMap<String, usize>,
}

impl ListPool {
    pub fn new(pool: Vec<String>) -> Self {
        Self {
            pool,
            pos: HashMap::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.pool.is_empty()
    }

    pub fn len(&self) -> usize {
        self.pool.len()
    }

    /// Up to `n` entries for `arm`, in pool order, skipping anything in `skip`
    /// (this arm's already-scheduled words), advancing the arm's own cursor past
    /// every entry it inspects. A given pool position is served at most once *per
    /// arm*, so the wordlist is re-applied independently to each directory.
    pub fn next_chunk(&mut self, arm: &str, skip: &HashSet<String>, n: usize) -> Vec<String> {
        let mut out = Vec::with_capacity(n.min(16));
        let pos = self.pos.entry(arm.to_string()).or_insert(0);
        while *pos < self.pool.len() && out.len() < n {
            let w = &self.pool[*pos];
            *pos += 1;
            if !skip.contains(w) {
                out.push(w.clone());
            }
        }
        out
    }

    /// Whether `arm` has walked the whole pool.
    pub fn arm_exhausted(&self, arm: &str) -> bool {
        self.pos.get(arm).copied().unwrap_or(0) >= self.pool.len()
    }

    /// Whether any of `arms` still has pool entries left to serve (a never-seen arm
    /// counts as having the whole pool ahead of it).
    pub fn any_remaining(&self, arms: &[String]) -> bool {
        !self.pool.is_empty() && arms.iter().any(|a| !self.arm_exhausted(a))
    }
}

/// Tracks every URL already discovered/scheduled so nothing is scanned twice.
#[derive(Default)]
pub struct SeenPaths {
    seen: HashSet<String>,
}

impl SeenPaths {
    pub fn new() -> Self {
        Self::default()
    }

    /// Canonical key for a URL: host-independent slash-joined path.
    fn key(url: &str) -> String {
        path_segments(url).join("/")
    }

    /// Record a URL; returns `true` if it was newly inserted.
    pub fn insert(&mut self, url: &str) -> bool {
        self.seen.insert(Self::key(url))
    }

    pub fn contains(&self, url: &str) -> bool {
        self.seen.contains(&Self::key(url))
    }

    pub fn len(&self) -> usize {
        self.seen.len()
    }

    pub fn is_empty(&self) -> bool {
        self.seen.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dedups_and_caps() {
        let preds = vec![
            ("users".to_string(), 0.4),
            ("users".to_string(), 0.3), // dup
            ("/auth/".to_string(), 0.2),
            ("health".to_string(), 0.1),
        ];
        let wl = build_round_wordlist(&preds, 2);
        assert_eq!(wl, vec!["users".to_string(), "auth".to_string()]);
    }

    #[test]
    fn merge_seed_appends_unique_capped() {
        let primary = vec!["api".to_string(), "v1".to_string()];
        let seed = vec![
            "api".to_string(),      // dup, skipped
            "admin".to_string(),
            "#comment".to_string(), // comment, skipped
            "login".to_string(),
            "config".to_string(),
        ];
        let merged = merge_seed(&primary, &seed, 2);
        assert_eq!(merged, ["api", "v1", "admin", "login"]);
    }

    #[test]
    fn load_list_dir_ranks_by_signal_dedups_and_caps() {
        let dir = std::env::temp_dir().join(format!("ferox-lists-{}", std::process::id()));
        let _ = std::fs::create_dir_all(&dir);
        // "api" appears in BOTH files (document frequency 2) -> highest signal.
        // the rest have df 1 -> ordered shorter-first, then lexicographically.
        std::fs::write(dir.join("a.txt"), "admin\n# comment\nlogin\n\napi\n").unwrap();
        std::fs::write(dir.join("b.txt"), "api\nusers\n").unwrap();
        let pool = load_list_dir(dir.to_str().unwrap(), 100).unwrap();
        // api (df 2) first; then df-1 by len asc then lex: admin,login,users (all len 5)
        assert_eq!(pool, ["api", "admin", "login", "users"]);
        // cap keeps the HIGHEST-signal entries
        let capped = load_list_dir(dir.to_str().unwrap(), 2).unwrap();
        assert_eq!(capped, ["api", "admin"]);
        // cap == 0 means unlimited (every entry)
        let all = load_list_dir(dir.to_str().unwrap(), 0).unwrap();
        assert_eq!(all, ["api", "admin", "login", "users"]);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn load_list_dir_cached_hits_and_invalidates() {
        let base = std::env::temp_dir().join(format!("ferox-cache-{}", std::process::id()));
        let dir = base.join("lists");
        let cache = base.join("cache");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::create_dir_all(&cache).unwrap();
        std::fs::write(dir.join("a.txt"), "admin\napi\n").unwrap();
        let (d, c) = (dir.to_str().unwrap(), cache.to_str().unwrap());

        // first load: cache MISS, ranks fresh and writes the cache
        let (p1, hit1) = load_list_dir_cached(d, 0, c).unwrap();
        assert!(!hit1, "first load must be a miss");
        // second load (unchanged tree): cache HIT, identical result
        let (p2, hit2) = load_list_dir_cached(d, 0, c).unwrap();
        assert!(hit2, "second load must hit the cache");
        assert_eq!(p1, p2);

        // changing the tree invalidates the cache (new file -> new signature)
        std::fs::write(dir.join("b.txt"), "newtoken\n").unwrap();
        let (p3, hit3) = load_list_dir_cached(d, 0, c).unwrap();
        assert!(!hit3, "a changed tree must miss");
        assert!(p3.contains(&"newtoken".to_string()));
        // and the refreshed cache hits again
        let (_p4, hit4) = load_list_dir_cached(d, 0, c).unwrap();
        assert!(hit4);

        // cap is applied after the cache read
        let (capped, hit5) = load_list_dir_cached(d, 1, c).unwrap();
        assert!(hit5);
        assert_eq!(capped.len(), 1);

        let _ = std::fs::remove_dir_all(&base);
    }

    #[test]
    fn load_list_dir_floats_common_tokens_above_esoteric_ones() {
        // mimic SecLists: one generic token in many files, plus per-file junk
        let dir = std::env::temp_dir().join(format!("ferox-sig-{}", std::process::id()));
        let _ = std::fs::create_dir_all(&dir);
        std::fs::write(dir.join("a.txt"), "admin\nzzz-vendor-specific-aaa\n").unwrap();
        std::fs::write(dir.join("b.txt"), "admin\nzzz-vendor-specific-bbb\n").unwrap();
        std::fs::write(dir.join("c.txt"), "admin\nzzz-vendor-specific-ccc\n").unwrap();
        let pool = load_list_dir(dir.to_str().unwrap(), 0).unwrap();
        assert_eq!(pool[0], "admin", "common token must rank first: {pool:?}");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn load_list_dir_walks_subdirectories() {
        // a SecLists-style tree: files nested in subdirectories must all be read
        let dir = std::env::temp_dir().join(format!("ferox-tree-{}", std::process::id()));
        let sub = dir.join("CMS");
        let deep = sub.join("inner");
        std::fs::create_dir_all(&deep).unwrap();
        std::fs::write(dir.join("top.txt"), "admin\n").unwrap();
        std::fs::write(sub.join("wp.txt"), "wp-login.php\n").unwrap();
        std::fs::write(deep.join("more.txt"), "xmlrpc.php\nadmin\n").unwrap(); // "admin" dedups
        let pool = load_list_dir(dir.to_str().unwrap(), 0).unwrap();
        // all three files contributed; sorted by full path: CMS/inner/more, CMS/wp, top
        assert!(pool.contains(&"admin".to_string()));
        assert!(pool.contains(&"wp-login.php".to_string()));
        assert!(pool.contains(&"xmlrpc.php".to_string()));
        assert_eq!(pool.len(), 3, "deduped across the whole tree: {pool:?}");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn list_pool_serves_each_arm_in_order_skipping_tried_then_stops() {
        let mut pool = ListPool::new(vec![
            "a".into(), "b".into(), "c".into(), "d".into(), "e".into(),
        ]);
        let mut tried: HashSet<String> = HashSet::new();
        tried.insert("b".into()); // already scheduled for this arm -> skipped, scanned past

        let first = pool.next_chunk("/x/", &tried, 2); // scans a(ok), b(skip), c(ok)
        assert_eq!(first, ["a", "c"]);
        for w in &first {
            tried.insert(w.clone());
        }
        let second = pool.next_chunk("/x/", &tried, 10); // d, e
        assert_eq!(second, ["d", "e"]);
        assert!(pool.arm_exhausted("/x/"));
        assert!(pool.next_chunk("/x/", &tried, 5).is_empty()); // no wrap-around
    }

    #[test]
    fn list_pool_reapplies_whole_wordlist_per_directory() {
        let mut pool = ListPool::new(vec!["a".into(), "b".into(), "c".into()]);
        let empty: HashSet<String> = HashSet::new();
        // each arm walks the full pool independently: "a" is served under BOTH arms
        assert_eq!(pool.next_chunk("/one/", &empty, 3), ["a", "b", "c"]);
        assert!(pool.arm_exhausted("/one/"));
        assert!(!pool.arm_exhausted("/two/")); // unseen arm still has the whole pool
        assert!(pool.any_remaining(&["/one/".into(), "/two/".into()]));
        assert_eq!(pool.next_chunk("/two/", &empty, 3), ["a", "b", "c"]);
        assert!(!pool.any_remaining(&["/one/".into(), "/two/".into()]));
    }

    #[test]
    fn seen_is_host_independent_on_path() {
        let mut s = SeenPaths::new();
        assert!(s.insert("https://x.test/api/v1"));
        assert!(!s.insert("https://x.test/api/v1/")); // same path key
        assert!(s.insert("https://x.test/api/v2"));
        assert_eq!(s.len(), 2);
    }
}
