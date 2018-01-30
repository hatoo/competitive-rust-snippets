use std;

#[snippet = "SEG_LAZY"]
#[allow(dead_code)]
/// Lazy Segment Tree
pub struct SEG<T: SEGimpl> {
    n: usize,
    buf: Vec<T::Elem>,
    zero: T::Elem,
    phantom: std::marker::PhantomData<T>,
}

#[snippet = "SEG_LAZY"]
impl<T: SEGimpl> SEG<T> {
    #[allow(dead_code)]
    pub fn new(n: usize, zero: T::Elem) -> SEG<T> {
        let n = (1..).map(|i| 1 << i).find(|&x| x >= n).unwrap();
        SEG {
            n,
            buf: vec![zero.clone(); 2 * n],
            zero,
            phantom: std::marker::PhantomData,
        }
    }

    #[allow(dead_code)]
    fn eval(&mut self, k: usize, l: usize, r: usize) {
        if r - l > 1 {
            let (l, r) = self.buf.split_at_mut(2 * k + 1);
            let (c1, c2) = r.split_at_mut(1);
            T::eval(&mut l[k], Some((&mut c1[0], &mut c2[0])));
        } else {
            T::eval(&mut self.buf[k], None);
        }
    }

    #[allow(dead_code)]
    pub fn update(&mut self, i: usize, x: T::Elem) {
        let mut k = i + self.n - 1;
        self.buf[k] = x;
        self.eval(k, i, i + 1);

        while k > 0 {
            k = (k - 1) / 2;
            let (l, r) = self.buf.split_at_mut(2 * k + 1);
            let (c1, c2) = r.split_at_mut(1);
            T::reduce(&mut l[k], &c1[0], &c2[0]);
        }
    }

    #[allow(dead_code)]
    pub fn get(&mut self, i: usize) -> Option<T::R> {
        self.query(i, i + 1)
    }

    #[allow(dead_code)]
    fn r(&mut self, x: &T::A, a: usize, b: usize, k: usize, l: usize, r: usize) {
        self.eval(k, l, r);
        if r <= a || b <= l {
            return;
        }
        if a <= l && r <= b {
            T::range(x, &mut self.buf[k], l, r);
            self.eval(k, l, r);
            return;
        }

        self.r(x, a, b, 2 * k + 1, l, (l + r) / 2);
        self.r(x, a, b, 2 * k + 2, (l + r) / 2, r);
        let (l, r) = self.buf.split_at_mut(2 * k + 1);
        let (c1, c2) = r.split_at_mut(1);
        T::reduce(&mut l[k], &c1[0], &c2[0]);
    }

    #[allow(dead_code)]
    pub fn range_add(&mut self, x: &T::A, a: usize, b: usize) {
        let n = self.n;
        self.r(x, a, b, 0, 0, n);
    }

    #[allow(dead_code)]
    pub fn add(&mut self, x: &T::A, i: usize) {
        self.range_add(x, i, i + 1);
    }

    #[allow(dead_code)]
    fn q(&mut self, a: usize, b: usize, k: usize, l: usize, r: usize) -> Option<T::Elem> {
        self.eval(k, l, r);
        if r <= a || b <= l {
            return None;
        }
        if a <= l && r <= b {
            Some(self.buf[k].clone())
        } else {
            let vl = self.q(a, b, k * 2 + 1, l, (l + r) / 2);
            let vr = self.q(a, b, k * 2 + 2, (l + r) / 2, r);
            match (vl, vr) {
                (Some(l), Some(r)) => {
                    let mut res = self.zero.clone();
                    T::reduce(&mut res, &l, &r);
                    Some(res)
                }
                (Some(l), None) => Some(l),
                (None, Some(r)) => Some(r),
                _ => None,
            }
        }
    }
    #[allow(dead_code)]
    pub fn query(&mut self, a: usize, b: usize) -> Option<T::R> {
        let n = self.n;
        self.q(a, b, 0, 0, n).map(T::to_result)
    }
}

#[snippet = "SEG_LAZY"]
pub trait SEGimpl {
    type Elem: Clone;
    type A;
    type R;

    fn eval(parent: &mut Self::Elem, children: Option<(&mut Self::Elem, &mut Self::Elem)>);
    fn range(x: &Self::A, elem: &mut Self::Elem, l: usize, r: usize);
    fn reduce(parent: &mut Self::Elem, c1: &Self::Elem, c2: &Self::Elem);
    fn to_result(elem: Self::Elem) -> Self::R;
}

#[allow(dead_code)]
struct RangeAddSum();
impl SEGimpl for RangeAddSum {
    type Elem = (u64, u64);
    type A = u64;
    type R = u64;

