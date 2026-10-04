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
#[derive(Clone, Copy, Debug)]
pub struct Signature {
    pub status: u16,
    pub len_bucket: u64,
    pub words: u64,
    pub lines: u64,
    /// Hash of the redirect `Location` (0 when absent / not a redirect).
    pub loc: u64,
    /// 64-bit SimHash of the response body (0 when no body is available, e.g. the
    /// NDJSON stream which carries only counts). Used ONLY by the fuzzy body match
    /// (Layer 2b) via Hamming distance; deliberately excluded from equality and
    /// hashing (see the manual impls below) so the exact `bogus` set and the
    /// calibration guard's distinct-signature test keep their numeric-only
    /// semantics and don't shift when bodies vary by a token.
    pub simhash: u64,
}

// Equality/hash cover the numeric shape only — NOT `simhash`. Two responses with
// the same shape are "the same signature" for the exact filter and the distinct-
// signature catch-all guard regardless of body hash; the body hash is a separate,
// fuzzy signal applied by `shape_close_to`.
impl PartialEq for Signature {
    fn eq(&self, o: &Self) -> bool {
        self.status == o.status
            && self.len_bucket == o.len_bucket
            && self.words == o.words
            && self.lines == o.lines
            && self.loc == o.loc
    }
}
impl Eq for Signature {}
impl std::hash::Hash for Signature {
    fn hash<H: std::hash::Hasher>(&self, h: &mut H) {
        self.status.hash(h);
        self.len_bucket.hash(h);
        self.words.hash(h);
        self.lines.hash(h);
        self.loc.hash(h);
    }
}

impl Signature {
    pub fn new(status: u16, content_length: u64, words: u64, lines: u64) -> Self {
        Self {
            status,
            len_bucket: content_length / 16, // 16-byte buckets
            words,
            lines,
            loc: 0,
            simhash: 0,
        }
    }

    /// Fold a body SimHash into the signature (Layer 2b). `0` leaves it absent.
    pub fn with_simhash(mut self, simhash: u64) -> Self {
        self.simhash = simhash;
        self
    }

    /// Fold a redirect `Location` into the signature. Only meaningful for 3xx
    /// (callers pass `None` otherwise); `None` or an empty value leaves `loc` 0.
    ///
    /// The `Location` is keyed on its **path only** — the volatile tail (query
    /// string `?…` and matrix/session params `;jsessionid=…`) is stripped before
    /// hashing. A catch-all auth wall redirects every path to the *same* target
    /// but echoes the original path (or a per-request session id) in that tail, so
    /// without stripping it every redirect gets a unique `loc` and the soft-404
    /// filter can never generalize — the runaway that floods list-mode output on
    /// such hosts. Keying on the path collapses them to one signature the filter
    /// catches, while a genuine self-redirect (`/admin` → `/admin/`) keeps a
    /// distinct path and still survives.
    pub fn with_location(mut self, location: Option<&str>) -> Self {
        self.loc = match location {
            Some(l) if !l.is_empty() => {
                let path = l.split(['?', ';']).next().unwrap_or(l);
                fnv1a64(path.as_bytes())
            }
            _ => 0,
        };
        self
    }
}

/// Relative tolerance for the fuzzy body-shape match (Layer 2). A response whose
/// bucketed length, word count and line count are all within this fraction of a
/// learned bogus shape is treated as the same template. Small enough that
/// genuinely different pages don't collide, large enough to absorb the per-path
/// jitter of a 200 "shell" catch-all (a CSP nonce, a build hash, the echoed path
/// in a canonical tag) that pushes each response just outside an exact bucket.
const SHAPE_REL_TOL: f64 = 0.10;

/// Max SimHash Hamming distance for two bodies to count as the same template
/// (Layer 2b). A SPA "shell" served for every path differs by only the few tokens
/// that vary per path (an echoed path, a nonce), so near-duplicate bodies sit a
/// handful of bits apart, while a genuinely different page is tens of bits away —
/// this threshold separates the two and, unlike the size band, is independent of
/// how much the body's *length* varies.
const SIMHASH_HAMMING_TOL: u32 = 6;

