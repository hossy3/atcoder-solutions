use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
    }

    const MOD: usize = 998244353;
    let mut dp = vec![(0, 0); n];
    dp[0] = (1, 1);
    for i in 1..n {
        if ab[i].0 != ab[i - 1].0 {
            dp[i].0 += dp[i - 1].0;
        }
        if ab[i].0 != ab[i - 1].1 {
            dp[i].0 += dp[i - 1].1;
        }
        if ab[i].1 != ab[i - 1].0 {
            dp[i].1 += dp[i - 1].0;
        }
        if ab[i].1 != ab[i - 1].1 {
            dp[i].1 += dp[i - 1].1;
        }
        dp[i].0 %= MOD;
        dp[i].1 %= MOD;
    }

    let result = (dp[n - 1].0 + dp[n - 1].1) % MOD;
    println!("{}", result);
}
