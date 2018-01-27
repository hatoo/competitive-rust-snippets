/// Same of std::cmp::Reverse.
/// It is useful since Atcoder's rustc is 1.15 which is not support std::cmp::Reverse.

use std::cmp::Ordering;

#[snippet = "Rev"]
#[derive(Eq, PartialEq, Clone, Debug)]
/// Equivalent to std::cmp::Reverse
pub struct Rev<T>(pub T);

#[snippet = "Rev"]
impl<T: PartialOrd> PartialOrd for Rev<T> {
    fn partial_cmp(&self, other: &Rev<T>) -> Option<Ordering> {
        other.0.partial_cmp(&self.0)
    }
}

#[snippet = "Rev"]
impl<T: Ord> Ord for Rev<T> {
    fn cmp(&self, other: &Rev<T>) -> Ordering {
        other.0.cmp(&self.0)
    }
}

#[test]
fn test_rev() {
    let mut vec = vec![2, 6, 1, 8, 4, 5, 3, 6];
    vec.sort_by_key(|&x| Rev(x));

    for w in vec.windows(2) {
        assert!(w[0] >= w[1]);
    }
}
