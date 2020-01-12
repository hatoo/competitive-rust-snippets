#[snippet("RcList")]
use std::rc::Rc;

#[snippet("RcList")]
#[derive(Debug)]
struct RcListInner<T> {
    parent: RcList<T>,
    value: T,
}

/// O(1) clone, O(1) push
#[snippet("RcList")]
#[derive(Clone, Debug)]
struct RcList<T>(Option<Rc<RcListInner<T>>>);

#[snippet("RcList")]
impl<T: Clone> RcList<T> {
    #[allow(dead_code)]
    fn new() -> Self {
        RcList(None)
    }

    #[allow(dead_code)]
    #[inline]
    fn push(&mut self, value: T) {
        *self = RcList(Some(Rc::new(RcListInner {
            parent: self.clone(),
            value,
        })));
    }

    #[allow(dead_code)]
    fn to_vec(&self) -> Vec<T> {
        if let Some(ref inner) = self.0 {
            let mut p = inner.parent.to_vec();
            p.push(inner.value.clone());
            p
        } else {
            Vec::new()
        }
    }
}
