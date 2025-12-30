use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
        c: [usize; n],
    }

    let mut dp = vec![[0usize; 3]; n + 1];
    dp[1][0] = a[0];
    dp[2][0] = a[0] + a[1];
    dp[2][1] = a[0] + b[1];
    for i in 2..n {
        dp[i + 1][0] = dp[i][0] + a[i];
        dp[i + 1][1] = dp[i][0].max(dp[i][1]) + b[i];
        dp[i + 1][2] = dp[i][1].max(dp[i][2]) + c[i];
    }
    let result = dp[n][2];
    println!("{result}");
}
