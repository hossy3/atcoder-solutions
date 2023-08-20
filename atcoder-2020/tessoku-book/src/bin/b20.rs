use proconio::{input, marker::Chars};

// Edit distance

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    // dp[i][j]: distance of s[..i] with t[..j]
    let mut dp = vec![vec![0; t.len() + 1]; s.len() + 1];

    for i in 0..=s.len() {
        dp[i][0] = i;
    }
    for j in 0..=t.len() {
        dp[0][j] = j;
    }
    for i in 0..s.len() {
        for j in 0..t.len() {
            if s[i] == t[j] {
                dp[i + 1][j + 1] = dp[i][j];
            } else {
                dp[i + 1][j + 1] = dp[i][j].min(dp[i + 1][j]).min(dp[i][j + 1]) + 1;
            }
        }
    }
    let result = dp[s.len()][t.len()];
    println!("{}", result);
}
