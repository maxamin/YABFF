//! Dynamic Score-Decomposed Trie (DynSDT) for top-k path autocomplete.
//!
//! Adapted from Validark's DynSDT (<https://validark.dev/DynSDT/>) to our domain:
//! the "characters" are URL path **segments**, and the model stores the observed
//! directory-tree structure. Each node is a path prefix with its own observation
//! `score` (how often that exact path was a hit) and `subtree_max` (the best score
//! anywhere below it) — the *score decomposition*. Each node keeps its children
//! **sorted by `subtree_max` descending** (the horizontal heap property), so a
//! top-k query is a best-first walk that only follows the best child and the next
//! sibling (the "binary max-heap drawn with right angles") instead of scanning
//! every child. It is **dynamic**: `observe` bumps a terminal's score and re-sorts
//! the affected nodes up the path online, so repeated runs accumulate — more data
//! => better predictions — and it serializes to JSON for persistence across runs.

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

use serde::{Deserialize, Serialize};

use crate::interfaces::{PathModel, Predictor};
use crate::tokenize::path_segments;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct Node {
    seg: String,
    score: f64,
    subtree_max: f64,
    /// segment -> child node id (for insertion lookup)
    child_ids: HashMap<String, usize>,
    /// child node ids sorted by `subtree_max` descending (for top-k navigation)
    order: Vec<usize>,
}

/// A dynamic score-decomposed trie over path segments.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynSdt {
    nodes: Vec<Node>, // arena; nodes[0] is the root (empty prefix)
}

impl Default for DynSdt {
    fn default() -> Self {
        Self::new()
    }
}

impl DynSdt {
    pub fn new() -> Self {
        Self {
            nodes: vec![Node::default()],
        }
    }

