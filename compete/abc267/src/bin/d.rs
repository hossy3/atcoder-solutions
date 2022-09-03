use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i64; n],
    }
    let mut dp = vec![vec![0; n]; m];
    for i in 0..n {
        dp[0][i] = a[i];
    }
    for i in 0..(m - 1) {
        let mut max = dp[i][i];
        for j in i..(n - 1) {
            max = max.max(dp[i][j]);
            dp[i + 1][j + 1] = max + a[j + 1] * ((i + 2) as i64);
        }
    }
    let score = dp[m - 1][(m - 1)..].iter().max().unwrap_or(&0);
    println!("{}", score);
}
