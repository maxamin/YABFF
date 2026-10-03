//! Ternary Search Tree — the classic autocomplete structure.
//!
//! A TST stores a trie compactly: instead of a per-node child map, each node holds
//! one segment `key` and three links — `lo`/`hi` are the BST alternatives at the
//! same position, `eq` advances to the next segment. Here the "characters" are URL
//! path **segments**. Lookups are `O(m + log σ)` per level (σ = fan-out), trading
//! the trie's hashing for pointer-light BST navigation and better memory locality.
//! Like the other tries it is frequency-scored; top-k scans the completion subtree
//! and sorts, so its predictions match [`FreqTrie`](crate::freqtrie::FreqTrie) and
//! [`DynSdt`](crate::dynsdt::DynSdt) — it is included to compare tree *layouts*.

use serde::{Deserialize, Serialize};

use crate::interfaces::{PathModel, Predictor};
use crate::tokenize::path_segments;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Node {
    key: String,
    score: f64,
    lo: Option<usize>,
    eq: Option<usize>,
    hi: Option<usize>,
}

/// A frequency-scored ternary search tree over path segments.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Tst {
    nodes: Vec<Node>,
    root: Option<usize>,
}

impl Tst {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn observe(&mut self, path: &str) {
        self.add_weighted(&path_segments(path), 1.0);
    }

    pub fn add_weighted(&mut self, segments: &[String], weight: f64) {
        if segments.is_empty() || weight <= 0.0 {
            return;
        }
        self.root = self.insert(self.root, segments, 0, weight);
    }

    fn new_node(&mut self, key: String) -> usize {
        let id = self.nodes.len();
        self.nodes.push(Node {
            key,
            score: 0.0,
            lo: None,
            eq: None,
            hi: None,
        });
        id
    }

    fn insert(&mut self, node: Option<usize>, segs: &[String], i: usize, w: f64) -> Option<usize> {
        let n = match node {
            Some(n) => n,
            None => self.new_node(segs[i].clone()),
        };
        match segs[i].cmp(&self.nodes[n].key) {
            std::cmp::Ordering::Less => {
                let lo = self.nodes[n].lo;
                self.nodes[n].lo = self.insert(lo, segs, i, w);
            }
            std::cmp::Ordering::Greater => {
                let hi = self.nodes[n].hi;
                self.nodes[n].hi = self.insert(hi, segs, i, w);
            }
            std::cmp::Ordering::Equal => {
                if i + 1 == segs.len() {
                    self.nodes[n].score += w;
                } else {
                    let eq = self.nodes[n].eq;
                    self.nodes[n].eq = self.insert(eq, segs, i + 1, w);
                }
            }
        }
        Some(n)
    }

    /// Node that terminates `prefix` (its `eq` subtree holds all completions).
    fn locus(&self, prefix: &[String]) -> Option<usize> {
        let mut node = self.root;
        let mut i = 0;
        while let Some(n) = node {
            match prefix[i].cmp(&self.nodes[n].key) {
                std::cmp::Ordering::Less => node = self.nodes[n].lo,
                std::cmp::Ordering::Greater => node = self.nodes[n].hi,
                std::cmp::Ordering::Equal => {
                    i += 1;
                    if i == prefix.len() {
                        return Some(n);
                    }
                    node = self.nodes[n].eq;
                }
            }
        }
        None
    }

    fn collect(&self, node: Option<usize>, base: &[String], out: &mut Vec<(Vec<String>, f64)>) {
        let Some(n) = node else { return };
        self.collect(self.nodes[n].lo, base, out);
        let mut word = base.to_vec();
        word.push(self.nodes[n].key.clone());
        if self.nodes[n].score > 0.0 {
            out.push((word.clone(), self.nodes[n].score));
        }
        self.collect(self.nodes[n].eq, &word, out);
        self.collect(self.nodes[n].hi, base, out);
    }

