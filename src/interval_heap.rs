use cargo_snippet::snippet;

/// IntervalHeap
#[snippet("IntervalHeap")]
#[derive(Clone, Debug)]
struct IntervalHeap<T: Ord + Eq> {
    data: Vec<T>,
}

#[snippet("IntervalHeap")]
impl<T: Ord + Eq> IntervalHeap<T> {
    #[allow(dead_code)]
    fn new() -> IntervalHeap<T> {
        IntervalHeap { data: Vec::new() }
    }

    #[allow(dead_code)]
    fn with_capacity(n: usize) -> IntervalHeap<T> {
        IntervalHeap {
            data: Vec::with_capacity(n),
        }
    }

    #[allow(dead_code)]
    #[inline]
    fn len(&self) -> usize {
        self.data.len()
    }

    #[allow(dead_code)]
    #[inline]
    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    #[allow(dead_code)]
    #[inline]
    fn push(&mut self, x: T) {
        let i = self.data.len();
        self.data.push(x);
        self.up(i);
    }

    #[allow(dead_code)]
    #[inline]
    fn peek_min(&self) -> Option<&T> {
        self.data.first()
    }

    #[allow(dead_code)]
    #[inline]
    fn peek_max(&self) -> Option<&T> {
        if self.data.len() > 1 {
            self.data.get(1)
        } else {
            self.data.first()
        }
    }

    #[allow(dead_code)]
    #[inline]
    fn pop_min(&mut self) -> Option<T> {
        if self.data.len() == 1 {
            return self.data.pop();
        }

        if self.data.is_empty() {
            return None;
        }

        let len = self.data.len();
        self.data.swap(0, len - 1);
        let res = self.data.pop();
        self.down(0);
        res
    }

    #[allow(dead_code)]
    #[inline]
    fn pop_max(&mut self) -> Option<T> {
        if self.data.len() <= 2 {
            return self.data.pop();
        }

        if self.data.is_empty() {
            return None;
        }

        let len = self.data.len();
        self.data.swap(1, len - 1);
        let res = self.data.pop();
        self.down(1);
        res
    }

    #[allow(dead_code)]
    #[inline]
    fn parent(i: usize) -> usize {
        ((i >> 1) - 1) & !1
    }

    #[allow(dead_code)]
    #[inline]
    fn down(&mut self, i: usize) {
        let mut i = i;

        let n = self.data.len();
        if i & 1 == 0 {
            while (i << 1) + 2 < n {
                let mut k = (i << 1) + 2;
                // if k + 2 < n && self.data[k + 2] < self.data[k] {
                if k + 2 < n
                    && unsafe { self.data.get_unchecked(k + 2) }
                        < unsafe { self.data.get_unchecked(k) }
                {
                    k += 2;
                }
                // if self.data[i] > self.data[k] {
                if unsafe { self.data.get_unchecked(i) } > unsafe { self.data.get_unchecked(k) } {
                    self.data.swap(i, k);
                    i = k;
                    // if i + 1 < self.data.len() && self.data[i] > self.data[i + 1] {
                    if i + 1 < self.data.len()
                        && unsafe { self.data.get_unchecked(i) }
                            > unsafe { self.data.get_unchecked(i + 1) }
                    {
                        self.data.swap(i, i + 1);
                    }
                } else {
                    break;
                }
            }
        } else {
            while (i << 1) + 1 < n {
                let mut k = (i << 1) + 1;
                // if k + 2 < n && self.data[k + 2] > self.data[k] {
                if k + 2 < n
                    && unsafe { self.data.get_unchecked(k + 2) }
                        > unsafe { self.data.get_unchecked(k) }
                {
                    k += 2;
                }
                // if self.data[i] < self.data[k] {
                if unsafe { self.data.get_unchecked(i) } < unsafe { self.data.get_unchecked(k) } {
                    self.data.swap(i, k);
                    i = k;
                    // if i > 0 && self.data[i] < self.data[i - 1] {
                    if i > 0
                        && unsafe { self.data.get_unchecked(i) }
                            < unsafe { self.data.get_unchecked(i - 1) }
                    {
                        self.data.swap(i, i - 1);
                    }
                } else {
                    break;
                }
            }
        }
    }

