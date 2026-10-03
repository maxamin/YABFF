//! Phase 2 — variable-order Markov / PPM path predictor.
//!
//! Stores transition weights keyed by a **context** (a slice of recent path
//! segments). Prediction uses prediction-by-partial-matching: it backs off from
//! the longest context that has evidence down to the empty (global) context.
//! Seeded per profile from [`crate::profiles::seed_matrix`], then updated online
//! as feroxbuster discovers real paths.
//!
//! `predictor = "markov1"` in the config forces `max_order = 1` (a plain
//! first-order chain) behind the same interface.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::interfaces::Predictor;
use crate::profiles;
use crate::tokenize::{path_segments, subword_tokens};

#[derive(Debug, Clone)]
pub struct MarkovModel {
    /// context (tokens) -> { next_token -> weight }
    rows: HashMap<Vec<String>, HashMap<String, f64>>,
    max_order: usize,
    alpha: f64,
    threshold: f64,
}

#[derive(Serialize, Deserialize)]
struct MarkovDto {
    rows: Vec<(Vec<String>, Vec<(String, f64)>)>,
    max_order: usize,
    alpha: f64,
    threshold: f64,
}

impl MarkovModel {
    pub fn new(max_order: usize, alpha: f64, threshold: f64) -> Self {
        Self {
            rows: HashMap::new(),
            max_order: max_order.max(1),
            alpha,
            threshold,
        }
    }

    /// Build a model seeded with a profile's transition matrix.
    pub fn seeded(profile: &str, max_order: usize, alpha: f64, threshold: f64) -> Self {
        let mut m = Self::new(max_order, alpha, threshold);
        for (from, tos) in profiles::seed_matrix(profile) {
            let ctx: Vec<String> = if from.is_empty() {
                vec![]
            } else {
                vec![from.to_string()]
            };
            for (to, w) in tos {
                m.add(&ctx, to, *w);
            }
        }
        m
    }

    fn add(&mut self, context: &[String], next: &str, weight: f64) {
        let row = self.rows.entry(context.to_vec()).or_default();
        *row.entry(next.to_string()).or_insert(0.0) += weight;
    }

    /// Online-learn every context→next transition present in a discovered path.
    pub fn learn_path(&mut self, path: &str) {
        let segs = path_segments(path);
        if segs.is_empty() {
            return;
        }
        // empty context -> first segment (a "where do scans start" signal)
        self.add(&[], &segs[0], 1.0);
        for i in 0..segs.len() {
            let max_o = self.max_order.min(i);
            for order in 1..=max_o {
                let ctx = &segs[i - order..i];
                self.add(ctx, &segs[i], 1.0);
            }
        }
    }

    /// Fold another model's transition weights into this one (used to merge a
    /// model learned from the labs onto a profile seed, and to accumulate
    /// learning across targets/runs).
    pub fn merge(&mut self, other: &MarkovModel) {
        for (ctx, row) in &other.rows {
            let dst = self.rows.entry(ctx.clone()).or_default();
            for (tok, w) in row {
                *dst.entry(tok.clone()).or_insert(0.0) += *w;
            }
        }
    }

    /// Number of distinct contexts the model has learned (for reporting).
    pub fn context_count(&self) -> usize {
        self.rows.len()
    }

    /// Total number of distinct transitions across all contexts (for reporting).
    pub fn transition_count(&self) -> usize {
        self.rows.values().map(|r| r.len()).sum()
    }

