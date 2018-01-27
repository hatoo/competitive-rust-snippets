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
        let n = (1..)
            .map(|i| 2usize.pow(i as u32))
            .find(|&x| x > n)
            .unwrap();
        SEG {
            n: n,
            buf: vec![zero.clone(); 2 * n],
            reducer: f,
            zero: zero.clone(),
        }
    }

    #[allow(dead_code)]
    pub fn update(&mut self, k: usize, a: T) {
        let mut k = k + self.n - 1;
        self.buf[k] = a;

        while k > 0 {
            k = (k - 1) / 2;
            self.buf[k] = (self.reducer)(&self.buf[k * 2 + 1], &self.buf[k * 2 + 2]);
        }
    }

    #[allow(dead_code)]
    pub fn add(&mut self, k: usize, a: &T) {
        let mut k = k + self.n - 1;
        self.buf[k] = (self.reducer)(&self.buf[k], a);

        while k > 0 {
            k = (k - 1) / 2;
            self.buf[k] = (self.reducer)(&self.buf[k * 2 + 1], &self.buf[k * 2 + 2]);
        }
    }

    #[allow(dead_code)]
    fn q(&self, a: usize, b: usize, k: usize, l: usize, r: usize) -> Option<T> {
        if r <= a || b <= l {
            return None;
        }

        if a <= l && r <= b {
            Some(self.buf[k].clone())
        } else {
            let vl = self.q(a, b, k * 2 + 1, l, (l + r) / 2);
            let vr = self.q(a, b, k * 2 + 2, (l + r) / 2, r);

            match (vl, vr) {
                (Some(l), Some(r)) => Some((self.reducer)(&l, &r)),
                (Some(l), None) => Some(l),
                (None, Some(r)) => Some(r),
                _ => None,
            }
        }
    }

    #[allow(dead_code)]
    pub fn query(&self, a: usize, b: usize) -> T {
        self.q(a, b, 0, 0, self.n)
            .unwrap_or_else(|| self.zero.clone())
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
        seg.add(i, &x);
    }

    for _ in 0..1000 {
        let r = random_range(&mut rng, 0, size);
        assert_eq!(seg.query(r.start, r.end), cum_sum[r.end] - cum_sum[r.start]);
    }
}

#[test]
fn test_segtree_same_index() {
    let seg = SEG::new(8, &0, |&a, &b| a + b);
    assert_eq!(seg.query(0, 0), 0);
}
