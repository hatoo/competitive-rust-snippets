use cargo_snippet::snippet;
use std;
use std::cmp::{max, min};
// Too complicated

#[snippet("Bucket")]
pub struct Bucket<I: BucketImpl> {
    buf: Vec<I::Elem>,
    parent: Vec<I::Parent>,
    sqrt: usize,
    phantom_i: std::marker::PhantomData<I>,
}

#[snippet("Bucket")]
impl<I: BucketImpl> Bucket<I>
where
    I::Parent: Clone,
{
    #[allow(dead_code)]
    pub fn new(init_elem: Vec<I::Elem>, init_parent: I::Parent) -> Self {
        let sqrt = (1..).find(|x| x * x >= init_elem.len()).unwrap();
        let mut parent = vec![init_parent; sqrt];

        for (i, e) in init_elem.iter().enumerate() {
            I::reduce_parent(&mut parent[i / sqrt], e);
        }

        Bucket {
            buf: init_elem,
            parent: parent,
            sqrt: sqrt,
            phantom_i: std::marker::PhantomData,
        }
    }

    // (left cut, middle, right_cut)
    #[allow(dead_code)]
    pub fn ranges(
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

    #[allow(dead_code)]
    fn pe(&mut self, i: usize) -> (&mut I::Parent, &mut I::Elem) {
        (&mut self.parent[i / self.sqrt], &mut self.buf[i])
    }

    #[allow(dead_code)]
    pub fn add(&mut self, l: usize, r: usize, delta: &I::A) {
        let (left, mid, right) = self.ranges(l, r);

        for i in left.chain(right) {
            let (p, e) = self.pe(i);
            I::add(p, e, delta);
        }

        for i in mid {
            I::add_parent(&mut self.parent[i], delta);
        }
    }

    #[allow(dead_code)]
    pub fn sum(&mut self, l: usize, r: usize) -> Option<I::R> {
        let (left, mid, right) = self.ranges(l, r);

        let mut iter = left
            .chain(right)
            .map(|i| I::elem_to_result(&self.buf[i], &self.parent[i / self.sqrt]))
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

#[snippet("Bucket")]
pub trait BucketImpl {
    type Elem;
    type Parent;
    type A;
    type R;

    fn reduce_parent(p: &mut Self::Parent, e: &Self::Elem);

    fn add(p: &mut Self::Parent, e: &mut Self::Elem, v: &Self::A);
    fn add_parent(p: &mut Self::Parent, d: &Self::A);

    fn parent_to_result(p: &Self::Parent) -> Self::R;
    fn elem_to_result(e: &Self::Elem, p: &Self::Parent) -> Self::R;
    fn reduce_result(a: &mut Self::R, b: &Self::R);
}

#[snippet("Bucket-RangeAddQueryMax")]
#[allow(dead_code)]
struct RangeAddQueryMax();

#[snippet("Bucket-RangeAddQueryMax")]
impl BucketImpl for RangeAddQueryMax {
    type Elem = u64;
    // (max, delta)
    type Parent = (u64, u64);
    type A = u64;
    type R = u64;

    fn reduce_parent(p: &mut Self::Parent, e: &Self::Elem) {
        p.0 = max(p.1 + e, p.0);
    }

    fn add(p: &mut Self::Parent, e: &mut Self::Elem, v: &Self::A) {
        *e += v;
        p.0 = max(p.0, *e + p.1);
    }

    fn add_parent(p: &mut Self::Parent, d: &Self::A) {
        p.0 += d;
        p.1 += d;
    }

    fn parent_to_result(p: &Self::Parent) -> Self::R {
        p.0
    }

    fn elem_to_result(e: &Self::Elem, p: &Self::Parent) -> Self::R {
        e + p.1
    }

    fn reduce_result(a: &mut Self::R, b: &Self::R) {
        *a = max(*a, *b);
    }
}

#[test]
fn test_range_add_query_max() {
    use crate::util::random_range;
    use rand::{Rng, SeedableRng, StdRng};

    // Test against naive vector
    let size = 1000;
    let mut vec = vec![0; size];
    let mut bucket: Bucket<RangeAddQueryMax> = Bucket::new(vec.clone(), (0, 0));

    let mut rng = StdRng::from_seed(&[1, 2, 3]);

    for _ in 0..1000 {
        // Add
        let delta = rng.next_u32() as u64 % 256;

        let range = random_range(&mut rng, 0, size);
        for i in range.clone() {
            vec[i] += delta;
        }
        bucket.add(range.start, range.end, &delta);
        // Sum
        let range = random_range(&mut rng, 0, size);

        assert_eq!(
            bucket.sum(range.start, range.end),
            vec[range].iter().max().cloned()
        );
    }
}
