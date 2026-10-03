//! Phase 3 — which discovered directory ("arm") to expand next.
//!
//! * [`ThompsonScheduler`] (default): each arm keeps a Beta(α, β) posterior over
//!   its hit-rate; `choose` samples every arm and picks the highest draw. This
//!   automatically shifts the request budget toward productive branches.
//! * [`Ucb1Scheduler`]: deterministic upper-confidence-bound alternative.
//! * [`RoundRobinScheduler`]: naive baseline.
//!
//! All three implement [`Scheduler`]; the config's `scheduler` field selects one.

use std::collections::HashMap;

use crate::interfaces::Scheduler;
use crate::rng::Rng;

// ----------------------------- Thompson -----------------------------

struct Beta {
    alpha: f64,
    beta: f64,
    exhausted: bool,
}

pub struct ThompsonScheduler {
    arms: HashMap<String, Beta>,
    order: Vec<String>, // insertion order, for stable tie-breaking
    rng: Rng,
    /// Non-stationarity discount in `(0, 1]`. Before each update the evidence an
    /// arm has accumulated (its Beta mass above the uniform prior) is scaled by
    /// this factor, so old rewards fade and the posterior tracks the arm's
    /// *recent* yield. `1.0` disables decay (stationary Beta — the classic
    /// Thompson sampler), which is the default and keeps historical behavior.
    decay: f64,
}

impl ThompsonScheduler {
    pub fn new(seed: u64) -> Self {
        Self::with_decay(seed, 1.0)
    }

    /// Thompson sampler with a non-stationarity `decay` in `(0, 1]` (see the
    /// [`decay`](ThompsonScheduler::decay) field). Values outside the range are
    /// clamped; `1.0` is the stationary default.
    pub fn with_decay(seed: u64, decay: f64) -> Self {
        Self {
            arms: HashMap::new(),
            order: Vec::new(),
            rng: Rng::new(seed),
            decay: decay.clamp(f64::MIN_POSITIVE, 1.0),
        }
    }

    /// Mark an arm spent so it is never chosen again (e.g. depth/budget cap).
    pub fn retire(&mut self, arm: &str) {
        if let Some(b) = self.arms.get_mut(arm) {
            b.exhausted = true;
        }
    }
}

impl Scheduler for ThompsonScheduler {
    fn add_arm(&mut self, arm: &str) {
        if !self.arms.contains_key(arm) {
            self.arms.insert(
                arm.to_string(),
                Beta {
                    alpha: 1.0,
                    beta: 1.0,
                    exhausted: false,
                },
            );
            self.order.push(arm.to_string());
        }
    }

    fn update(&mut self, arm: &str, reward: f64) {
        let r = reward.clamp(0.0, 1.0);
        let decay = self.decay;
        if let Some(b) = self.arms.get_mut(arm) {
            // Discount prior evidence toward the uniform Beta(1,1) prior before
            // folding in the new reward, so a stale "productive" arm decays once
            // it stops paying off. decay == 1.0 => no discount (classic Thompson).
            if decay < 1.0 {
                b.alpha = 1.0 + (b.alpha - 1.0) * decay;
                b.beta = 1.0 + (b.beta - 1.0) * decay;
            }
            b.alpha += r;
            b.beta += 1.0 - r;
        }
    }

    fn choose(&mut self) -> Option<String> {
        let mut best: Option<(String, f64)> = None;
        for name in &self.order {
            let b = &self.arms[name];
            if b.exhausted {
                continue;
            }
            let sample = self.rng.beta(b.alpha, b.beta);
            match &best {
                Some((_, s)) if *s >= sample => {}
                _ => best = Some((name.clone(), sample)),
            }
        }
        let chosen = best.map(|(n, _)| n)?;
        // one expansion per selection: retire so the loop always makes progress
        self.retire(&chosen);
        Some(chosen)
    }

