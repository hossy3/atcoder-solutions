use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    const MOD: usize = 998244353;
    let mut dp = vec![0usize; s.len() + 1];
    dp[0] = 1;

    for (i, &c) in s.iter().enumerate() {
        let dp0 = dp;
        dp = vec![0usize; s.len() + 1];

        for j in 0..=i {
            if c == '?' || c == '(' {
                dp[j + 1] = dp0[j];
            }
            if j > 0 && (c == '?' || c == ')') {
                dp[j - 1] = (dp[j - 1] + dp0[j]) % MOD;
            }
        }
    }

    println!("{}", dp[0]);
}
