use std;

#[snippet = "M"]
#[allow(dead_code)]
pub const M: u64 = 1_000_000_007;

#[snippet = "CmpBy"]
pub struct CmpBy<T, U>(T, U);
#[snippet = "CmpBy"]
impl<T: PartialEq, U> PartialEq for CmpBy<T, U> {
    fn eq(&self, other: &Self) -> bool {
        (self.0).eq(&other.0)
    }
}
#[snippet = "CmpBy"]
impl<T: Eq, U> Eq for CmpBy<T, U> {}
#[snippet = "CmpBy"]
impl<T: PartialOrd, U> PartialOrd for CmpBy<T, U> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (self.0).partial_cmp(&other.0)
    }
}
#[snippet = "CmpBy"]
impl<T: Ord, U> Ord for CmpBy<T, U> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.0).cmp(&other.0)
    }
}
