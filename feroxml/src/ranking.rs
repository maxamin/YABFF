//! BM25 ranking of candidate path tokens against the vocabulary observed on the
//! target so far. Candidates whose subwords occur in the discovered corpus are
//! ranked higher, so the request budget is spent on the most target-relevant
//! guesses first. `ranker = "none"` keeps the predictor's own ordering.

use std::collections::HashMap;

use crate::tokenize::subword_tokens;

const K1: f64 = 1.5;
const B: f64 = 0.75;

/// A BM25 model over the corpus of path segments discovered so far. Each
/// discovered segment is a "document" of its subwords.
pub struct Bm25 {
    docs: Vec<Vec<String>>,
    df: HashMap<String, usize>,
    avg_len: f64,
}

impl Bm25 {
    pub fn new() -> Self {
        Self {
            docs: Vec::new(),
            df: HashMap::new(),
            avg_len: 0.0,
        }
    }

    /// Add an observed path segment to the corpus.
    pub fn add_document(&mut self, segment: &str) {
        let toks = subword_tokens(segment);
        if toks.is_empty() {
            return;
        }
        let mut seen: HashMap<&str, ()> = HashMap::new();
        for t in &toks {
            if seen.insert(t.as_str(), ()).is_none() {
                *self.df.entry(t.clone()).or_insert(0) += 1;
            }
        }
        self.docs.push(toks);
        let total: usize = self.docs.iter().map(|d| d.len()).sum();
        self.avg_len = total as f64 / self.docs.len() as f64;
    }

    /// Affinity weight of a term = how present it is in the observed corpus.
    /// Unlike classic IDF (which rewards *rarity*), we want to reward *presence*
    /// in the target's vocabulary, so a candidate that shares tokens with paths
    /// already seen ranks higher. An unseen term contributes zero.
    fn affinity(&self, term: &str) -> f64 {
        let df = *self.df.get(term).unwrap_or(&0) as f64;
        (1.0 + df).ln()
    }

    /// Corpus-affinity score (BM25/TF-IDF family) of a candidate: the summed
    /// affinity of its subwords, with BM25 term-frequency saturation so a
    /// repeated subword doesn't dominate. `0.0` when nothing matches the corpus.
    pub fn score(&self, candidate: &str) -> f64 {
        if self.docs.is_empty() {
            return 0.0;
        }
        let q = subword_tokens(candidate);
        let dl = q.len() as f64;
        let mut score = 0.0;
        // term frequency within the candidate itself
        let mut tf: HashMap<&str, f64> = HashMap::new();
        for t in &q {
            *tf.entry(t.as_str()).or_insert(0.0) += 1.0;
        }
        for (term, f) in tf {
            let w = self.affinity(term);
            if w <= 0.0 {
                continue;
            }
            let norm = f * (K1 + 1.0)
                / (f + K1 * (1.0 - B + B * dl / self.avg_len.max(1.0)));
            score += w * norm;
        }
        score
    }

    /// Re-rank `(token, prior)` predictions by `prior * (1 + bm25)`, so target
    /// relevance boosts but never fully overrides the predictor's probability.
    pub fn rerank(&self, preds: &[(String, f64)]) -> Vec<(String, f64)> {
        let mut out: Vec<(String, f64)> = preds
            .iter()
            .map(|(tok, prior)| (tok.clone(), prior * (1.0 + self.score(tok))))
            .collect();
        out.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap().then(a.0.cmp(&b.0)));
        out
    }
}

impl Default for Bm25 {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_vocabulary_ranks_higher() {
        let mut bm = Bm25::new();
        for seg in ["users", "user-profile", "user-settings", "orders"] {
            bm.add_document(seg);
        }
        // "user" appears across several docs; "zzz" nowhere
        assert!(bm.score("user") > bm.score("zzz"));
    }

    #[test]
    fn rerank_promotes_relevant_candidate() {
        let mut bm = Bm25::new();
        for seg in ["api", "api-users", "api-auth"] {
            bm.add_document(seg);
        }
        let preds = vec![("random".to_string(), 0.30), ("users".to_string(), 0.25)];
        let ranked = bm.rerank(&preds);
        // "users" shares subwords with the corpus and should climb
        assert_eq!(ranked[0].0, "users", "ranked: {ranked:?}");
    }

    #[test]
    fn empty_corpus_is_safe() {
        let bm = Bm25::new();
        assert_eq!(bm.score("anything"), 0.0);
    }
}
