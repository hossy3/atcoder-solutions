use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h],
    }

    let mut dp = vec![vec![0usize; w]; h];
    dp[0][0] = 1;
    for i in 0..h {
        for j in 0..w {
            if c[i][j] == '#' {
                continue;
            }
            if i > 0 {
                dp[i][j] += dp[i - 1][j];
            }
            if j > 0 {
                dp[i][j] += dp[i][j - 1];
            }
        }
    }

    let result = dp[h - 1][w - 1];
    println!("{}", result);
}
