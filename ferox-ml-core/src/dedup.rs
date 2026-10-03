//! Noise suppression so template/soft-404 pages don't pollute the bandit reward.
//!
//! Two complementary tools:
//! * [`simhash`] — a 64-bit SimHash of arbitrary text, with [`hamming`] distance,
//!   used on the Phase-1 probe bodies (where we actually have a body).
//! * [`SoftNotFoundFilter`] — the stream filter. feroxbuster's `--json` gives us
//!   `status`, `content_length`, `word_count`, `line_count` (no body), so this
//!   learns the *response signatures* of known-bogus probes (random paths) and
//!   drops later responses whose signature matches — the same idea feroxbuster's
//!   own wildcard filter uses, applied on our side of the stream.

use std::collections::HashSet;

/// 64-bit SimHash of whitespace-delimited tokens in `text`.
pub fn simhash(text: &str) -> u64 {
    let mut bits = [0i64; 64];
    let mut any = false;
    for tok in text.split(|c: char| !c.is_alphanumeric()).filter(|t| !t.is_empty()) {
        any = true;
        let h = fnv1a64(tok.to_lowercase().as_bytes());
        for (i, b) in bits.iter_mut().enumerate() {
            if (h >> i) & 1 == 1 {
                *b += 1;
            } else {
                *b -= 1;
            }
        }
    }
    if !any {
        return 0;
    }
    let mut out = 0u64;
    for (i, b) in bits.iter().enumerate() {
        if *b > 0 {
            out |= 1u64 << i;
        }
    }
    out
}

/// Hamming distance between two SimHash signatures.
pub fn hamming(a: u64, b: u64) -> u32 {
    (a ^ b).count_ones()
}

fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for &byte in bytes {
        h ^= byte as u64;
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    h
}

/// A coarse response fingerprint available from feroxbuster's JSON stream.
/// `content_length` is bucketed so near-identical templates collide.
///
/// For a redirect (3xx) the body carries no signal (it is usually empty), so
/// two redirects to *different* targets would otherwise share one signature and
/// a genuine directory hit (`/admin` → `/admin/`) would be lumped with the
/// random calibration probe's redirect and dropped as a soft-404. `loc` folds
/// the `Location` target into the signature so only redirects pointing at the
/// *same* place collide — a generic catch-all redirect stays filtered, while a
/// self-referential directory redirect (unique `Location` per path) survives.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Signature {
    pub status: u16,
    pub len_bucket: u64,
    pub words: u64,
    pub lines: u64,
    /// Hash of the redirect `Location` (0 when absent / not a redirect).
    pub loc: u64,
}

impl Signature {
    pub fn new(status: u16, content_length: u64, words: u64, lines: u64) -> Self {
        Self {
            status,
            len_bucket: content_length / 16, // 16-byte buckets
            words,
            lines,
            loc: 0,
        }
    }

    /// Fold a redirect `Location` into the signature. Only meaningful for 3xx
    /// (callers pass `None` otherwise); `None` or an empty value leaves `loc` 0.
    pub fn with_location(mut self, location: Option<&str>) -> Self {
        self.loc = match location {
            Some(l) if !l.is_empty() => fnv1a64(l.as_bytes()),
            _ => 0,
        };
        self
    }
}

/// Learns bogus signatures from known-negative probes, then flags matches.
#[derive(Default)]
pub struct SoftNotFoundFilter {
    bogus: HashSet<Signature>,
}

impl SoftNotFoundFilter {
    pub fn new() -> Self {
        Self::default()
    }

    /// Record the signature of a response known to be a soft-404 (e.g. from a
    /// random, almost-certainly-absent probe path).
    pub fn learn_bogus(&mut self, sig: Signature) {
        self.bogus.insert(sig);
    }

    /// True if a response looks like a learned soft-404/template.
    pub fn is_soft_not_found(&self, sig: &Signature) -> bool {
        self.bogus.contains(sig)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simhash_near_for_similar_text() {
        let a = simhash("the quick brown fox jumps over the lazy dog");
        let b = simhash("the quick brown fox jumps over the lazy cat");
        let c = simhash("completely unrelated content here indeed");
        assert!(hamming(a, b) < hamming(a, c));
    }

    #[test]
    fn soft_404_signature_matching() {
        let mut f = SoftNotFoundFilter::new();
        let bogus = Signature::new(200, 1500, 120, 30);
        f.learn_bogus(bogus);
        assert!(f.is_soft_not_found(&Signature::new(200, 1500, 120, 30)));
        assert!(f.is_soft_not_found(&Signature::new(200, 1495, 120, 30))); // same 16-byte bucket
        assert!(!f.is_soft_not_found(&Signature::new(200, 4096, 400, 90)));
    }

    #[test]
    fn redirect_location_distinguishes_otherwise_identical_signatures() {
        // All three are bare 301s with an identical (empty) body, so without the
        // Location they would share one signature.
        let probe = Signature::new(301, 17, 2, 0).with_location(Some("https://connect.example.com/"));
        let mut f = SoftNotFoundFilter::new();
        f.learn_bogus(probe);

        // A catch-all redirect to the SAME target is still a soft-404.
        let catch_all =
            Signature::new(301, 17, 2, 0).with_location(Some("https://connect.example.com/"));
        assert!(f.is_soft_not_found(&catch_all));

        // A real directory hit redirects to its own path -> different Location ->
        // different signature -> NOT filtered.
        let real_dir =
            Signature::new(301, 17, 2, 0).with_location(Some("https://example.com/admin/"));
        assert!(!f.is_soft_not_found(&real_dir));

        // No Location (loc = 0) is distinct from a located redirect.
        assert!(!f.is_soft_not_found(&Signature::new(301, 17, 2, 0)));
    }
}
