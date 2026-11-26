//! The engine's only RNG: SplitMix64, seeded explicitly (ADR-0001).
//!
//! Hand-written so its output can never change under us; a known-answer test
//! pins it. Changing the algorithm needs a new ADR.

use core::ops::Range;

/// SplitMix64 (Steele, Lea, Flood 2014).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rng {
    state: u64,
}

impl Rng {
    pub fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    pub fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }

    /// A value in `range` (start inclusive, end exclusive), drawing one `u64`.
    /// Uses a plain modulo; the bias is negligible for game-sized ranges.
    ///
    /// Panics if `range` is empty.
    pub fn range(&mut self, range: Range<i32>) -> i32 {
        assert!(range.start < range.end, "empty range {range:?}");
        let span = i64::from(range.end) - i64::from(range.start);
        let span = u64::try_from(span).expect("positive span");
        let offset = i64::try_from(self.next_u64() % span).expect("offset < span");
        i32::try_from(i64::from(range.start) + offset).expect("value in range")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_answer_for_seed_0() {
        let mut rng = Rng::new(0);
        assert_eq!(rng.next_u64(), 0xe220_a839_7b1d_cdaf);
        assert_eq!(rng.next_u64(), 0x6e78_9e6a_a1b9_65f4);
        assert_eq!(rng.next_u64(), 0x06c4_5d18_8009_454f);
    }

    #[test]
    fn range_stays_in_bounds() {
        let mut rng = Rng::new(7);
        for _ in 0..1000_u32 {
            let v = rng.range(-3_i32..4_i32);
            assert!((-3_i32..4_i32).contains(&v), "{v}");
        }
        assert_eq!(rng.range(i32::MIN..i32::MIN + 1_i32), i32::MIN);
    }

    #[test]
    #[should_panic(expected = "empty range")]
    fn range_rejects_empty() {
        Rng::new(0).range(5_i32..5_i32);
    }
}
