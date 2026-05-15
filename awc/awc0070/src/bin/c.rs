use proconio::{input, marker::Chars};

type Mint = ac_library::ModInt1000000007;

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut dp = vec![Mint::new(0); n + 1];
    dp[0] = Mint::new(1);
    for i in 0..n {
        for j in (i + 1)..=(i + 3).min(n) {
            if s[j - 1] == '.' {
                dp[j] = dp[j] + dp[i];
            }
        }
    }
    let result = dp[n];
    println!("{result}");
}
