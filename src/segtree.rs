#[snippet = "SEG"]
#[allow(dead_code)]
/// Segment Tree
pub struct SEG<T: Clone, F: Fn(&T, &T) -> T> {
    n: usize,
    buf: Vec<T>,
    reducer: F,
    zero: T,
}

#[snippet = "SEG"]
impl<T: Clone, F: Fn(&T, &T) -> T> SEG<T, F> {
    #[allow(dead_code)]
    pub fn new(n: usize, zero: &T, f: F) -> SEG<T, F> {
        SEG {
            n,
            buf: vec![zero.clone(); 2 * n],
            reducer: f,
            zero: zero.clone(),
        }
    }

    #[allow(dead_code)]
    pub fn update(&mut self, k: usize, a: T) {
        let mut k = k + self.n;
        self.buf[k] = a;

        while k > 0 {
            k >>= 1;
            self.buf[k] = (self.reducer)(&self.buf[k << 1], &self.buf[(k << 1) | 1]);
        }
    }

    #[allow(dead_code)]
    pub fn add(&mut self, k: usize, a: &T) {
        let mut k = k + self.n;
        self.buf[k] = (self.reducer)(&self.buf[k], a);

        while k > 0 {
            k >>= 1;
            self.buf[k] = (self.reducer)(&self.buf[k << 1], &self.buf[(k << 1) | 1]);
        }
    }

    #[allow(dead_code)]
    fn query(&self, l: usize, r: usize) -> Option<T> {
        let combine = |resl, resr| match (resl, resr) {
            (Some(l), Some(r)) => Some((self.reducer)(&l, &r)),
            (Some(l), None) => Some(l),
            (None, Some(r)) => Some(r),
            _ => None,
        };

        let mut vl = None;
        let mut vr = None;

        let mut l = l + self.n;
        let mut r = r + self.n;

        while l < r {
            if l & 1 == 1 {
                vl = combine(vl, Some(self.buf[l].clone()));
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                vr = combine(Some(self.buf[r].clone()), vr);
            }

            l >>= 1;
            r >>= 1;
        }
        combine(vl, vr)
    }
}

#[test]
fn test_segtree_vs_cumulative_sum() {
    use rand::{Rng, SeedableRng, StdRng};
    use util::random_range;

    let size = 1000;
    let mut cum_sum = vec![0; size + 1];
    let mut seg = SEG::new(size, &0, |&a, &b| a + b);

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
            seg.query(r.start, r.end).unwrap_or(0),
            cum_sum[r.end] - cum_sum[r.start]
        );
    }
}

#[test]
fn test_segtree_same_index() {
    let seg = SEG::new(8, &0, |&a, &b| a + b);
    assert_eq!(seg.query(0, 0).unwrap_or(0), 0);
}

#[cfg(test)]
use test::Bencher;

#[bench]
fn bench_segtree_update(b: &mut Bencher) {
    use rand::{Rng, SeedableRng, StdRng};

    let size = 10000;
    let mut seg = SEG::new(size, &0, |&a, &b| a + b);
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
    use util;
    use rand::{Rng, SeedableRng, StdRng};

    let size = 10000;
    let mut seg = SEG::new(size, &0, |&a, &b| a + b);
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