    fn value(&self, arm: &str) -> f64 {
        match self.arms.get(arm) {
            // posterior mean of the Beta(alpha, beta) hit-rate estimate
            Some(b) => b.alpha / (b.alpha + b.beta),
            None => 0.5,
        }
    }
}

// ------------------------------- UCB1 -------------------------------

struct UcbArm {
    reward_sum: f64,
    pulls: f64,
    exhausted: bool,
}

pub struct Ucb1Scheduler {
    arms: HashMap<String, UcbArm>,
    order: Vec<String>,
    total_pulls: f64,
}

impl Ucb1Scheduler {
    pub fn new() -> Self {
        Self {
            arms: HashMap::new(),
            order: Vec::new(),
            total_pulls: 0.0,
        }
    }
}

impl Default for Ucb1Scheduler {
    fn default() -> Self {
        Self::new()
    }
}

impl Scheduler for Ucb1Scheduler {
    fn add_arm(&mut self, arm: &str) {
        if !self.arms.contains_key(arm) {
            self.arms.insert(
                arm.to_string(),
                UcbArm {
                    reward_sum: 0.0,
                    pulls: 0.0,
                    exhausted: false,
                },
            );
            self.order.push(arm.to_string());
        }
    }

    fn update(&mut self, arm: &str, reward: f64) {
        let r = reward.clamp(0.0, 1.0);
        if let Some(a) = self.arms.get_mut(arm) {
            a.reward_sum += r;
            a.pulls += 1.0;
            self.total_pulls += 1.0;
        }
    }

    fn choose(&mut self) -> Option<String> {
        let t = self.total_pulls.max(1.0);
        let mut best: Option<(String, f64)> = None;
        for name in &self.order {
            let a = &self.arms[name];
            if a.exhausted {
                continue;
            }
            let score = if a.pulls == 0.0 {
                f64::INFINITY // explore every arm at least once
            } else {
                a.reward_sum / a.pulls + (2.0 * t.ln() / a.pulls).sqrt()
            };
            match &best {
                Some((_, s)) if *s >= score => {}
                _ => best = Some((name.clone(), score)),
            }
        }
        let chosen = best.map(|(n, _)| n)?;
        if let Some(a) = self.arms.get_mut(&chosen) {
            a.exhausted = true;
        }
        Some(chosen)
    }

    fn value(&self, arm: &str) -> f64 {
        match self.arms.get(arm) {
            // mean observed reward so far; unpulled arms are neutral
            Some(a) if a.pulls > 0.0 => (a.reward_sum / a.pulls).clamp(0.0, 1.0),
            _ => 0.5,
        }
    }
}

// ---------------------------- Round robin ----------------------------

pub struct RoundRobinScheduler {
    queue: Vec<String>,
    seen: HashMap<String, ()>,
    idx: usize,
}

impl RoundRobinScheduler {
    pub fn new() -> Self {
        Self {
            queue: Vec::new(),
            seen: HashMap::new(),
            idx: 0,
        }
    }
}

impl Default for RoundRobinScheduler {
    fn default() -> Self {
        Self::new()
    }
}

impl Scheduler for RoundRobinScheduler {
    fn add_arm(&mut self, arm: &str) {
        if self.seen.insert(arm.to_string(), ()).is_none() {
            self.queue.push(arm.to_string());
        }
    }

    fn update(&mut self, _arm: &str, _reward: f64) {}

    fn choose(&mut self) -> Option<String> {
        if self.idx >= self.queue.len() {
            return None;
        }
        let item = self.queue[self.idx].clone();
        self.idx += 1;
        Some(item)
    }
}

/// Build the scheduler named in the config (stationary Thompson — no decay).
pub fn build(name: &str, seed: u64) -> Box<dyn Scheduler + Send + Sync> {
    build_decayed(name, seed, 1.0)
}

