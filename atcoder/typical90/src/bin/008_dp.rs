use proconio::{input, marker::Chars};

type Mint = ac_library::ModInt1000000007;

fn main() {
    input! {
        n: usize,
        s: Chars,
    };

    const M: usize = 8; // "atcoder".len() + 1
    let mut dp = vec![vec![Mint::new(0); M]; n + 1];
    dp[0][0] = Mint::new(1);

    let s0: Vec<_> = "atcoder".chars().collect();

    for (i, &c) in s.iter().enumerate() {
        for j in 0..M {
            let x = dp[i][j];
            dp[i + 1][j] += x;
        }
        for (j, &c0) in s0.iter().enumerate() {
            if c == c0 {
                let x = dp[i][j];
                dp[i + 1][j + 1] += x;
            }
        }
    }

    let result = dp[n][M - 1];
    println!("{result}");
}