    /// Number of trie nodes (distinct prefixes), for reporting.
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.nodes[0].order.is_empty()
    }

    /// Observe a path once (weight 1.0).
    pub fn observe(&mut self, path: &str) {
        self.add_weighted(&path_segments(path), 1.0);
    }

    /// Add `weight` to the terminal node for `segments`, creating nodes as needed,
    /// then re-establish `subtree_max` and the horizontal (child) sort up the path.
    pub fn add_weighted(&mut self, segments: &[String], weight: f64) {
        if segments.is_empty() || weight <= 0.0 {
            return;
        }
        let mut cur = 0usize;
        let mut path = Vec::with_capacity(segments.len() + 1);
        path.push(cur);
        for seg in segments {
            cur = match self.nodes[cur].child_ids.get(seg) {
                Some(&id) => id,
                None => {
                    let id = self.nodes.len();
                    let mut node = Node::default();
                    node.seg = seg.clone();
                    self.nodes.push(node);
                    self.nodes[cur].child_ids.insert(seg.clone(), id);
                    self.nodes[cur].order.push(id);
                    id
                }
            };
            path.push(cur);
        }
        self.nodes[cur].score += weight;

        // bottom-up: recompute subtree_max, then re-sort each parent's child order
        for i in (0..path.len()).rev() {
            let id = path[i];
            let mut m = self.nodes[id].score;
            for &c in &self.nodes[id].order {
                if self.nodes[c].subtree_max > m {
                    m = self.nodes[c].subtree_max;
                }
            }
            self.nodes[id].subtree_max = m;
            if i > 0 {
                self.sort_children(path[i - 1]);
            }
        }
    }

    /// Keep a node's children sorted by `subtree_max` desc (ties by id for
    /// determinism) — the DynSDT horizontal heap property.
    fn sort_children(&mut self, parent: usize) {
        let mut order = std::mem::take(&mut self.nodes[parent].order);
        order.sort_by(|&a, &b| {
            self.nodes[b]
                .subtree_max
                .total_cmp(&self.nodes[a].subtree_max)
                .then(a.cmp(&b))
        });
        self.nodes[parent].order = order;
    }

    /// Top-k highest-scoring completions of `prefix` (slash-joined suffixes), in
    /// descending score order. Best-first over the score-decomposed structure.
    pub fn top_k(&self, prefix: &[String], k: usize) -> Vec<(String, f64)> {
        if k == 0 {
            return vec![];
        }
        let mut cur = 0usize;
        for seg in prefix {
            match self.nodes[cur].child_ids.get(seg) {
                Some(&id) => cur = id,
                None => return vec![],
            }
        }

        let mut heap: BinaryHeap<Item> = BinaryHeap::new();
        let mut seq: u64 = 0;
        let mut push = |heap: &mut BinaryHeap<Item>, key: f64, kind: Kind| {
            heap.push(Item { key, seq, kind });
            seq += 1;
        };

        if let Some(&first) = self.nodes[cur].order.first() {
            push(&mut heap, self.nodes[first].subtree_max, Kind::Branch { parent: cur, idx: 0, base: String::new() });
        }

        let mut out = Vec::with_capacity(k);
        while let Some(item) = heap.pop() {
            match item.kind {
                Kind::Emit { suffix, score } => {
                    out.push((suffix, score));
                    if out.len() >= k {
                        break;
                    }
                }
                Kind::Branch { parent, idx, base } => {
                    let node = self.nodes[parent].order[idx];
                    let suffix = if base.is_empty() {
                        self.nodes[node].seg.clone()
                    } else {
                        format!("{}/{}", base, self.nodes[node].seg)
                    };
                    // horizontal: the next sibling (already sorted by subtree_max)
                    if idx + 1 < self.nodes[parent].order.len() {
                        let sib = self.nodes[parent].order[idx + 1];
                        push(&mut heap, self.nodes[sib].subtree_max, Kind::Branch { parent, idx: idx + 1, base });
                    }
                    // this node's own completion, ordered by its own score
                    let score = self.nodes[node].score;
                    if score > 0.0 {
                        push(&mut heap, score, Kind::Emit { suffix: suffix.clone(), score });
                    }
                    // vertical: this node's best child
                    if let Some(&fc) = self.nodes[node].order.first() {
                        push(&mut heap, self.nodes[fc].subtree_max, Kind::Branch { parent: node, idx: 0, base: suffix });
                    }
                }
            }
        }
        out
    }

    /// Fold another trie's observations into this one (cross-run accumulation).
    pub fn merge(&mut self, other: &DynSdt) {
        for (segs, w) in other.terminals() {
            self.add_weighted(&segs, w);
        }
    }

    /// All (segments, score) for terminal nodes (score > 0).
    fn terminals(&self) -> Vec<(Vec<String>, f64)> {
        let mut out = Vec::new();
        let mut stack = vec![(0usize, Vec::<String>::new())];
        while let Some((id, segs)) = stack.pop() {
            if self.nodes[id].score > 0.0 && !segs.is_empty() {
                out.push((segs.clone(), self.nodes[id].score));
            }
            for (seg, &c) in &self.nodes[id].child_ids {
                let mut s = segs.clone();
                s.push(seg.clone());
                stack.push((c, s));
            }
        }
        out
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

enum Kind {
    /// explore the `idx`-th child of `parent` (siblings sorted by subtree_max)
    Branch { parent: usize, idx: usize, base: String },
    /// emit this node's own completion
    Emit { suffix: String, score: f64 },
}

struct Item {
    key: f64,
    seq: u64,
    kind: Kind,
}

impl PartialEq for Item {
    fn eq(&self, o: &Self) -> bool {
        self.cmp(o) == Ordering::Equal
    }
}
impl Eq for Item {}
impl PartialOrd for Item {
    fn partial_cmp(&self, o: &Self) -> Option<Ordering> {
        Some(self.cmp(o))
    }
}
impl Ord for Item {
    fn cmp(&self, o: &Self) -> Ordering {
        // max-heap by key; ties resolved FIFO (lower seq pops first) for determinism
        self.key.total_cmp(&o.key).then_with(|| o.seq.cmp(&self.seq))
    }
}

impl Predictor for DynSdt {
    fn predict(&self, path: &str, top_n: usize) -> Vec<(String, f64)> {
        self.top_k(&path_segments(path), top_n)
    }

    fn learn(&mut self, path: &str) {
        self.observe(path);
    }
}

impl PathModel for DynSdt {
    fn save_json(&self) -> anyhow::Result<String> {
        self.to_json()
    }

    fn merge_json(&mut self, text: &str) -> anyhow::Result<()> {
        self.merge(&DynSdt::from_json(text)?);
        Ok(())
    }

    fn save_bytes(&self) -> anyhow::Result<Vec<u8>> {
        self.to_bincode()
    }

    fn merge_bytes(&mut self, bytes: &[u8]) -> anyhow::Result<()> {
        self.merge(&DynSdt::from_bincode(bytes)?);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn top_k_returns_completions_in_score_order() {
        let mut t = DynSdt::new();
        t.observe("https://x/api/v1");
        t.observe("https://x/api/v1"); // score 2
        t.observe("https://x/api/v1/users");
        t.observe("https://x/api/v2");

        let top = t.top_k(&["api".to_string()], 3);
        assert_eq!(top[0], ("v1".to_string(), 2.0)); // strictly highest
        // the other two are the score-1 completions, in any tie order
        let rest: std::collections::HashSet<(String, u64)> =
            top[1..].iter().map(|(s, v)| (s.clone(), *v as u64)).collect();
        assert_eq!(
            rest,
            [("v2".to_string(), 1), ("v1/users".to_string(), 1)].into_iter().collect()
        );
    }

    #[test]
    fn dynamic_updates_reorder_predictions() {
        let mut t = DynSdt::new();
        t.observe("https://x/a/one");
        t.observe("https://x/a/two");
        // tie -> both present
        assert_eq!(t.top_k(&["a".to_string()], 2).len(), 2);
        // make "two" clearly hotter; it must now lead
        for _ in 0..5 {
            t.observe("https://x/a/two");
        }
        assert_eq!(t.top_k(&["a".to_string()], 1), vec![("two".to_string(), 6.0)]);
    }

    #[test]
    fn deeper_completion_surfaces_when_hottest() {
        let mut t = DynSdt::new();
        for _ in 0..3 {
            t.observe("https://x/shop/cart/items"); // deep path, score 3
        }
        t.observe("https://x/shop/about"); // shallow, score 1
        let top = t.top_k(&["shop".to_string()], 1);
        assert_eq!(top, vec![("cart/items".to_string(), 3.0)]);
    }

    #[test]
    fn merge_and_json_roundtrip_preserve_predictions() {
        let mut a = DynSdt::new();
        a.observe("https://x/api/v1");
        let mut b = DynSdt::new();
        b.observe("https://x/api/v2");
        b.observe("https://x/api/v2"); // score 2
        a.merge(&b);
        let top = a.top_k(&["api".to_string()], 2);
        assert_eq!(top[0], ("v2".to_string(), 2.0));

        let json = a.to_json().unwrap();
        let back = DynSdt::from_json(&json).unwrap();
        assert_eq!(a.top_k(&["api".to_string()], 2), back.top_k(&["api".to_string()], 2));
        assert_eq!(a.node_count(), back.node_count());
    }

    #[test]
    fn unknown_prefix_is_empty_and_predictor_trait_works() {
        let mut t = DynSdt::new();
        t.observe("https://x/api/v1/users");
        assert!(t.top_k(&["nope".to_string()], 5).is_empty());
        // Predictor impl: learn + predict
        let preds = t.predict("https://x/api/v1", 5);
        assert!(preds.iter().any(|(s, _)| s == "users"));
    }
}
