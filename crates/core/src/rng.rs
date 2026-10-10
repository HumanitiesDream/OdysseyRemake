//! Deterministic random numbers. A fresh stream is derived from (seed, tag, month),
//! so the result never depends on thread order.

#[derive(Clone, Debug)]
pub struct Rng(u64);

fn mix(mut z: u64) -> u64 {
    z = z.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

/// FNV-1a over bytes. Used for names and fingerprints.
pub fn fnv64(bytes: &[u8]) -> u64 {
    let mut h = 0xcbf2_9ce4_8422_2325u64;
    for &b in bytes { h = (h ^ b as u64).wrapping_mul(0x0000_0100_0000_01b3); }
    h
}

impl Rng {
    pub fn new(seed: u64) -> Self { Rng(mix(seed)) }
    /// A stream for one (seed, tag, step) triple.
    pub fn derive(seed: u64, tag: &str, step: u64) -> Self {
        Rng(mix(seed ^ mix(fnv64(tag.as_bytes())) ^ mix(step.wrapping_mul(0xD6E8_FEB8_6659_FD93))))
    }
    pub fn next_u64(&mut self) -> u64 { self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15); mix(self.0) }
    /// Uniform in [0, 1).
    pub fn f64(&mut self) -> f64 { (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64 }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn same_inputs_same_stream() {
        let mut a = Rng::derive(7, "fire", 100);
        let mut b = Rng::derive(7, "fire", 100);
        for _ in 0..10 { assert_eq!(a.next_u64(), b.next_u64()); }
        let mut c = Rng::derive(7, "fire", 101);
        assert_ne!(Rng::derive(7, "fire", 100).next_u64(), c.next_u64());
    }
    #[test]
    fn uniform_in_range() {
        let mut r = Rng::new(1);
        let m: f64 = (0..10000).map(|_| r.f64()).sum::<f64>() / 10000.0;
        assert!((m - 0.5).abs() < 0.02);
    }
}
