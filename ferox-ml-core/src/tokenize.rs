//! Path tokenization.
//!
//! Two levels:
//! * [`path_segments`] splits a URL path into its directory/file **segments**
//!   (`/api/v1/users` -> `["api", "v1", "users"]`). The Markov engine predicts
//!   at this level, matching the seed matrices.
//! * [`subword_tokens`] breaks a single segment into normalized **subwords**
//!   (`getUserById` -> `["get","user","by","id"]`, `wp-login.php` ->
//!   `["wp","login","php"]`). This is a deterministic, rule-based BPE-style
//!   splitter: it generalizes across `user`/`users`/`userId` for the BM25
//!   ranker and the prediction fallback. A trained BPE/WordPiece model could
//!   replace it behind the same function signature.

/// Split a URL or path into non-empty lowercase segments.
pub fn path_segments(path: &str) -> Vec<String> {
    // strip scheme+host if a full URL was passed
    let tail = if let Some(idx) = path.find("://") {
        match path[idx + 3..].find('/') {
            Some(slash) => &path[idx + 3..][slash..],
            None => "",
        }
    } else {
        path
    };
    let cut = tail.split(['?', '#']).next().unwrap_or(tail);
    cut.split('/')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_lowercase())
        .collect()
}

/// The last meaningful segment of a path, or empty string for the root.
pub fn last_segment(path: &str) -> String {
    path_segments(path).pop().unwrap_or_default()
}

/// Break a single segment into normalized subword tokens.
pub fn subword_tokens(segment: &str) -> Vec<String> {
    let seg = segment.trim();
    if seg.is_empty() {
        return vec![];
    }
    let mut out: Vec<String> = Vec::new();
    let mut cur = String::new();
    let mut prev: Option<char> = None;
    for ch in seg.chars() {
        let boundary = matches!(ch, '_' | '-' | '.' | ' ' | '+');
        if boundary {
            if !cur.is_empty() {
                out.push(std::mem::take(&mut cur));
            }
            prev = None;
            continue;
        }
        // camelCase / letter<->digit transitions start a new token
        if let Some(p) = prev {
            let upper_hump = p.is_lowercase() && ch.is_uppercase();
            let digit_edge = p.is_ascii_digit() != ch.is_ascii_digit();
            if (upper_hump || digit_edge) && !cur.is_empty() {
                out.push(std::mem::take(&mut cur));
            }
        }
        for lc in ch.to_lowercase() {
            cur.push(lc);
        }
        prev = Some(ch);
    }
    if !cur.is_empty() {
        out.push(cur);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn segments_from_url_and_path() {
        assert_eq!(path_segments("https://x.test/api/v1/users"), ["api", "v1", "users"]);
        assert_eq!(path_segments("/wp-content/plugins/"), ["wp-content", "plugins"]);
        assert_eq!(path_segments("/a?b=c#d"), ["a"]);
        assert!(path_segments("https://x.test/").is_empty());
    }

    #[test]
    fn last_segment_works() {
        assert_eq!(last_segment("/api/v1/users"), "users");
        assert_eq!(last_segment("https://x.test/"), "");
    }

    #[test]
    fn subwords_generalize() {
        assert_eq!(subword_tokens("getUserById"), ["get", "user", "by", "id"]);
        assert_eq!(subword_tokens("wp-login.php"), ["wp", "login", "php"]);
        assert_eq!(subword_tokens("api_v1"), ["api", "v", "1"]);
        assert_eq!(subword_tokens("users"), ["users"]);
    }
}
