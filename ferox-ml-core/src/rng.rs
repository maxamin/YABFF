//! A tiny, deterministic PRNG plus Gamma/Beta samplers.
//!
//! Implemented by hand so the whole crate stays on a lean, well-known
//! dependency set and every run is reproducible from a single `u64` seed. The
//! generator is SplitMix64 (for seeding) feeding xoshiro256**; Gamma uses the
//! Marsaglia–Tsang method and Beta is derived from two Gamma draws.

/// Deterministic random source.
pub struct Rng {
    s: [u64; 4],
}

impl Rng {
    /// Seed the generator. The same seed always yields the same sequence.
    pub fn new(seed: u64) -> Self {
        let mut sm = seed;
        let mut next = || {
            sm = sm.wrapping_add(0x9E37_79B9_7F4A_7C15);
            let mut z = sm;
            z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
            z ^ (z >> 31)
        };
        Self {
            s: [next(), next(), next(), next()],
        }
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        let result = self.s[1].wrapping_mul(5).rotate_left(7).wrapping_mul(9);
        let t = self.s[1] << 17;
        self.s[2] ^= self.s[0];
        self.s[3] ^= self.s[1];
        self.s[1] ^= self.s[2];
        self.s[0] ^= self.s[3];
        self.s[2] ^= t;
        self.s[3] = self.s[3].rotate_left(45);
        result
    }

    /// Uniform `f64` in `[0, 1)`.
    #[inline]
    pub fn uniform(&mut self) -> f64 {
        // 53 bits of mantissa precision
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }

    /// Standard normal via Box–Muller.
    fn normal(&mut self) -> f64 {
        let u1 = (self.uniform()).max(1e-12);
        let u2 = self.uniform();
        (-2.0 * u1.ln()).sqrt() * (std::f64::consts::TAU * u2).cos()
    }

    /// Gamma(shape=k, scale=1) via Marsaglia–Tsang (valid for k > 0).
    pub fn gamma(&mut self, k: f64) -> f64 {
        if k < 1.0 {
            // boost: Gamma(k) = Gamma(k+1) * U^(1/k)
            let g = self.gamma(k + 1.0);
            let u = self.uniform().max(1e-12);
            return g * u.powf(1.0 / k);
        }
        let d = k - 1.0 / 3.0;
        let c = 1.0 / (9.0 * d).sqrt();
        loop {
            let x = self.normal();
            let v = (1.0 + c * x).powi(3);
            if v <= 0.0 {
                continue;
            }
            let u = self.uniform();
            if u < 1.0 - 0.0331 * x.powi(4)
                || u.ln() < 0.5 * x * x + d * (1.0 - v + v.ln())
            {
                return d * v;
            }
        }
    }

    /// Beta(a, b) sample in `[0, 1]`.
    pub fn beta(&mut self, a: f64, b: f64) -> f64 {
        let x = self.gamma(a.max(1e-6));
        let y = self.gamma(b.max(1e-6));
        if x + y <= 0.0 {
            0.5
        } else {
            x / (x + y)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deterministic_sequence() {
        let mut a = Rng::new(42);
        let mut b = Rng::new(42);
        for _ in 0..1000 {
            assert_eq!(a.next_u64(), b.next_u64());
        }
    }

    #[test]
    fn uniform_in_range() {
        let mut r = Rng::new(7);
        for _ in 0..10_000 {
            let u = r.uniform();
            assert!((0.0..1.0).contains(&u));
        }
    }

    #[test]
    fn beta_mean_is_sane() {
        // Beta(8, 2) has mean 0.8; check the empirical mean is close.
        let mut r = Rng::new(1);
        let n = 20_000;
        let mean: f64 = (0..n).map(|_| r.beta(8.0, 2.0)).sum::<f64>() / n as f64;
        assert!((mean - 0.8).abs() < 0.02, "mean was {mean}");
    }
}
