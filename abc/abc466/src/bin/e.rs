use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        ab: [(usize, usize); n],
    }

    let mut dp = vec![vec![0usize; k * 2 + 1]; n + 1];
    for (i, &(a, b)) in ab.iter().enumerate() {
        // 前回が表
        for j in 0..k {
            dp[i + 1][j * 2] = dp[i + 1][j * 2].max(dp[i][j * 2] + a);
            dp[i + 1][j * 2 + 1] = dp[i + 1][j * 2 + 1].max(dp[i][j * 2] + b);
        }
        dp[i + 1][k * 2] = dp[i + 1][k * 2].max(dp[i][k * 2] + a);

        // 前回が裏
        for j in 0..k {
            dp[i + 1][j * 2 + 1] = dp[i + 1][j * 2 + 1].max(dp[i][j * 2 + 1] + b);
            dp[i + 1][j * 2 + 2] = dp[i + 1][j * 2 + 2].max(dp[i][j * 2 + 1] + a);
        }
    }

    let result = *dp[n].iter().max().unwrap();
    println!("{result}");
}
