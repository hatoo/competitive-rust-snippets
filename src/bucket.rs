use std::cmp::{max, min};
use std;
// Too complicated

#[snippet = "Bucket"]
struct Bucket<I: BucketImpl> {
    buf: Vec<I::Elem>,
    parent: Vec<I::Parent>,
    sqrt: usize,
    phantom_i: std::marker::PhantomData<I>,
}

#[snippet = "Bucket"]
impl<I: BucketImpl> Bucket<I>
where
    I::Elem: Clone,
    I::Parent: Clone,
{
    fn new(init: &[I::Elem]) -> Self {
        let sqrt = (1..).find(|x| x * x >= init.len()).unwrap();
        let mut parent = vec![I::init_parent(); sqrt];

        for (i, e) in init.iter().enumerate() {
            I::reduce_parent(&mut parent[i / sqrt], e);
        }

        Self {
            buf: init.into(),
            parent: parent,
            sqrt: sqrt,
            phantom_i: std::marker::PhantomData,
        }
    }

    // (left cut, middle, right_cut)
    fn ranges(
        &self,
        l: usize,
        r: usize,
    ) -> (
        std::ops::Range<usize>,
        std::ops::Range<usize>,
        std::ops::Range<usize>,
    ) {
        if l / self.sqrt == r / self.sqrt {
            return (l..r, 0..0, 0..0);
        }
        let left = l..min((l + self.sqrt - 1) / self.sqrt * self.sqrt, r);
        let mid = (l + self.sqrt - 1) / self.sqrt..r / self.sqrt;
        let right = r / self.sqrt * self.sqrt..r;

        (left, mid, right)
    }

    fn pe(&mut self, i: usize) -> (&mut I::Parent, &mut I::Elem) {
        (&mut self.parent[i / self.sqrt], &mut self.buf[i])
    }

    fn range_add(&mut self, l: usize, r: usize, delta: &I::A) {
        let (left, mid, right) = self.ranges(l, r);

        for i in left.chain(right) {
            I::add(self.pe(i), delta);
        }

        for i in mid {
            I::add_parent(&mut self.parent[i], delta);
        }
    }

    fn query(&mut self, l: usize, r: usize) -> Option<I::R> {
        let (left, mid, right) = self.ranges(l, r);

        let mut iter = left.chain(right)
            .map(|i| I::elem_to_result(&self.buf[i]))
            .chain(mid.map(|i| I::parent_to_result(&self.parent[i])));

        if let Some(mut r) = iter.next() {
            for x in iter {
                I::reduce_result(&mut r, &x);
            }
            Some(r)
        } else {
            None
        }
    }
}

#[snippet = "Bucket"]
trait BucketImpl {
    type Elem;
    type Parent;
    type A;
    type R;

    fn init_parent() -> Self::Parent;
    fn reduce_parent(&mut Self::Parent, &Self::Elem);

    fn add((&mut Self::Parent, &mut Self::Elem), &Self::A);
    fn add_parent(&mut Self::Parent, &Self::A);

    fn parent_to_result(&Self::Parent) -> Self::R;
    fn elem_to_result(&Self::Elem) -> Self::R;
    fn reduce_result(&mut Self::R, &Self::R);
}

struct RangeAddQueryMax();

impl BucketImpl for RangeAddQueryMax {
    type Elem = u64;
    type Parent = (u64, u64);
    type A = u64;
    type R = u64;

    fn init_parent() -> Self::Parent {
        (0, 0)
    }

    fn reduce_parent(p: &mut Self::Parent, e: &Self::Elem) {
        p.0 = max(p.1 + e, p.0);
    }

    fn add(pe: (&mut Self::Parent, &mut Self::Elem), v: &Self::A) {
        let (p, e) = pe;
        *e += v;
        p.0 = max(p.0, *e + p.1);
    }

    fn add_parent(p: &mut Self::Parent, d: &Self::A) {
        p.0 += d;
        p.1 += d;
    }

    fn parent_to_result(p: &Self::Parent) -> Self::R {
        p.0.clone()
    }

    fn elem_to_result(e: &Self::Elem) -> Self::R {
        e.clone()
    }

    fn reduce_result(a: &mut Self::R, b: &Self::R) {
        *a = max(*a, *b);
    }
}