/// Build the scheduler named in the config, applying a non-stationarity `decay`
/// in `(0, 1]` to the Thompson sampler (ignored by UCB1 / round-robin, which have
/// no Beta posterior to discount). `decay == 1.0` is the stationary default and is
/// what [`build`] uses.
pub fn build_decayed(name: &str, seed: u64, decay: f64) -> Box<dyn Scheduler + Send + Sync> {
    match name {
        "ucb1" => Box::new(Ucb1Scheduler::new()),
        "round_robin" => Box::new(RoundRobinScheduler::new()),
        _ => Box::new(ThompsonScheduler::with_decay(seed, decay)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thompson_favors_the_productive_arm() {
        let mut s = ThompsonScheduler::new(1);
        s.add_arm("good");
        s.add_arm("bad");
        // reward history: good hits often, bad rarely
        for _ in 0..40 {
            s.update("good", 0.9);
            s.update("bad", 0.05);
        }
        // over many fresh selections, "good" should dominate
        let mut good = 0;
        for _ in 0..200 {
            let mut t = ThompsonScheduler::new(good as u64 + 1);
            t.arms.insert(
                "good".into(),
                Beta { alpha: 37.0, beta: 5.0, exhausted: false },
            );
            t.arms.insert(
                "bad".into(),
                Beta { alpha: 3.0, beta: 39.0, exhausted: false },
            );
            t.order = vec!["good".into(), "bad".into()];
            if t.choose().as_deref() == Some("good") {
                good += 1;
            }
        }
        assert!(good > 180, "good chosen {good}/200 times");
    }

    #[test]
    fn decay_lets_the_posterior_track_a_regime_change() {
        // An arm that paid off early then goes cold. With decay, its value
        // estimate should collapse toward the recent (zero) yield; without decay
        // the long productive history keeps it high.
        let warm = || {
            let mut s = ThompsonScheduler::with_decay(1, 0.5);
            for _ in 0..20 {
                s.add_arm("a");
                s.update("a", 1.0);
            }
            s
        };
        let stationary = || {
            let mut s = ThompsonScheduler::new(1);
            for _ in 0..20 {
                s.add_arm("a");
                s.update("a", 1.0);
            }
            s
        };

        let mut decayed = warm();
        let mut flat = stationary();
        // regime change: the arm now returns nothing for a while
        for _ in 0..10 {
            decayed.update("a", 0.0);
            flat.update("a", 0.0);
        }

        // the decayed estimate has forgotten the stale wins and dropped much lower
        assert!(
            decayed.value("a") < flat.value("a") - 0.2,
            "decayed={} flat={}",
            decayed.value("a"),
            flat.value("a")
        );
        assert!(decayed.value("a") < 0.35, "decayed should track recent cold streak: {}", decayed.value("a"));
    }

    #[test]
    fn decay_defaults_to_stationary() {
        // build() and ThompsonScheduler::new() must be the classic (undiscounted)
        // sampler, so default behavior is unchanged.
        let mut a = ThompsonScheduler::new(7);
        let mut b = ThompsonScheduler::with_decay(7, 1.0);
        for _ in 0..5 {
            a.add_arm("x");
            b.add_arm("x");
            a.update("x", 1.0);
            b.update("x", 1.0);
        }
        assert_eq!(a.value("x"), b.value("x"));
    }

    #[test]
    fn ucb1_explores_then_exploits() {
        let mut s = Ucb1Scheduler::new();
        s.add_arm("a");
        s.add_arm("b");
        let first = s.choose(); // unpulled -> infinite score, explore
        assert!(first.is_some());
    }

    #[test]
    fn round_robin_visits_each_once() {
        let mut s = RoundRobinScheduler::new();
        s.add_arm("a");
        s.add_arm("b");
        s.add_arm("a"); // dup ignored
        assert_eq!(s.choose().as_deref(), Some("a"));
        assert_eq!(s.choose().as_deref(), Some("b"));
        assert_eq!(s.choose(), None);
    }
}