    #[allow(dead_code)]
    #[inline]
    fn up(&mut self, i: usize) {
        let mut i = i;
        // if i & 1 == 1 && self.data[i] < self.data[i - 1] {
        if i & 1 == 1
            && unsafe { self.data.get_unchecked(i) } < unsafe { self.data.get_unchecked(i - 1) }
        {
            self.data.swap(i, i - 1);
            i -= 1;
        }

        //  while i > 1 && self.data[i] < self.data[Self::parent(i)] {
        while i > 1
            && unsafe { self.data.get_unchecked(i) }
                < unsafe { self.data.get_unchecked(Self::parent(i)) }
        {
            let p = Self::parent(i);
            self.data.swap(i, p);
            i = p;
        }

        // while i > 1 && self.data[i] > self.data[Self::parent(i) + 1] {
        while i > 1
            && unsafe { self.data.get_unchecked(i) }
                > unsafe { self.data.get_unchecked(Self::parent(i) + 1) }
        {
            let p = Self::parent(i) + 1;
            self.data.swap(i, p);
            i = p;
        }
    }

    #[allow(dead_code)]
    #[inline]
    fn clear(&mut self) {
        self.data.clear();
    }
}

#[snippet("IntervalHeap")]
#[derive(Clone, Debug)]
struct LimitedIntervalHeap<T: Ord + Eq> {
    heap: IntervalHeap<T>,
    limit: usize,
}

#[snippet("IntervalHeap")]
impl<T: Ord + Eq> LimitedIntervalHeap<T> {
    #[allow(dead_code)]
    fn new(limit: usize) -> LimitedIntervalHeap<T> {
        LimitedIntervalHeap {
            heap: IntervalHeap::with_capacity(limit),
            limit,
        }
    }

    #[allow(dead_code)]
    #[inline]
    fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    #[allow(dead_code)]
    #[inline]
    fn push(&mut self, x: T) -> Option<T> {
        if self.heap.len() < self.limit {
            self.heap.push(x);
            None
        } else if self.heap.data[0] < x {
            let mut x = x;
            std::mem::swap(&mut x, &mut self.heap.data[0]);
            if self.heap.len() >= 2 && self.heap.data[0] > self.heap.data[1] {
                self.heap.data.swap(0, 1);
            }
            self.heap.down(0);

            Some(x)
        } else {
            Some(x)
        }
    }

    #[allow(dead_code)]
    #[inline]
    fn pop(&mut self) -> Option<T> {
        self.heap.pop_max()
    }

    #[allow(dead_code)]
    #[inline]
    fn clear(&mut self) {
        self.heap.clear();
    }
}

#[test]
fn test_interval_heap_push_pop() {
    let size = 10000;

    use rand::{Rng, SeedableRng, StdRng};
    use std::collections::BinaryHeap;

    let mut rng = StdRng::from_seed(&[1, 2, 3]);

    let mut interval_heap = IntervalHeap::new();
    let mut binary_heap = BinaryHeap::new();

    for _ in 0..size {
        let x = rng.next_u64();

        interval_heap.push(x);
        binary_heap.push(x);
    }

    for _ in 0..size + 1 {
        assert_eq!(binary_heap.pop(), interval_heap.pop_max());
    }
}