    fn eval(parent: &mut Self::Elem, children: Option<(&mut Self::Elem, &mut Self::Elem)>) {
        let x = parent.1;
        parent.0 += x;
        parent.1 = 0;
        if let Some((c1, c2)) = children {
            c1.1 += x / 2;
            c2.1 += x / 2;
        }
    }
    fn range(x: &Self::A, elem: &mut Self::Elem, l: usize, r: usize) {
        elem.1 += (r - l) as u64 * x;
    }
    fn reduce(parent: &mut Self::Elem, c1: &Self::Elem, c2: &Self::Elem) {
        parent.0 = c1.0 + c2.0;
    }
    fn to_result(elem: Self::Elem) -> Self::R {
        elem.0
    }
}

#[allow(dead_code)]
struct NonCommutative;
impl SEGimpl for NonCommutative {
    type Elem = Vec<u64>;
    type A = u64;
    type R = Vec<u64>;

    fn eval(_parent: &mut Self::Elem, _children: Option<(&mut Self::Elem, &mut Self::Elem)>) {}
    fn range(x: &Self::A, elem: &mut Self::Elem, _l: usize, _r: usize) {
        elem.push(*x);
    }
    fn reduce(parent: &mut Self::Elem, c1: &Self::Elem, c2: &Self::Elem) {
        parent.clear();
        parent.extend(c1.iter());
        parent.extend(c2.iter());
    }
    fn to_result(elem: Self::Elem) -> Self::R {
        elem
    }
}

#[test]
fn test_seg_lazy() {
    use util;
    use rand::{Rng, SeedableRng, StdRng};

    let size = 1000;
    let mut v = vec![0; size];
    let mut seg: SEG<RangeAddSum> = SEG::new(size, (0, 0));
    let mut rng = StdRng::from_seed(&[1, 2, 3, 4, 5]);

    for i in 0..size {
        let x = rng.next_u64() % 256;
        seg.update(i, (x, 0));
        v[i] = x;

        assert_eq!(seg.get(i), Some(v[i]));
    }

    for _ in 0..1000 {
        let x = rng.next_u64() % 256;
        let r = util::random_range(&mut rng, 0, size);
        seg.range_add(&x, r.start, r.end);
        for i in r {
            v[i] += x;
        }

        let r = util::random_range(&mut rng, 0, size);
        let seg_sum = seg.query(r.start, r.end).unwrap_or(0);
        let mut sum = 0;
        for i in r {
            sum += v[i];
        }

        assert_eq!(seg_sum, sum);
    }
}

#[test]
fn test_seg_lazy_non_commutative() {
    use util;
    use rand::{Rng, SeedableRng, StdRng};
    let mut rng = StdRng::from_seed(&[1, 2, 3, 4, 5]);

    let size = 100;
    let mut seg: SEG<NonCommutative> = SEG::new(size, Vec::new());
    let mut v = vec![0; size];

    for i in 0..size {
        let x = rng.next_u64();
        seg.update(i, vec![x]);
        v[i] = x;
    }

    for _ in 0..100 {
        let r = util::random_range(&mut rng, 0, size);
        let res = seg.query(r.start, r.end);
        assert_eq!(res.as_ref().map(|a| a.as_slice()).unwrap_or(&[]), &v[r]);
    }
}

#[cfg(test)]
use test::Bencher;

#[bench]
fn bench_lazy_segtree_range_add(b: &mut Bencher) {
    use util;
    use rand::{Rng, SeedableRng, StdRng};

    let size = 10000;
    let mut seg: SEG<RangeAddSum> = SEG::new(size, (0, 0));
    let mut rng = StdRng::from_seed(&[1, 2, 3, 4, 5]);

    for i in 0..size {
        let x = rng.next_u64() % 256;
        seg.update(i, (x, 0));
    }

    let cases = (0..1000)
        .map(|_| {
            let x = rng.next_u64() % 256;
            let r = util::random_range(&mut rng, 0, size);
            (x, r)
        })
        .collect::<Vec<_>>();

    b.iter(|| {
        for &(x, ref r) in &cases {
            seg.range_add(&x, r.start, r.end);
        }
    });
}

#[bench]
fn bench_lazy_segtree_query(b: &mut Bencher) {
    use util;
    use rand::{Rng, SeedableRng, StdRng};

    let size = 10000;
    let mut seg: SEG<RangeAddSum> = SEG::new(size, (0, 0));
    let mut rng = StdRng::from_seed(&[1, 2, 3, 4, 5]);

    for i in 0..size {
        let x = rng.next_u64() % 256;
        seg.update(i, (x, 0));
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
