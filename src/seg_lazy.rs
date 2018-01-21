use std;

#[snippet = "SEG_LAZY"]
#[allow(dead_code)]
struct SEG<T: SEGimpl> {
    n: usize,
    buf: Vec<T::Elem>,
    phantom: std::marker::PhantomData<T>,
}

#[snippet = "SEG_LAZY"]
impl<T: SEGimpl> SEG<T> {
    #[allow(dead_code)]
    fn new(n: usize, init: T::Elem) -> SEG<T> {
        let n = (1..).map(|i| 1 << i).find(|&x| x >= n).unwrap();
        SEG {
            n: n,
            buf: vec![init; 2 * n],
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
    fn range_add(&mut self, x: &T::A, a: usize, b: usize) {
        let n = self.n;
        self.r(x, a, b, 0, 0, n);
    }

    #[allow(dead_code)]
    fn q(&mut self, a: usize, b: usize, k: usize, l: usize, r: usize) -> Option<T::R> {
        self.eval(k, l, r);
        if r <= a || b <= l {
            return None;
        }
        if a <= l && r <= b {
            Some(T::result(&self.buf[k]))
        } else {
            let vl = self.q(a, b, k * 2 + 1, l, (l + r) / 2);
            let vr = self.q(a, b, k * 2 + 2, (l + r) / 2, r);
            match (vl, vr) {
                (Some(l), Some(r)) => Some(T::reduce_result(l, r)),
                (Some(l), None) => Some(l),
                (None, Some(r)) => Some(r),
                _ => None,
            }
        }
    }
    #[allow(dead_code)]
    fn query(&mut self, a: usize, b: usize) -> Option<T::R> {
        let n = self.n;
        self.q(a, b, 0, 0, n)
    }
}

#[snippet = "SEG_LAZY"]
trait SEGimpl {
    type Elem: Clone;
    type A;
    type R;

    fn eval(parent: &mut Self::Elem, children: Option<(&mut Self::Elem, &mut Self::Elem)>);
    fn range(x: &Self::A, elem: &mut Self::Elem, l: usize, r: usize);
    fn reduce(parent: &mut Self::Elem, c1: &Self::Elem, c2: &Self::Elem);
    fn result(elem: &Self::Elem) -> Self::R;
    fn reduce_result(a: Self::R, b: Self::R) -> Self::R;
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
    fn result(elem: &Self::Elem) -> Self::R {
        elem.0
    }
    fn reduce(parent: &mut Self::Elem, c1: &Self::Elem, c2: &Self::Elem) {
        parent.0 = c1.0 + c2.0;
    }
    fn reduce_result(a: Self::R, b: Self::R) -> Self::R {
        a + b
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
