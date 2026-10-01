//! feroxml — an adaptive, ML-driven fuzzing orchestrator built around the real
//! `feroxbuster` binary.
//!
//! It adds two cooperating engines on top of feroxbuster:
//!
//! 1. **K-Means / nearest-centroid target fingerprinting** ([`fingerprint`]) —
//!    a bounded probe classifies the target into one of four framework profiles.
//! 2. **Variable-order Markov / PPM path prediction** ([`markov`]) — seeded per
//!    profile, it proposes the most probable next path tokens for any discovered
//!    directory.
//!
//! A **Thompson-sampling scheduler** ([`scheduler`]) decides which directory to
//! expand next, a **BM25 ranker** ([`ranking`]) orders candidate paths, and a
//! **SimHash / response-signature soft-404 filter** ([`dedup`]) keeps noise out
//! of the reward signal. The [`orchestrator`] wires it all together.
//!
//! ## feroxbuster reality this is built around
//! feroxbuster's `--json` output is newline-delimited JSON, one object per line,
//! tagged by a `type` field (`configuration`, `response`, `statistics`, `log`).
//! Only `response` objects are findings. feroxbuster **cannot** accept new
//! wordlist entries into a running scan, so the adaptive loop works by spawning
//! new, bounded feroxbuster scans per round — see [`orchestrator`].

// The ML engines are shared with feroxbuster-ml via the `ferox-ml-core` crate;
// re-export them under their historical paths so the rest of feroxml
// (`crate::markov`, `crate::fingerprint`, `crate::PROFILES`, ...) is unchanged.
pub use ferox_ml_core::{
    config, dedup, ferox, fingerprint, interfaces, markov, orchestrator, profiles, ranking, rng,
    scheduler, scope, tokenize, wordlist, ProbeResp, PROFILES,
};