    /// Top-k highest-scoring completions of `prefix` (slash-joined suffixes).
    pub fn top_k(&self, prefix: &[String], k: usize) -> Vec<(String, f64)> {
        if k == 0 {
            return vec![];
        }
        let locus = if prefix.is_empty() {
            self.root
        } else {
            match self.locus(prefix) {
                Some(n) => self.nodes[n].eq,
                None => return vec![],
            }
        };
        let mut found: Vec<(Vec<String>, f64)> = Vec::new();
        self.collect(locus, &[], &mut found);
        let mut found: Vec<(String, f64)> =
            found.into_iter().map(|(segs, s)| (segs.join("/"), s)).collect();
        found.sort_by(|a, b| b.1.total_cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
        found.truncate(k);
        found
    }

    fn terminals(&self) -> Vec<(Vec<String>, f64)> {
        let mut out = Vec::new();
        self.collect(self.root, &[], &mut out);
        out
    }

    pub fn merge(&mut self, other: &Tst) {
        for (segs, w) in other.terminals() {
            self.add_weighted(&segs, w);
        }
    }

    pub fn to_json(&self) -> anyhow::Result<String> {
        Ok(serde_json::to_string(self)?)
    }

    pub fn from_json(text: &str) -> anyhow::Result<Self> {
        Ok(serde_json::from_str(text)?)
    }

    pub fn to_bincode(&self) -> anyhow::Result<Vec<u8>> {
        use bincode::Options;
        Ok(bincode::DefaultOptions::new().serialize(self)?)
    }

    pub fn from_bincode(bytes: &[u8]) -> anyhow::Result<Self> {
        use bincode::Options;
        Ok(bincode::DefaultOptions::new().deserialize(bytes)?)
    }
}

impl Predictor for Tst {
    fn predict(&self, path: &str, top_n: usize) -> Vec<(String, f64)> {
        self.top_k(&path_segments(path), top_n)
    }

    fn learn(&mut self, path: &str) {
        self.observe(path);
    }
}

impl PathModel for Tst {
    fn save_json(&self) -> anyhow::Result<String> {
        self.to_json()
    }

    fn merge_json(&mut self, text: &str) -> anyhow::Result<()> {
        self.merge(&Tst::from_json(text)?);
        Ok(())
    }

    fn save_bytes(&self) -> anyhow::Result<Vec<u8>> {
        self.to_bincode()
    }

    fn merge_bytes(&mut self, bytes: &[u8]) -> anyhow::Result<()> {
        self.merge(&Tst::from_bincode(bytes)?);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn top_k_is_score_ordered() {
        let mut t = Tst::new();
        t.observe("https://x/api/v1");
        t.observe("https://x/api/v1");
        t.observe("https://x/api/v1/users");
        t.observe("https://x/api/v2");
        let top = t.top_k(&["api".to_string()], 3);
        assert_eq!(top[0], ("v1".to_string(), 2.0));
        let rest: std::collections::HashSet<(String, u64)> =
            top[1..].iter().map(|(s, v)| (s.clone(), *v as u64)).collect();
        assert_eq!(
            rest,
            [("v2".to_string(), 1), ("v1/users".to_string(), 1)].into_iter().collect()
        );
    }

    #[test]
    fn handles_many_siblings_via_bst_links() {
        let mut t = Tst::new();
        // insert out of order to exercise lo/hi balancing
        for seg in ["m", "d", "t", "a", "z", "k"] {
            t.observe(&format!("https://x/root/{seg}"));
        }
        t.observe("https://x/root/z"); // z now hottest
        let top = t.top_k(&["root".to_string()], 2);
        assert_eq!(top[0], ("z".to_string(), 2.0));
        assert_eq!(top.len(), 2);
    }

    #[test]
    fn merge_and_json_roundtrip() {
        let mut a = Tst::new();
        a.observe("https://x/api/v1");
        let mut b = Tst::new();
        b.observe("https://x/api/v2");
        b.observe("https://x/api/v2");
        a.merge(&b);
        assert_eq!(a.top_k(&["api".to_string()], 1), vec![("v2".to_string(), 2.0)]);
        let back = Tst::from_json(&a.to_json().unwrap()).unwrap();
        assert_eq!(a.top_k(&["api".to_string()], 2), back.top_k(&["api".to_string()], 2));
    }

    #[test]
    fn unknown_prefix_is_empty() {
        let mut t = Tst::new();
        t.observe("https://x/api/v1");
        assert!(t.top_k(&["nope".to_string()], 5).is_empty());
    }
}
