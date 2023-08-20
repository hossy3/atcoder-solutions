use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut dp = Vec::with_capacity(n); // [l][len - 1]
    for i in 0..n {
        let mut v = Vec::with_capacity(n - i);
        v.push(1);
        dp.push(v);
    }
    for i in 0..(n - 1) {
        let c = if s[i] == s[i + 1] { 2 } else { 1 };
        dp[i].push(c);
    }
    if n > 2 {
        for j in 2..n {
            for i in 0..(n - j) {
                let mut c = dp[i][j - 1].max(dp[i + 1][j - 1]);
                if s[i] == s[i + j] {
                    c = c.max(dp[i + 1][j - 2] + 2);
                }
                dp[i].push(c);
            }
        }
    }
    let result = dp[0][n - 1];
    println!("{}", result);
}
