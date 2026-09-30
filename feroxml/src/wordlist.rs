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
    fn seen_is_host_independent_on_path() {
        let mut s = SeenPaths::new();
        assert!(s.insert("https://x.test/api/v1"));
        assert!(!s.insert("https://x.test/api/v1/")); // same path key
        assert!(s.insert("https://x.test/api/v2"));
        assert_eq!(s.len(), 2);
    }
}
