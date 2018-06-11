#[snippet]
#[allow(dead_code)]
/// v[n][k] = nCk / 2^n
fn nck_prob(n: usize) -> Vec<Vec<f64>> {
    let mut res = vec![vec![1.0]];

    for _ in 1..n {
        let mut v = Vec::new();
        {
            let last = res.last().unwrap();
            v.push(last.first().unwrap() / 2.0);
            for i in 0..last.len() - 1 {
                v.push((last[i] + last[i + 1]) / 2.0);
            }
            v.push(last.last().unwrap() / 2.0);
        }
        res.push(v);
    }
    res
}

#[snippet = "partition_dp"]
#[allow(dead_code)]
/// dp[i][j] = j th partition number of i
pub fn partition_dp(n: usize, m: usize, p: u64) -> Vec<Vec<u64>> {
    let mut dp = vec![vec![0; m + 1]; n + 1];
    for i in 0..m + 1 {
        dp[0][i] = 1;
    }
    for i in 1..n + 1 {
        for j in 1..m + 1 {
            if i >= j {
                dp[i][j] = (dp[i - j][j] + dp[i][j - 1]) % p;
            } else {
                dp[i][j] = dp[i][j - 1];
            }
        }
    }
    dp
}

#[snippet = "convex_hull_check"]
#[allow(dead_code)]
/// A check function for convex hull trick
pub fn convex_hull_check((a1, b1): (i64, i64), (a2, b2): (i64, i64), (a3, b3): (i64, i64)) -> bool {
    // Convert to f64 due to overflow
    (a2 as f64 - a1 as f64) * (b3 as f64 - b2 as f64)
        >= (b2 as f64 - b1 as f64) * (a3 as f64 - a2 as f64)
}

#[snippet = "factor_table"]
#[allow(dead_code)]
pub fn factor_table(max_n: usize) -> Vec<usize> {
    let mut res = vec![0; max_n + 1];
    // res[1] = 1;
    for i in 2..max_n + 1 {
        if res[i] == 0 {
            let mut j = i;
            while j <= max_n {
                res[j] = i;
                j += i;
            }
        }
    }

    res
}

#[snippet = "XorShift"]
#[derive(Debug)]
#[allow(dead_code)]
pub struct Xorshift {
    seed: u64,
}

#[snippet = "XorShift"]
impl Xorshift {
    #[allow(dead_code)]
    pub fn new() -> Xorshift {
        Xorshift {
            seed: 0xf0fb588ca2196dac,
        }
    }

    #[allow(dead_code)]
    pub fn with_seed(seed: u64) -> Xorshift {
        Xorshift { seed: seed }
    }

    #[inline(always)]
    #[allow(dead_code)]
    pub fn next(&mut self) -> u64 {
        self.seed = self.seed ^ (self.seed << 13);
        self.seed = self.seed ^ (self.seed >> 7);
        self.seed = self.seed ^ (self.seed << 17);
        self.seed
    }

    #[inline(always)]
    #[allow(dead_code)]
    pub fn rand(&mut self, m: u64) -> u64 {
        self.next() % m
    }

    #[inline(always)]
    #[allow(dead_code)]
    pub fn randf(&mut self) -> f64 {
        use std::mem;
        const UPPER_MASK: u64 = 0x3FF0000000000000;
        const LOWER_MASK: u64 = 0xFFFFFFFFFFFFF;
        let tmp = UPPER_MASK | (self.next() & LOWER_MASK);
        let result: f64 = unsafe { mem::transmute(tmp) };
        result - 1.0
    }
}

#[test]
fn test_xorshift_randf() {
    let mut rng = Xorshift::new();
    for _ in 0..1_000_000 {
        let f = rng.randf();
        assert!(f >= 0.0);
        assert!(f <= 1.0);
    }
}

#[cfg(test)]
use test::Bencher;

#[bench]
fn bench_xor_shift_next(b: &mut Bencher) {
    let mut rng = Xorshift::new();
    b.iter(|| {
        for _ in 0..1_000_000 {
            rng.next();
        }
    });
}

#[bench]
fn bench_xor_shift_rand(b: &mut Bencher) {
    let mut rng = Xorshift::new();
    b.iter(|| {
        for _ in 0..1_000_000 {
            rng.rand(10000);
        }
    });
}

#[bench]
fn bench_xor_shift_randf(b: &mut Bencher) {
    let mut rng = Xorshift::new();
    b.iter(|| {
        for _ in 0..1_000_000 {
            rng.randf();
        }
    });
}

#[test]
fn test_partition_dp() {
    const M: u64 = 1000000007;
    let dp = partition_dp(100, 50, M);

    assert_eq!(dp[4][3], 4);
    assert_eq!(dp[5][4], 6);
}

#[test]
fn test_factor_table() {
    let n = 1000;
    let table = factor_table(n);
    for i in 2..n + 1 {
        assert_eq!(i % table[i], 0);
    }
}
