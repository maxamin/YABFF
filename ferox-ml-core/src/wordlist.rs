//! Per-round wordlist construction and cross-round de-duplication.

use std::collections::HashSet;

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

/// Load every file in `dir` (sorted by path for determinism) and merge them into
/// one de-duplicated, order-preserving pool. Lines are trimmed; empty lines and
/// `#` comments are skipped. `cap` bounds the total number of entries. Files that
/// can't be read (e.g. binary) are skipped rather than aborting the whole load.
pub fn load_list_dir(dir: &str, cap: usize) -> std::io::Result<Vec<String>> {
    let mut files: Vec<std::path::PathBuf> = std::fs::read_dir(dir)?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.is_file())
        .collect();
    files.sort(); // determinism: filename order, not read_dir order
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
                if pool.len() >= cap {
                    return Ok(pool);
                }
            }
        }
    }
    Ok(pool)
}

/// A forward cursor over a merged list pool. Each `next_chunk` returns the next
/// run of entries not already tried, in pool order, advancing past everything it
/// scans. It **stops** at the end of the pool (no wrap-around): once exhausted it
/// returns an empty chunk.
pub struct ListCursor {
    pool: Vec<String>,
    pos: usize,
}

impl ListCursor {
    pub fn new(pool: Vec<String>) -> Self {
        Self { pool, pos: 0 }
    }

    /// Up to `n` entries from the pool not present in `tried`, in pool order.
    /// Advances the cursor past every entry it inspects (tried or not), so a given
    /// pool position is served at most once across the campaign.
    pub fn next_chunk(&mut self, tried: &HashSet<String>, n: usize) -> Vec<String> {
        let mut out = Vec::with_capacity(n.min(16));
        while self.pos < self.pool.len() && out.len() < n {
            let w = &self.pool[self.pos];
            self.pos += 1;
            if !tried.contains(w) {
                out.push(w.clone());
            }
        }
        out
    }

    /// Whether the cursor has consumed the whole pool.
    pub fn is_exhausted(&self) -> bool {
        self.pos >= self.pool.len()
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
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn list_cursor_serves_unseen_in_order_then_stops() {
        let mut cur = ListCursor::new(vec![
            "a".into(), "b".into(), "c".into(), "d".into(), "e".into(),
        ]);
        let mut tried: HashSet<String> = HashSet::new();
        tried.insert("b".into()); // already scheduled -> skipped, but still scanned past

        let first = cur.next_chunk(&tried, 2); // scans a(ok), b(skip), c(ok)
        assert_eq!(first, ["a", "c"]);
        for w in &first {
            tried.insert(w.clone());
        }
        let second = cur.next_chunk(&tried, 10); // d, e
        assert_eq!(second, ["d", "e"]);
        assert!(cur.is_exhausted());
        assert!(cur.next_chunk(&tried, 5).is_empty()); // no wrap-around
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