    /// Pure MLE distribution for one context, normalized to sum to 1.0.
    /// Used for verification; prediction uses the smoothed path below.
    pub fn distribution(&self, context: &[String]) -> Vec<(String, f64)> {
        let Some(row) = self.rows.get(context) else {
            return vec![];
        };
        let total: f64 = row.values().sum();
        if total <= 0.0 {
            return vec![];
        }
        let mut out: Vec<(String, f64)> =
            row.iter().map(|(k, v)| (k.clone(), v / total)).collect();
        out.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap().then(a.0.cmp(&b.0)));
        out
    }

    /// PPM back-off: the longest context suffix that has a row wins. Returns
    /// the winning context (owned); an empty `Vec` is the global context.
    fn best_context(&self, recent: &[String]) -> Option<Vec<String>> {
        let max_o = self.max_order.min(recent.len());
        for order in (1..=max_o).rev() {
            let ctx = recent[recent.len() - order..].to_vec();
            if self.rows.contains_key(&ctx) {
                return Some(ctx);
            }
        }
        // fall back to the empty/global context
        if self.rows.contains_key(&Vec::<String>::new()) {
            Some(vec![])
        } else {
            None
        }
    }

    /// Smoothed top-N prediction for a sequence of recent segments.
    pub fn predict_tokens(&self, recent: &[String], top_n: usize) -> Vec<(String, f64)> {
        let Some(ctx) = self.best_context(recent) else {
            return vec![];
        };
        let Some(row) = self.rows.get(&ctx) else {
            return vec![];
        };
        let total: f64 = row.values().sum();
        let vocab = row.len() as f64;
        let denom = total + self.alpha * vocab.max(1.0);
        let mut out: Vec<(String, f64)> = row
            .iter()
            .map(|(k, v)| (k.clone(), (v + self.alpha) / denom))
            .filter(|(_, p)| *p >= self.threshold)
            .collect();
        out.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap().then(a.0.cmp(&b.0)));
        out.truncate(top_n);
        out
    }

    /// E8 — subword back-off. When no context has segment-level evidence for the
    /// last recent segment, surface learned tokens that share subword tokens with
    /// it (e.g. after `getUser…`, a previously-learned `getUserById`). Scored by
    /// subword Jaccard overlap, highest first.
    fn subword_candidates(&self, recent: &[String], top_n: usize) -> Vec<(String, f64)> {
        let Some(last) = recent.last() else {
            return vec![];
        };
        let q: std::collections::HashSet<String> = subword_tokens(last).into_iter().collect();
        if q.is_empty() {
            return vec![];
        }
        let mut seen: std::collections::HashSet<&str> = std::collections::HashSet::new();
        let mut scored: Vec<(String, f64)> = Vec::new();
        for row in self.rows.values() {
            for tok in row.keys() {
                if tok == last || !seen.insert(tok.as_str()) {
                    continue;
                }
                let t: std::collections::HashSet<String> =
                    subword_tokens(tok).into_iter().collect();
                let inter = q.intersection(&t).count();
                if inter == 0 {
                    continue;
                }
                let union = q.union(&t).count().max(1);
                scored.push((tok.clone(), inter as f64 / union as f64));
            }
        }
        scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap().then(a.0.cmp(&b.0)));
        scored.truncate(top_n);
        scored
    }

    /// Build the serializable DTO (shared by every encoder).
    fn to_dto(&self) -> MarkovDto {
        MarkovDto {
            rows: self
                .rows
                .iter()
                .map(|(k, v)| (k.clone(), v.iter().map(|(t, w)| (t.clone(), *w)).collect()))
                .collect(),
            max_order: self.max_order,
            alpha: self.alpha,
            threshold: self.threshold,
        }
    }

    /// Rebuild from the DTO (shared by every decoder).
    fn from_dto(dto: MarkovDto) -> Self {
        let mut rows: HashMap<Vec<String>, HashMap<String, f64>> = HashMap::new();
        for (ctx, tos) in dto.rows {
            rows.insert(ctx, tos.into_iter().collect());
        }
        Self {
            rows,
            max_order: dto.max_order,
            alpha: dto.alpha,
            threshold: dto.threshold,
        }
    }

    pub fn to_json(&self) -> anyhow::Result<String> {
        // compact, not pretty: this feeds --ml-export and must stay small
        Ok(serde_json::to_string(&self.to_dto())?)
    }

    pub fn from_json(text: &str) -> anyhow::Result<Self> {
        Ok(Self::from_dto(serde_json::from_str(text)?))
    }

    pub fn to_bincode(&self) -> anyhow::Result<Vec<u8>> {
        use bincode::Options;
        // varint encoding: 1-byte lengths for the many short tokens, vs the
        // 8-byte fixint prefixes the bare `bincode::serialize` would emit
        Ok(bincode::DefaultOptions::new().serialize(&self.to_dto())?)
    }

    pub fn from_bincode(bytes: &[u8]) -> anyhow::Result<Self> {
        use bincode::Options;
        Ok(Self::from_dto(bincode::DefaultOptions::new().deserialize(bytes)?))
    }
}

impl Predictor for MarkovModel {
    fn predict(&self, path: &str, top_n: usize) -> Vec<(String, f64)> {
        let recent = path_segments(path);
        let mut out = self.predict_tokens(&recent, top_n);
        // E8 subword back-off: if there's no context-specific evidence for the last
        // segment (no prediction, or we had to back off below the full context),
        // offer learned tokens that share subwords with it.
        let backed_off = self
            .best_context(&recent)
            .map(|c| c.len())
            .unwrap_or(0)
            < recent.len();
        if out.len() < top_n && (out.is_empty() || backed_off) {
            let have: std::collections::HashSet<String> =
                out.iter().map(|(t, _)| t.clone()).collect();
            for cand in self.subword_candidates(&recent, top_n) {
                if out.len() >= top_n {
                    break;
                }
                if !have.contains(&cand.0) {
                    out.push(cand);
                }
            }
        }
        out
    }

