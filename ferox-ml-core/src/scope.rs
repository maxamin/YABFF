//! Scope enforcement. Every candidate URL is checked before it is scanned; an
//! out-of-scope host is dropped and logged. An engagement never touches hosts
//! outside the provided scope.

use url::Url;

/// The set of hosts considered in-scope (the target host plus any `--scope`).
#[derive(Clone, Debug, Default)]
pub struct Scope {
    hosts: Vec<String>,
}

impl Scope {
    /// Build scope from the target URL plus extra host/URL strings.
    pub fn new(target: &str, extra: &[String]) -> anyhow::Result<Self> {
        let mut hosts = Vec::new();
        if let Some(h) = host_of(target) {
            hosts.push(h);
        }
        for e in extra {
            if let Some(h) = host_of(e) {
                hosts.push(h);
            } else {
                // a bare domain like "example.com"
                let t = e.trim().to_lowercase();
                if !t.is_empty() {
                    hosts.push(t);
                }
            }
        }
        if hosts.is_empty() {
            anyhow::bail!("could not determine any in-scope host from target/scope");
        }
        Ok(Self { hosts })
    }

    /// In-scope when the URL's host equals, or is a subdomain of, a scope host.
    pub fn allows(&self, candidate: &str) -> bool {
        let Some(host) = host_of(candidate) else {
            return false;
        };
        self.hosts.iter().any(|h| host == *h || host.ends_with(&format!(".{h}")))
    }

    pub fn hosts(&self) -> &[String] {
        &self.hosts
    }
}

/// Lowercase host of a URL string, or `None` if it has no parseable host.
pub fn host_of(s: &str) -> Option<String> {
    let s = s.trim();
    // allow bare "host[:port]/path" by giving it a scheme for parsing
    let parsed = if s.contains("://") {
        Url::parse(s).ok()
    } else {
        Url::parse(&format!("https://{s}")).ok()
    };
    parsed
        .and_then(|u| u.host_str().map(|h| h.to_lowercase()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn target_host_is_in_scope() {
        let s = Scope::new("https://app.example.com/", &[]).unwrap();
        assert!(s.allows("https://app.example.com/api/v1"));
        assert!(!s.allows("https://evil.test/x"));
    }

    #[test]
    fn subdomains_of_scope_allowed() {
        let s = Scope::new("https://example.com", &["internal.test".into()]).unwrap();
        assert!(s.allows("https://api.example.com/x"));
        assert!(s.allows("https://foo.internal.test/y"));
        assert!(!s.allows("https://example.com.evil.test/"));
    }

    #[test]
    fn host_parsing() {
        assert_eq!(host_of("https://a.b.c/x").as_deref(), Some("a.b.c"));
        assert_eq!(host_of("a.b.c:8443/x").as_deref(), Some("a.b.c"));
    }
}
