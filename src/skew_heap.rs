//! Skew Heap
//!
//! std::collections::BinaryHeap is always faster in my experience.

use std;
use cargo_snippet::snippet;

#[snippet("SkewHeap")]
#[derive(Debug, Clone)]
struct SkewHeapNode<T: Ord> {
    v: T,
    l: SkewHeap<T>,
    r: SkewHeap<T>,
    length: usize,
}
#[snippet("SkewHeap")]
#[derive(Debug, Clone)]
pub struct SkewHeap<T: Ord>(Option<Box<SkewHeapNode<T>>>);

#[snippet("SkewHeap")]
impl<T: Ord> SkewHeapNode<T> {
    fn swap(&mut self) {
        let &mut SkewHeapNode {
            ref mut l,
            ref mut r,
            ..
        } = self;

        std::mem::swap(l, r);
    }

    fn divide(self) -> (T, SkewHeap<T>, SkewHeap<T>) {
        let SkewHeapNode { v, l, r, .. } = self;

        (v, l, r)
    }
}

#[snippet("SkewHeap")]
impl<T: Ord> SkewHeap<T> {
    pub fn new() -> SkewHeap<T> {
        SkewHeap(None)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_none()
    }

    pub fn len(&self) -> usize {
        self.0.as_ref().map(|n| n.length).unwrap_or(0)
    }

    pub fn meld(&mut self, mut other: SkewHeap<T>) {
        if other.0.is_none() {
            return;
        }

        if self.0.is_none() {
            *self = other;
            return;
        }

        if self.0.as_ref().unwrap().as_ref().v < other.0.as_ref().unwrap().as_ref().v {
            std::mem::swap(self, &mut other);
        }

        if let Some(ref mut node) = self.0.as_mut() {
            node.length += other.0.as_ref().unwrap().length;
            node.r.meld(other);
            node.swap();
        }
    }

    pub fn push(&mut self, x: T) {
        let n = SkewHeap(Some(Box::new(SkewHeapNode {
            v: x,
            l: SkewHeap::new(),
            r: SkewHeap::new(),
            length: 1,
        })));
        self.meld(n);
    }

    pub fn pop(&mut self) -> Option<T> {
        if let Some(node) = self.0.take() {
            let (v, mut l, r) = node.divide();
            l.meld(r);
            *self = l;
            Some(v)
        } else {
            None
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.0.as_ref().map(|node| &node.v)
    }
}

#[test]
fn test_skew_heap() {
    use rand::{Rng, SeedableRng, StdRng};
    let mut rng = StdRng::from_seed(&[1, 2, 3, 1, 2, 3, 4]);

    let size = 100_000;
    let mut bin_heap = std::collections::BinaryHeap::new();
    let mut skew_heap = SkewHeap::new();

    for _ in 0..size {
        let item = rng.next_u64();
        bin_heap.push(item);
        skew_heap.push(item);
        assert_eq!(skew_heap.len(), bin_heap.len());
    }

    for _ in 0..size {
        assert_eq!(skew_heap.pop(), bin_heap.pop());
        assert_eq!(skew_heap.len(), bin_heap.len());
    }
}

#[test]
fn test_skew_heap_real() {
    use rand::{Rng, SeedableRng, StdRng};
    let mut rng = StdRng::from_seed(&[1, 2, 3, 1, 2, 3, 4]);

    let size = 1_000_000;
    let mut bin_heap = std::collections::BinaryHeap::new();
    let mut skew_heap = SkewHeap::new();

    for _ in 0..size {
        if rng.next_f64() > 0.3 {
            let item = rng.next_u64();
            bin_heap.push(item);
            skew_heap.push(item);
            assert_eq!(skew_heap.len(), bin_heap.len());
        } else {
            assert_eq!(skew_heap.pop(), bin_heap.pop());
            assert_eq!(skew_heap.len(), bin_heap.len());
        }
    }
}
