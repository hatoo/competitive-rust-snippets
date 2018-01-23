#[snippet = "partition_dp"]
#[allow(dead_code)]
fn partition_dp(n: usize, m: usize, p: u64) -> Vec<Vec<u64>> {
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
type ConvexHullType = i64;

#[snippet = "convex_hull_check"]
#[allow(dead_code)]
fn convex_hull_check(
    (a1, b1): (ConvexHullType, ConvexHullType),
    (a2, b2): (ConvexHullType, ConvexHullType),
    (a3, b3): (ConvexHullType, ConvexHullType),
) -> bool {
    // Convert to f64 due to overflow
    (a2 as f64 - a1 as f64) * (b3 as f64 - b2 as f64)
        >= (b2 as f64 - b1 as f64) * (a3 as f64 - a2 as f64)
}

#[test]
fn test_partition_dp() {
    const M: u64 = 1000000007;
    let dp = partition_dp(100, 50, M);

    assert_eq!(dp[4][3], 4);
    assert_eq!(dp[5][4], 6);
}
