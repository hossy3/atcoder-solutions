use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; 2 * n],
    }

    let mut dp = vec![vec![0; 2 * n + 1]; 2 * n + 1]; // l..r
    for w in 1..=n {
        for i in 0..=(2 * (n - w)) {
            let mut x = dp[i + 1][i + w * 2 - 1] + (a[i] - a[i + w * 2 - 1]).abs();
            for j in 1..w {
                x = x.min(dp[i][i + j * 2] + dp[i + j * 2][i + w * 2]);
            }
            dp[i][i + w * 2] = x;
        }
    }
    let result = dp[0][2 * n];
    println!("{}", result);
}