#[test]
fn test_interval_heap_vs_vec_deque() {
    let size = 10000;

    use rand::{Rng, SeedableRng, StdRng};
    use std::collections::VecDeque;

    let mut rng = StdRng::from_seed(&[1, 2, 3]);

    let mut interval_heap = IntervalHeap::new();
    let mut vec = Vec::new();

    for _ in 0..size {
        let x = rng.next_u64();

        interval_heap.push(x);
        vec.push(x);
    }

    vec.sort();
    let mut vec_deque = vec.into_iter().collect::<VecDeque<_>>();

    for _ in 0..size + 1 {
        if rng.next_f64() < 0.5 {
            assert_eq!(interval_heap.pop_min(), vec_deque.pop_front());
        } else {
            assert_eq!(interval_heap.pop_max(), vec_deque.pop_back());
        }
    }
}

#[test]
fn test_interval_heap_combined() {
    let size = 1000;

    use rand::{Rng, SeedableRng, StdRng};
    use std::collections::HashSet;

    let mut rng = StdRng::from_seed(&[1, 2, 3]);

    let mut interval_heap = IntervalHeap::new();
    let mut set = HashSet::new();

    for _ in 0..size {
        let x = rng.next_u64();

        interval_heap.push(x);
        set.insert(x);
    }

    for _ in 0..size {
        if rng.next_f64() < 0.5 {
            let x = rng.next_u64();
            interval_heap.push(x);
            set.insert(x);
        } else {
            if rng.next_f64() < 0.5 {
                let expected = set.iter().cloned().min();
                assert_eq!(interval_heap.pop_min(), expected);
                if let Some(x) = expected {
                    set.remove(&x);
                }
            } else {
                let expected = set.iter().cloned().max();
                assert_eq!(interval_heap.pop_max(), expected);
                if let Some(x) = expected {
                    set.remove(&x);
                }
            }
        }
    }
}

#[test]
fn test_limited_interval_heap() {
    let size = 10000;

    use rand::{Rng, SeedableRng, StdRng};
    use std::collections::BinaryHeap;

    let mut rng = StdRng::from_seed(&[1, 2, 3]);

    let mut interval_heap = LimitedIntervalHeap::new(size / 2);
    let mut binary_heap = BinaryHeap::new();

    for _ in 0..size {
        let x = rng.next_u64();

        interval_heap.push(x);
        binary_heap.push(x);
    }

    for _ in 0..size / 2 {
        assert_eq!(binary_heap.pop(), interval_heap.pop());
    }
}

#[cfg(test)]
use test::Bencher;

#[bench]
fn bench_sort(b: &mut Bencher) {
    let size = 100000;

    use rand::{Rng, SeedableRng, StdRng};

    let mut vec = Vec::with_capacity(size);
    let mut rng = StdRng::from_seed(&[1, 2, 3]);

    b.iter(|| {
        for _ in 0..size {
            vec.push(rng.next_u64());
        }
        vec.sort();
    });
}

#[bench]
fn bench_binary_heap(b: &mut Bencher) {
    let size = 100000;

    use rand::{Rng, SeedableRng, StdRng};
    use std::collections::BinaryHeap;

    let mut heap = BinaryHeap::new();
    let mut rng = StdRng::from_seed(&[1, 2, 3]);

    b.iter(|| {
        for _ in 0..size {
            heap.push(rng.next_u64());
        }
    });
}

#[bench]
fn bench_interval_heap(b: &mut Bencher) {
    let size = 100000;

    use rand::{Rng, SeedableRng, StdRng};

    let mut heap = IntervalHeap::new();
    let mut rng = StdRng::from_seed(&[1, 2, 3]);

    b.iter(|| {
        for _ in 0..size {
            heap.push(rng.next_u64());
        }
    });
}

#[bench]
fn bench_limited_interval_heap_limit_half(b: &mut Bencher) {
    let size = 100000;

    use rand::{Rng, SeedableRng, StdRng};

    let mut heap = LimitedIntervalHeap::new(size / 2);
    let mut rng = StdRng::from_seed(&[1, 2, 3]);

    b.iter(|| {
        for _ in 0..size {
            heap.push(rng.next_u64());
        }
    });
}
