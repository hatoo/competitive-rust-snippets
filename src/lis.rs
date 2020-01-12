use cargo_snippet::snippet;
use std::cmp::Ordering;

use crate::binary_search::BinarySearch;

#[snippet("lis")]
#[snippet(include = "BinarySearch")]
#[derive(PartialEq, Eq, Clone, Debug)]
enum Inf<T> {
    Val(T),
    Inf,
}

#[snippet("lis")]
impl<T: Ord> Inf<T> {
    #[allow(dead_code)]
    fn val(self) -> Option<T> {
        match self {
            Inf::Val(v) => Some(v),
            _ => None,
        }
    }
}

#[snippet("lis")]
impl<T: PartialOrd> PartialOrd for Inf<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (&Inf::Inf, &Inf::Inf) => Some(Ordering::Equal),
            (&Inf::Inf, &Inf::Val(_)) => Some(Ordering::Greater),
            (&Inf::Val(_), &Inf::Inf) => Some(Ordering::Less),
            (&Inf::Val(ref a), &Inf::Val(ref b)) => a.partial_cmp(b),
        }
    }
}

#[snippet("lis")]
impl<T: Ord> Ord for Inf<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (&Inf::Inf, &Inf::Inf) => Ordering::Equal,
            (&Inf::Inf, &Inf::Val(_)) => Ordering::Greater,
            (&Inf::Val(_), &Inf::Inf) => Ordering::Less,
            (&Inf::Val(ref a), &Inf::Val(ref b)) => a.cmp(b),
        }
    }
}

#[snippet("lis")]
#[allow(dead_code)]
/// Calculate length of Longest Increasing Subsequence. O(N log N)
pub fn lis<T: Ord>(seq: &[T]) -> usize {
    let mut dp: Vec<Inf<&T>> = vec![Inf::Inf; seq.len() + 1];

    for x in seq.iter() {
        let i = dp.lower_bound(&Inf::Val(x));
        dp[i] = Inf::Val(x);
    }

    dp.lower_bound(&Inf::Inf)
}

#[test]
fn test_lis() {
    let v = vec![4, 2, 3, 1, 5];

    assert_eq!(lis(&v), 3);
}