impl Signature {
    /// Whether this (body) response matches `other`'s template. Only meaningful
    /// for non-redirects (both `loc == 0`); redirects are matched exactly via the
    /// `loc` hash. When a body SimHash is present on both (Layer 2b), it is the
    /// decisive, size-independent signal — a template match regardless of how the
    /// body length varies per path. Otherwise (no body hash, e.g. the NDJSON
    /// stream) it falls back to the [`SHAPE_REL_TOL`] length/word/line band.
    fn shape_close_to(&self, other: &Signature) -> bool {
        if self.status != other.status || self.loc != 0 || other.loc != 0 {
            return false;
        }
        if self.simhash != 0 && other.simhash != 0 {
            return hamming(self.simhash, other.simhash) <= SIMHASH_HAMMING_TOL;
        }
        fn within(a: u64, b: u64) -> bool {
            let (a, b) = (a as f64, b as f64);
            let m = a.max(b);
            if m == 0.0 {
                return true; // both empty
            }
            (a - b).abs() / m <= SHAPE_REL_TOL
        }
        within(self.len_bucket, other.len_bucket)
            && within(self.words, other.words)
            && within(self.lines, other.lines)
    }
}

/// Learns bogus signatures from known-negative probes, then flags matches.
///
/// Two layers. **Exact** (`bogus`) catches identical templates and — since a
/// redirect's `loc` now keys on the Location path — path-echoing 302 catch-alls.
/// **Fuzzy** (`shapes`) catches a *200 "shell" catch-all*: a host that answers
/// every path with the same page whose body varies just enough per path
/// (nonce/build-hash/echoed path) to dodge an exact signature, so each response
/// would otherwise be recorded. The fuzzy layer only ever contains shapes learned
/// from the random calibration probes, so it is **inert on a normal host** (where
/// those probes 404 and nothing is learned) — it suppresses a catch-all's template
/// without risking real findings on clean targets, and a response that is an
/// outlier from every learned shape still survives.
#[derive(Default)]
pub struct SoftNotFoundFilter {
    bogus: HashSet<Signature>,
    /// Body shapes (loc == 0) learned from bogus probes, for the fuzzy match.
    shapes: Vec<Signature>,
}

impl SoftNotFoundFilter {
    pub fn new() -> Self {
        Self::default()
    }

    /// Record the signature of a response known to be a soft-404 (e.g. from a
    /// random, almost-certainly-absent probe path).
    pub fn learn_bogus(&mut self, sig: Signature) {
        self.bogus.insert(sig);
        // Only body responses feed the fuzzy layer; redirects (loc != 0) are keyed
        // exactly by their path-normalized Location.
        if sig.loc == 0 {
            self.shapes.push(sig);
        }
    }

