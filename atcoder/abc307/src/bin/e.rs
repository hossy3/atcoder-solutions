use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }
    const MOD: usize = 998244353;
    let mut dp = vec![(0usize, 0usize); n];
    dp[0] = (0, 1);
    for i in 0..(n - 1) {
        dp[i + 1] = ((dp[i].0 * (m - 2) + dp[i].1 * (m - 1)) % MOD, dp[i].0);
    }
    let result = (dp[n - 1].0 * m) % MOD;
    println!("{}", result);
}
