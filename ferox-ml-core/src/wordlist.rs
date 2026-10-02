//! Per-round wordlist construction and cross-round de-duplication.

use std::collections::{HashMap, HashSet};

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
/// de-duplicated, order-preserving pool. Files are read in sorted full-path order
/// for determinism. Lines are trimmed; empty lines and `#` comments are skipped.
/// `cap` bounds the total number of entries; **`cap == 0` means unlimited** (use
/// every entry in the tree). Files that can't be read (e.g. binary) are skipped
/// rather than aborting the whole load.
pub fn load_list_dir(dir: &str, cap: usize) -> std::io::Result<Vec<String>> {
    // recursively collect every file under `dir`
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
    files.sort(); // determinism: full-path order, not read_dir order
    let unlimited = cap == 0;
    let mut pool: Vec<String> = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();
    for f in files {
        let Ok(text) = std::fs::read_to_string(&f) else {
            continue; // skip unreadable/binary files
        };
        for line in text.lines() {
            let w = line.trim();
            if w.is_empty() || w.starts_with('#') {
                continue;
            }
            if seen.insert(w.to_string()) {
                pool.push(w.to_string());
                if !unlimited && pool.len() >= cap {
                    return Ok(pool);
                }
            }
        }
    }
    Ok(pool)
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
    fn load_list_dir_merges_sorted_dedups_and_caps() {
        let dir = std::env::temp_dir().join(format!("ferox-lists-{}", std::process::id()));
        let _ = std::fs::create_dir_all(&dir);
        // b.txt sorts after a.txt; "api" appears in both (dedup); comments/empties skipped
        std::fs::write(dir.join("a.txt"), "admin\n# comment\nlogin\n\napi\n").unwrap();
        std::fs::write(dir.join("b.txt"), "api\nusers\n").unwrap();
        let pool = load_list_dir(dir.to_str().unwrap(), 100).unwrap();
        assert_eq!(pool, ["admin", "login", "api", "users"]); // a.txt first, deduped
        // cap bounds the total
        let capped = load_list_dir(dir.to_str().unwrap(), 2).unwrap();
        assert_eq!(capped, ["admin", "login"]);
        // cap == 0 means unlimited (every entry)
        let all = load_list_dir(dir.to_str().unwrap(), 0).unwrap();
        assert_eq!(all, ["admin", "login", "api", "users"]);
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
