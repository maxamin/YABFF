//! Frequency prefix-trie — the classic autocomplete tree, as a baseline.
//!
//! A plain trie over URL path **segments**: each node is a prefix with an
//! observation `score`. Top-k completions are found by walking to the prefix
//! locus and **scanning the whole subtree**, then sorting — `O(m + c log c)` for a
//! subtree of `c` completions. It stores exactly the same counts as the
//! [`DynSdt`](crate::dynsdt::DynSdt) but without the score-decomposition, so it is
//! the honest "naive trie" reference point in the algorithm comparison: identical
//! predictions, higher query cost (it touches every descendant instead of just
//! the `k` best).

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::interfaces::{PathModel, Predictor};
use crate::tokenize::path_segments;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct Node {
    score: f64,
    children: HashMap<String, usize>,
}

/// A frequency-scored prefix trie over path segments.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FreqTrie {
    nodes: Vec<Node>, // arena; nodes[0] = root (empty prefix)
}

impl Default for FreqTrie {
    fn default() -> Self {
        Self::new()
    }
}

impl FreqTrie {
    pub fn new() -> Self {
        Self {
            nodes: vec![Node::default()],
        }
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
        let mut cur = 0usize;
        for seg in segments {
            cur = match self.nodes[cur].children.get(seg) {
                Some(&id) => id,
                None => {
                    let id = self.nodes.len();
                    self.nodes.push(Node::default());
                    self.nodes[cur].children.insert(seg.clone(), id);
                    id
                }
            };
        }
        self.nodes[cur].score += weight;
    }

    /// Top-k highest-scoring completions of `prefix` (slash-joined suffixes).
    pub fn top_k(&self, prefix: &[String], k: usize) -> Vec<(String, f64)> {
        if k == 0 {
            return vec![];
        }
        let mut cur = 0usize;
        for seg in prefix {
            match self.nodes[cur].children.get(seg) {
                Some(&id) => cur = id,
                None => return vec![],
            }
        }
        // scan the whole subtree below the locus, collecting scored completions
        let mut found: Vec<(String, f64)> = Vec::new();
        let mut stack: Vec<(usize, String)> = Vec::new();
        for (seg, &c) in &self.nodes[cur].children {
            stack.push((c, seg.clone()));
        }
        while let Some((id, suffix)) = stack.pop() {
            if self.nodes[id].score > 0.0 {
                found.push((suffix.clone(), self.nodes[id].score));
            }
            for (seg, &c) in &self.nodes[id].children {
                stack.push((c, format!("{}/{}", suffix, seg)));
            }
        }
        // sort by score desc, then suffix asc for determinism
        found.sort_by(|a, b| b.1.total_cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
        found.truncate(k);
        found
    }

    pub fn merge(&mut self, other: &FreqTrie) {
        let mut stack = vec![(0usize, Vec::<String>::new())];
        while let Some((id, segs)) = stack.pop() {
            if other.nodes[id].score > 0.0 && !segs.is_empty() {
                self.add_weighted(&segs, other.nodes[id].score);
            }
            for (seg, &c) in &other.nodes[id].children {
                let mut s = segs.clone();
                s.push(seg.clone());
                stack.push((c, s));
            }
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

impl Predictor for FreqTrie {
    fn predict(&self, path: &str, top_n: usize) -> Vec<(String, f64)> {
        self.top_k(&path_segments(path), top_n)
    }

    fn learn(&mut self, path: &str) {
        self.observe(path);
    }
}

impl PathModel for FreqTrie {
    fn save_json(&self) -> anyhow::Result<String> {
        self.to_json()
    }

    fn merge_json(&mut self, text: &str) -> anyhow::Result<()> {
        self.merge(&FreqTrie::from_json(text)?);
        Ok(())
    }

    fn save_bytes(&self) -> anyhow::Result<Vec<u8>> {
        self.to_bincode()
    }

    fn merge_bytes(&mut self, bytes: &[u8]) -> anyhow::Result<()> {
        self.merge(&FreqTrie::from_bincode(bytes)?);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn top_k_is_score_ordered() {
        let mut t = FreqTrie::new();
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
    fn merge_and_json_roundtrip() {
        let mut a = FreqTrie::new();
        a.observe("https://x/api/v1");
        let mut b = FreqTrie::new();
        b.observe("https://x/api/v2");
        b.observe("https://x/api/v2");
        a.merge(&b);
        assert_eq!(a.top_k(&["api".to_string()], 1), vec![("v2".to_string(), 2.0)]);
        let back = FreqTrie::from_json(&a.to_json().unwrap()).unwrap();
        assert_eq!(a.top_k(&["api".to_string()], 2), back.top_k(&["api".to_string()], 2));
    }

    #[test]
    fn unknown_prefix_is_empty() {
        let mut t = FreqTrie::new();
        t.observe("https://x/api/v1");
        assert!(t.top_k(&["nope".to_string()], 5).is_empty());
    }
}
