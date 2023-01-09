use proconio::{input, marker::Chars};

// LCS - Longest common subsequence

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let mut dp = vec![vec![0; t.len() + 1]; s.len() + 1];
    for i in 0..s.len() {
        for j in 0..t.len() {
            let mut x = dp[i + 1][j].max(dp[i][j + 1]);
            if s[i] == t[j] {
                x = x.max(dp[i][j] + 1);
            }
            dp[i + 1][j + 1] = x;
        }
    }

    let result = dp[s.len()][t.len()];
    println!("{}", result);
}
