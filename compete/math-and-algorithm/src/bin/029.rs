use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut dp = vec![0usize; n + 1];
    dp[0] = 1;
    for i in 0..(n - 1) {
        dp[i + 1] += dp[i];
        dp[i + 2] += dp[i];
    }
    dp[n] += dp[n - 1];
    let result = dp[n];
    println!("{}", result);
}
