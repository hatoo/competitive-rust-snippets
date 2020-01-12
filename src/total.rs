use std::cmp::Ordering;

#[snippet("Total")]
#[derive(PartialEq, PartialOrd)]
/// Implement Eq and Ord for a type which has only PartialEq and PartialOrd.
/// It is useful when sorting a Vec of f64
pub struct Total<T>(pub T);

#[snippet("Total")]
impl<T: PartialEq> Eq for Total<T> {}

#[snippet("Total")]
impl<T: PartialOrd> Ord for Total<T> {
    fn cmp(&self, other: &Total<T>) -> Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}

#[test]
fn test_total() {
    let mut vec = vec![9.4, 4.3, 2.3, 6.7, 3.2, 1.0, 0.0];
    vec.sort_by_key(|&f| Total(f));

    for w in vec.windows(2) {
        assert!(w[0] <= w[1]);
    }
}
