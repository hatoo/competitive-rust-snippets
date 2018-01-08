#[snippet = "RollingHash"]
#[allow(dead_code)]
struct RollingHash {
    hash: Vec<u64>,
    pow: Vec<u64>,
}

#[snippet = "RollingHash"]
#[allow(dead_code)]
impl RollingHash {
    const M: u64 = 1000000007;
    const B: u64 = 1009;

    fn new(s: &[u64]) -> RollingHash {
        let mut hash = Vec::with_capacity(s.len() + 1);
        let mut pow = Vec::with_capacity(s.len() + 1);

        hash.push(0);
        pow.push(1);

        for (i, &x) in s.iter().enumerate() {
            let h = hash[i];
            let p = pow[i];
            hash.push((h + x) * Self::B % Self::M);
            pow.push(p * Self::B % Self::M);
        }

        RollingHash {
            hash: hash,
            pow: pow,
        }
    }

    // [l, r)
    fn get(&self, l: usize, r: usize) -> u64 {
        (self.hash[r] + Self::M - self.hash[l] * self.pow[r - l] % Self::M) % Self::M
    }

    fn len(&self) -> usize {
        self.hash.len() - 1
    }
}

#[test]
fn test_rolling_hash() {
    let seq: Vec<u64> = "abcabc".chars().map(|c| c as u64).collect();

    let rh = RollingHash::new(&seq);

    assert_eq!(rh.get(0, 3), rh.get(3, 6));
    assert_ne!(rh.get(0, 4), rh.get(3, 6));
}
