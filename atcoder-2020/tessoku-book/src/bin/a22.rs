use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [Usize1; n - 1],
        b: [Usize1; n - 1],
    }

    let mut dp = vec![-1_000_000_000; n];
    dp[0] = 0;
    for i in 0..(n - 1) {
        let (a, b) = (a[i], b[i]);
        dp[a] = dp[a].max(dp[i] + 100);
        dp[b] = dp[b].max(dp[i] + 150);
    }

    let result = dp[n - 1];
    println!("{}", result);
}
