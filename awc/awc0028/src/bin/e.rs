use proconio::input;

type Mint = ac_library::ModInt1000000007;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
        b: [usize; k],
    }

    let mut dp = vec![vec![Mint::new(0); k + 1]; n + 1];
    dp[0][0] = Mint::new(1);

    for i in 0..n {
        for j in 0..k {
            if a[i] == b[j] {
                dp[i + 1][j + 1] = dp[i + 1][j + 1] + dp[i][j];
            }
        }
        for j in 0..=k {
            dp[i + 1][j] = dp[i + 1][j] + dp[i][j];
        }
    }

    let result = dp[n][k];
    println!("{result}");
}
