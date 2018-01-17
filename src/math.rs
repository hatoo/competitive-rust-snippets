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

#[test]
fn test_partition_dp() {
    const M: u64 = 1000000007;
    let dp = partition_dp(100, 50, M);

    assert_eq!(dp[4][3], 4);
    assert_eq!(dp[5][4], 6);
}
