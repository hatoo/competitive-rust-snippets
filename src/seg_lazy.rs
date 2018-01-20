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
        let n = (1..)
            .map(|i| 2usize.pow(i as u32))
            .find(|&x| x >= n)
            .unwrap();
        SEG {
            n: n,
            buf: vec![init; 2 * n],
            phantom: std::marker::PhantomData,
        }
    }

    #[allow(dead_code)]
    fn eval(&mut self, k: usize, l: usize, r: usize) {
        //
        /*
        let x = self.buf[k].1;
        self.buf[k].1 = 0;
        self.buf[k].0 += x;
        if r - l > 1 {
            self.buf[2 * k + 1].1 += x / 2;
            self.buf[2 * k + 2].1 += x / 2;
        }
        */
        if r - l > 1 {
            let (l, r) = self.buf.split_at_mut(2 * k + 1);
            let (c1, c2) = r.split_at_mut(1);
            T::eval(&mut l[k], Some((&mut c1[0], &mut c2[0])));
        } else {
            T::eval(&mut self.buf[k], None);
        }
    }

    #[allow(dead_code)]
    fn range_add(&mut self, x: &T::A, a: usize, b: usize, k: usize, l: usize, r: usize) {
        self.eval(k, l, r);
        if r <= a || b <= l {
            return;
        }
        if a <= l && r <= b {
            //
            // self.buf[k].1 += (r - l) as u64 * x;
            T::range(x, &mut self.buf[k], l, r);
            self.eval(k, l, r);
            return;
        }

        self.range_add(x, a, b, 2 * k + 1, l, (l + r) / 2);
        self.range_add(x, a, b, 2 * k + 2, (l + r) / 2, r);
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
                (Some(l), Some(r)) => Some(T::reduce(l, r)),
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
    fn result(elem: &Self::Elem) -> Self::R;
    fn reduce(a: Self::R, b: Self::R) -> Self::R;
}
