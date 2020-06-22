use crate::monoid::Monoid;
use cargo_snippet::snippet;
#[snippet("BIT")]
#[snippet(include = "Monoid")]
#[allow(dead_code)]
/// Generic Binary Indexed Tree
pub struct BIT<M: Monoid> {
    buf: Vec<M::T>,
}

#[snippet("BIT")]
impl<M: Monoid> BIT<M> {
    #[allow(dead_code)]
    pub fn new(n: usize) -> Self {
        Self {
            buf: vec![M::id(); n + 1],
        }
    }

    #[allow(dead_code)]
    pub fn sum(&self, i: usize) -> M::T {
        let mut i = i;
        let mut s = M::id();
        while i > 0 {
            s = M::op(&s, &self.buf[i]);
            i &= i - 1;
        }
        s
    }

    #[allow(dead_code)]
    pub fn add(&mut self, i: usize, x: &M::T) {
        let mut i = i as i64;
        while i < self.buf.len() as i64 {
            let t = &mut self.buf[i as usize];
            *t = M::op(&t, x);
            i += i & -i;
        }
    }
}

#[test]
fn test_bit_vs_cumsum() {
    use crate::monoid::SUM;
    use rand::{Rng, SeedableRng, StdRng};
    let size = 1000;
    let mut cum_sum = vec![0; size + 1];
    let mut bit: BIT<SUM> = BIT::new(size);

    let mut rng = StdRng::from_seed(&[1, 2, 3]);

    let mut sum = 0;
    for i in 1..size + 1 {
        let x = (rng.next_u32() as usize / (2 * size)) as u64;
        sum += x;
        cum_sum[i] = sum;
        bit.add(i, &x);
    }

    for _ in 0..1000 {
        let i = rng.next_u32() as usize % size + 1;

        assert_eq!(bit.sum(i), cum_sum[i]);
    }
}

#[cfg(test)]
use test::Bencher;

#[bench]
/// Add and sum 10^5 times to get averaged time.
/// This is typical scenario to solve a problem which is O(N log(N)) and N = 10^5.
fn bench_bit_add_sum_100k(b: &mut Bencher) {
    use crate::monoid::SUM;
    use rand::{Rng, SeedableRng, StdRng};

    let size = 100_000;
    let mut bit: BIT<SUM> = BIT::new(size);
    let mut rng = StdRng::from_seed(&[1, 2, 3]);

    let bench_size = 100000;
    let mut args = Vec::with_capacity(bench_size);

    for _ in 0..bench_size {
        let i = rng.next_u32() as usize % size + 1;
        let x = (rng.next_u32() as usize / bench_size) as u64;

        args.push((i, x));
    }

    b.iter(|| {
        for &(i, x) in &args {
            bit.add(i, &x);
            bit.sum(i);
        }
    });
}
