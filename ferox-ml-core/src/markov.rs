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
use crate::tokenize::path_segments;

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

    pub fn to_json(&self) -> anyhow::Result<String> {
        let dto = MarkovDto {
            rows: self
                .rows
                .iter()
                .map(|(k, v)| (k.clone(), v.iter().map(|(t, w)| (t.clone(), *w)).collect()))
                .collect(),
            max_order: self.max_order,
            alpha: self.alpha,
            threshold: self.threshold,
        };
        Ok(serde_json::to_string_pretty(&dto)?)
    }

    pub fn from_json(text: &str) -> anyhow::Result<Self> {
        let dto: MarkovDto = serde_json::from_str(text)?;
        let mut rows: HashMap<Vec<String>, HashMap<String, f64>> = HashMap::new();
        for (ctx, tos) in dto.rows {
            rows.insert(ctx, tos.into_iter().collect());
        }
        Ok(Self {
            rows,
            max_order: dto.max_order,
            alpha: dto.alpha,
            threshold: dto.threshold,
        })
    }
}

impl Predictor for MarkovModel {
    fn predict(&self, path: &str, top_n: usize) -> Vec<(String, f64)> {
        let recent = path_segments(path);
        self.predict_tokens(&recent, top_n)
    }

    fn learn(&mut self, path: &str) {
        self.learn_path(path);
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
}
