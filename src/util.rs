use rand::Rng;
use std::ops::Range;
use std::cmp::{max, min};

/// return a..b
/// l <= a <= b <= r
#[allow(dead_code)]
pub fn random_range<R: Rng>(rand: &mut R, l: usize, r: usize) -> Range<usize> {
    let a = l + rand.next_u32() as usize % (r - l + 1);
    let b = l + rand.next_u32() as usize % (r - l + 1);

    min(a, b)..max(a, b)
}