    /// True if a response looks like a learned soft-404/template.
    ///
    /// * **Body response** (a SimHash is present): decided purely by template
    ///   similarity against the learned body shapes (Layer 2b). This both *matches*
    ///   a shell served at wildly varying sizes (recall) and *spares* a real page
    ///   that merely shares a soft-404's coarse size but whose content is far in
    ///   token space (precision) — the coarse size bucket never filters a body on
    ///   its own.
    /// * **No body** (redirect, or the NDJSON stream with only counts): exact
    ///   signature, or the [`SHAPE_REL_TOL`] size band (Layer 2).
    pub fn is_soft_not_found(&self, sig: &Signature) -> bool {
        if sig.simhash != 0 {
            return self.shapes.iter().any(|b| sig.shape_close_to(b));
        }
        if self.bogus.contains(sig) {
            return true;
        }
        sig.loc == 0 && self.shapes.iter().any(|b| sig.shape_close_to(b))
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

    #[test]
    fn path_echoing_catch_all_redirect_collapses_to_one_signature() {
        // An auth wall 302s every path to the SAME login, echoing the requested
        // path (and a per-request session id) in the volatile tail. Learned from
        // one random calibration probe, every other such redirect must be filtered
        // — otherwise the whole wordlist is recorded (the observed runaway).
        let learn = |loc: &str| Signature::new(302, 0, 0, 0).with_location(Some(loc));
        let mut f = SoftNotFoundFilter::new();
        f.learn_bogus(learn("https://h.test/login;jsessionid=AAAAAAAA"));

        for loc in [
            "https://h.test/login;jsessionid=BBBBBBBB",      // different session id
            "https://h.test/login?returnUrl=/admin",          // query echoes the path
            "https://h.test/login?returnUrl=/wp-admin&x=1",   // different echoed path
            "https://h.test/login;jsessionid=CC?next=/secret",// both tails
        ] {
            assert!(
                f.is_soft_not_found(&learn(loc)),
                "catch-all redirect must collapse to the learned signature: {loc}"
            );
        }

        // A genuine self-redirect to a DIFFERENT path (no volatile tail) still has
        // a distinct Location path, so it survives the filter.
        assert!(!f.is_soft_not_found(&learn("https://h.test/admin/")));
    }

    #[test]
    fn fuzzy_shape_catches_200_shell_catch_all_but_keeps_outliers() {
        // A 200 "shell" host returns ~the same large page for every path, varying
        // a few % per path (nonce / echoed path) so exact signatures differ.
        // Learned from one random probe, near-identical shells must be filtered;
        // a genuinely different page (a real finding) must survive.
        let mut f = SoftNotFoundFilter::new();
        f.learn_bogus(Signature::new(200, 52_000, 4_000, 800)); // the shell

        // within SHAPE_REL_TOL on all axes -> same template -> filtered
        assert!(f.is_soft_not_found(&Signature::new(200, 53_500, 4_050, 806)));
        assert!(f.is_soft_not_found(&Signature::new(200, 50_200, 3_900, 790)));

        // a real, clearly different page is an outlier on every axis -> survives
        assert!(!f.is_soft_not_found(&Signature::new(200, 1_200, 90, 18)));
        // same size but a different status is not the shell
        assert!(!f.is_soft_not_found(&Signature::new(403, 52_000, 4_000, 800)));
    }

    #[test]
    fn fuzzy_layer_is_inert_until_a_bogus_is_learned() {
        // On a clean host the random probes 404 and nothing is learned, so the
        // fuzzy layer must never fire — real findings are never suppressed.
        let f = SoftNotFoundFilter::new();
        assert!(!f.is_soft_not_found(&Signature::new(200, 52_000, 4_000, 800)));
    }

    #[test]
    fn simhash_body_match_is_size_independent() {
        // A realistic SPA shell: a large body of shared markup/script tokens, with
        // only the echoed path differing per request (one token).
        let shared: String = (0..400).map(|i| format!("tok{i} ")).collect();
        let shell = |p: &str| format!("{shared} data path {p} end");
        let mut f = SoftNotFoundFilter::new();
        // learned from a random probe (small length column here)...
        f.learn_bogus(Signature::new(200, 500, 40, 1).with_simhash(simhash(&shell("zzabsentx"))));

        // ...a real path returns the SAME template with a FAR larger length
        // (outside the 10% size band). SimHash still matches -> filtered. This is
        // exactly the case size-clustering (Layer 2) misses.
        assert!(
            f.is_soft_not_found(
                &Signature::new(200, 90_000, 9_000, 1).with_simhash(simhash(&shell("admin")))
            ),
            "same template at a very different size must match via SimHash"
        );

        // A genuinely different page of the SAME size as the learned shell is an
        // outlier in token space -> survives. Size clustering could NOT tell these
        // apart; SimHash can.
        let real: String = (0..400).map(|i| format!("report{i} ")).collect();
        assert!(
            !f.is_soft_not_found(&Signature::new(200, 500, 40, 1).with_simhash(simhash(&real))),
            "a different page of the same size must survive"
        );
    }
}
