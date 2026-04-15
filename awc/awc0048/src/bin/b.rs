use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [usize; n],
        d: [usize; n],
    }
    let mut dp = vec![usize::MAX; n];
    dp[0] = if h[0] == 0 || d[0] == 0 { 0 } else { 1 };
    for i in 0..(n - 1) {
        dp[i + 1] = dp[i + 1].min(dp[i] + if h[i + 1] == 0 || d[i + 1] == 0 { 0 } else { 1 });
        if i < n - 2 {
            dp[i + 2] = dp[i + 2].min(dp[i] + if h[i + 2] == 0 || d[i + 2] == 0 { 0 } else { 1 });
        }
    }
    let result = dp[n - 1];
    println!("{result}");
}