    fn learn(&mut self, path: &str) {
        self.learn_path(path);
    }
}

impl crate::interfaces::PathModel for MarkovModel {
    fn save_json(&self) -> anyhow::Result<String> {
        self.to_json()
    }

    fn merge_json(&mut self, text: &str) -> anyhow::Result<()> {
        self.merge(&MarkovModel::from_json(text)?);
        Ok(())
    }

    fn save_bytes(&self) -> anyhow::Result<Vec<u8>> {
        self.to_bincode()
    }

    fn merge_bytes(&mut self, bytes: &[u8]) -> anyhow::Result<()> {
        self.merge(&MarkovModel::from_bincode(bytes)?);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seed_rows_are_normalizable_to_one() {
        for profile in crate::profiles::PROFILES {
            let m = MarkovModel::seeded(profile, 3, 0.5, 0.0);
            // every seeded context's MLE distribution sums to 1.0
            for (from, _) in profiles::seed_matrix(profile) {
                let ctx: Vec<String> = if from.is_empty() {
                    vec![]
                } else {
                    vec![from.to_string()]
                };
                let dist = m.distribution(&ctx);
                let sum: f64 = dist.iter().map(|(_, p)| p).sum();
                assert!((sum - 1.0).abs() < 1e-9, "{profile} row {from:?} summed to {sum}");
            }
        }
    }

    #[test]
    fn rest_api_predicts_v1_after_api() {
        let m = MarkovModel::seeded("REST_API", 3, 0.5, 0.0);
        let preds = m.predict("https://x.test/api", 3);
        assert_eq!(preds[0].0, "v1", "expected v1 top, got {preds:?}");
    }

    #[test]
    fn ppm_backs_off_to_shorter_context() {
        let mut m = MarkovModel::new(3, 0.5, 0.0);
        m.learn_path("/api/v1/users");
        // context ["api","v1"] is known; prediction after it yields "users"
        let preds = m.predict_tokens(&["api".into(), "v1".into()], 5);
        assert!(preds.iter().any(|(t, _)| t == "users"));
        // unknown deep context backs off and still returns something sane
        let preds2 = m.predict_tokens(&["zzz".into(), "v1".into()], 5);
        assert!(preds2.iter().any(|(t, _)| t == "users"));
    }

    #[test]
    fn online_learning_updates_rows() {
        let mut m = MarkovModel::new(2, 0.5, 0.0);
        m.learn("/shop/checkout");
        let preds = m.predict("/shop", 3);
        assert!(preds.iter().any(|(t, _)| t == "checkout"));
    }

    #[test]
    fn json_round_trip() {
        let m = MarkovModel::seeded("WORDPRESS_CMS", 3, 0.5, 0.01);
        let js = m.to_json().unwrap();
        let back = MarkovModel::from_json(&js).unwrap();
        assert_eq!(
            m.predict("/wp-content", 5),
            back.predict("/wp-content", 5)
        );
    }

    #[test]
    fn bincode_round_trip_matches_json_and_beats_pretty() {
        let m = MarkovModel::seeded("WORDPRESS_CMS", 3, 0.5, 0.01);
        let bytes = m.to_bincode().unwrap();
        let back = MarkovModel::from_bincode(&bytes).unwrap();
        assert_eq!(m.predict("/wp-content", 5), back.predict("/wp-content", 5));
        // the varint binary encoding is smaller than the pretty JSON it replaces
        // (on the real 24 MB model: ~7.8 MB vs ~24 MB; also far faster to load)
        let pretty = serde_json::to_string_pretty(&m.to_dto()).unwrap().len();
        assert!(
            bytes.len() < pretty,
            "bincode ({}) should beat pretty json ({})",
            bytes.len(),
            pretty
        );
    }

    #[test]
    fn save_bytes_then_merge_bytes_preserves_predictions() {
        use crate::interfaces::PathModel;
        let m = MarkovModel::seeded("WORDPRESS_CMS", 3, 0.5, 0.01);
        let bytes = m.save_bytes().unwrap();
        let mut empty = MarkovModel::new(3, 0.5, 0.01);
        empty.merge_bytes(&bytes).unwrap();
        assert_eq!(m.predict("/wp-content", 5), empty.predict("/wp-content", 5));
    }
}
