#[snippet("RollingHash")]
#[allow(dead_code)]
/// Rolling hash algorithm
pub struct RollingHash {
    hash_pow_list: Vec<(u64, Vec<(u64, u64)>)>,
}

#[snippet("RollingHash")]
#[allow(dead_code)]
impl RollingHash {
    pub fn new(s: &[u64]) -> RollingHash {
        RollingHash::with_base_mod_pairs(s, &[(1009, 1_000_000_007), (9973, 999_999_937)])
    }

    pub fn with_base_mod_pairs(s: &[u64], base_mod_pairs: &[(u64, u64)]) -> RollingHash {
        let hp_list = base_mod_pairs
            .iter()
            .map(|&(base, m)| {
                let mut hp = Vec::with_capacity(s.len() + 1);
                hp.push((0, 1));

                for (i, &x) in s.iter().enumerate() {
                    let (h, p) = hp[i];
                    hp.push(((h + x) * base % m, p * base % m));
                }
                (m, hp)
            })
            .collect();

        RollingHash {
            hash_pow_list: hp_list,
        }
    }

    // [l, r)
    pub fn get(&self, l: usize, r: usize) -> u64 {
        self.hash_pow_list
            .iter()
            .map(|&(m, ref hp)| (hp[r].0 + m - hp[l].0 * hp[r - l].1 % m) % m)
            .fold(0, |a, b| a ^ b)
    }

    pub fn len(&self) -> usize {
        self.hash_pow_list
            .first()
            .map(|v| v.1.len() - 1)
            .unwrap_or(0)
    }
}

#[test]
fn test_rolling_hash() {
    let seq: Vec<u64> = "abcabc".chars().map(|c| c as u64).collect();

    let rh = RollingHash::new(&seq);

    assert_eq!(rh.get(0, 3), rh.get(3, 6));
    assert_ne!(rh.get(0, 4), rh.get(3, 6));
    assert_ne!(rh.get(0, 3), rh.get(2, 5));
}
