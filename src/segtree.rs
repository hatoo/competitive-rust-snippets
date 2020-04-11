use cargo_snippet::snippet;
#[snippet("SEG")]
#[allow(dead_code)]
pub trait Monoid {
    type T: Clone;
    fn id() -> Self::T;
    fn op(a: &Self::T, b: &Self::T) -> Self::T;
}

#[snippet("SEG")]
#[allow(dead_code)]
/// Segment Tree
pub struct SEG<M: Monoid> {
    n: usize,
    buf: Vec<M::T>,
}

#[snippet("SEG")]
impl<M: Monoid> SEG<M> {
    #[allow(dead_code)]
    pub fn new(n: usize) -> SEG<M> {
        SEG {
            n: n,
            buf: vec![M::id().clone(); 2 * n],
        }
    }

    #[allow(dead_code)]
    pub fn update(&mut self, k: usize, a: M::T) {
        let mut k = k + self.n;
        self.buf[k] = a;

        while k > 0 {
            k >>= 1;
            self.buf[k] = M::op(&self.buf[k << 1], &self.buf[(k << 1) | 1]);
        }
    }

    #[allow(dead_code)]
    pub fn add(&mut self, k: usize, a: &M::T) {
        let mut k = k + self.n;
        self.buf[k] = M::op(&self.buf[k], a);

        while k > 0 {
            k >>= 1;
            self.buf[k] = M::op(&self.buf[k << 1], &self.buf[(k << 1) | 1]);
        }
    }

    #[allow(dead_code)]
    pub fn get(&self, i: usize) -> M::T {
        self.query(i, i + 1)
    }

    #[allow(dead_code)]
    pub fn query_range<R: std::ops::RangeBounds<usize>>(&self, range: R) -> M::T {
        let l = match range.start_bound() {
            std::ops::Bound::Excluded(&x) => {
                assert!(x > 0);
                x - 1
            }
            std::ops::Bound::Included(&x) => x,
            std::ops::Bound::Unbounded => 0,
        };
        let r = match range.end_bound() {
            std::ops::Bound::Excluded(&x) => x,
            std::ops::Bound::Included(&x) => (x + 1),
            std::ops::Bound::Unbounded => self.n,
        };

        self.query(l, r)
    }

    #[allow(dead_code)]
    pub fn query(&self, l: usize, r: usize) -> M::T {
        let mut vl = M::id();
        let mut vr = M::id();

        let mut l = l + self.n;
        let mut r = r + self.n;

        while l < r {
            if l & 1 == 1 {
                vl = M::op(&vl, &self.buf[l]);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                vr = M::op(&self.buf[r], &vr);
            }

            l >>= 1;
            r >>= 1;
        }
        M::op(&vl, &vr)
    }
}

#[snippet("Monoid-SUM")]
#[allow(dead_code)]
struct SUM;
#[snippet("Monoid-SUM")]
impl Monoid for SUM {
    type T = u64;
    fn id() -> Self::T {
        0
    }
    fn op(a: &Self::T, b: &Self::T) -> Self::T {
        *a + *b
    }
}

#[test]
fn test_segtree_vs_cumulative_sum() {
    use crate::util::random_range;
    use rand::{Rng, SeedableRng, StdRng};

    let size = 1000;
    let mut cum_sum = vec![0; size + 1];
    let mut seg: SEG<SUM> = SEG::new(size);

    let mut rng = StdRng::from_seed(&[1, 2, 3]);

    let mut sum = 0;
    for i in 0..size {
        let x = rng.next_u32() as u64;
        sum += x;
        cum_sum[i + 1] = sum;
        if 1 % 2 == 0 {
            seg.add(i, &x);
        } else {
            seg.update(i, x);
        }
    }

    for _ in 0..1000 {
        let r = random_range(&mut rng, 0, size);
        assert_eq!(
            seg.query_range(r.clone()),
            cum_sum[r.end] - cum_sum[r.start]
        );
    }
}

#[test]
fn test_segtree_same_index() {
    let seg: SEG<SUM> = SEG::new(8);
    assert_eq!(seg.query(0, 0), 0);
}

#[allow(dead_code)]
struct APPEND;
impl Monoid for APPEND {
    type T = Vec<u64>;
    fn id() -> Self::T {
        Vec::new()
    }
    fn op(a: &Self::T, b: &Self::T) -> Self::T {
        let mut res = a.clone();
        res.extend(b.iter().cloned());
        res
    }
}

#[test]
fn test_segtree_non_commutative() {
    use crate::util;
    use rand::{Rng, SeedableRng, StdRng};
    let mut rng = StdRng::from_seed(&[1, 2, 3, 4, 5]);

    let size = 100;
    let mut seg: SEG<APPEND> = SEG::new(size);
    let mut v = vec![0; size];

    for i in 0..size {
        let x = rng.next_u64();
        seg.update(i, vec![x]);
        v[i] = x;
    }

    for _ in 0..100 {
        let r = util::random_range(&mut rng, 0, size);
        let res = seg.query(r.start, r.end);
        assert_eq!(res.as_slice(), &v[r]);
    }
}

#[cfg(test)]
use test::Bencher;

#[bench]
fn bench_segtree_update(b: &mut Bencher) {
    use rand::{Rng, SeedableRng, StdRng};

    let size = 10000;
    let mut seg: SEG<SUM> = SEG::new(size);
    let mut rng = StdRng::from_seed(&[1, 2, 3, 4, 5]);

    for i in 0..size {
        let x = rng.next_u64() % 256;
        seg.update(i, x);
    }

    let cases = (0..1000)
        .map(|_| {
            let x = rng.next_u64() % 256;
            let i = rng.next_u32() as usize % size;
            (x, i)
        })
        .collect::<Vec<_>>();

    b.iter(|| {
        for &(x, i) in &cases {
            seg.update(i, x);
        }
    });
}

#[bench]
fn bench_segtree_query(b: &mut Bencher) {
    use crate::util;
    use rand::{Rng, SeedableRng, StdRng};

    let size = 10000;
    let mut seg: SEG<SUM> = SEG::new(size);
    let mut rng = StdRng::from_seed(&[1, 2, 3, 4, 5]);

    for i in 0..size {
        let x = rng.next_u64() % 256;
        seg.update(i, x);
    }

    let cases = (0..1000)
        .map(|_| {
            let r = util::random_range(&mut rng, 0, size);
            r
        })
        .collect::<Vec<_>>();

    b.iter(|| {
        for r in &cases {
            seg.query(r.start, r.end);
        }
    });
}
