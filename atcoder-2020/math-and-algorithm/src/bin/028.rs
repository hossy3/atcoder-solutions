use proconio::input;

fn abs_diff(a: usize, b: usize) -> usize {
    if a > b {
        a - b
    } else {
        b - a
    }
}

fn main() {
    input! {
        n: usize,
        h: [usize; n],
    }
    let mut dp = vec![std::usize::MAX; n];
    dp[0] = 0;
    for i in 0..(n - 2) {
        dp[i + 2] = dp[i] + abs_diff(h[i], h[i + 2]);
        dp[i + 1] = dp[i + 1].min(dp[i] + abs_diff(h[i], h[i + 1]));
    }
    dp[n - 1] = dp[n - 1].min(dp[n - 2] + abs_diff(h[n - 2], h[n - 1]));
    let result = dp[n - 1];
    println!("{}", result);
}
