use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
        k: usize,
        wb: [(usize, isize); n],
    }

    let mut dp = vec![vec![-1; l + 1]; n + 1]; // dp[枚数][長さ]=最大集客効果
    dp[0][0] = 0;
    for &(w, b) in &wb {
        for i in (0..n).rev() {
            for j in (0..=(l - w)).rev() {
                if dp[i][j] >= 0 {
                    dp[i + 1][j + w] = dp[i + 1][j + w].max(dp[i][j] + b);
                }
            }
        }
    }

    let mut result = -1;
    for i in 0..=n {
        for j in 0..=l {
            if (i + 1) * (k - 1) >= l - j {
                result = result.max(dp[i][j]);
            }
        }
    }
    println!("{result}");
}
