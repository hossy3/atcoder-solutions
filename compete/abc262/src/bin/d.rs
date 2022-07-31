use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    const P: usize = 998_244_353;
    let mut count = 0;
    for i in 1..=n {
        let mut dp = vec![vec![vec![0usize; i]; i + 2]; n + 1];
        dp[0][0][0] = 1;
        for j in 0..n {
            for k in 0..=i {
                for l in 0..i {
                    let m = (l + a[j]) % i;
                    dp[j + 1][k][l] = (dp[j + 1][k][l] + dp[j][k][l]) % P;
                    dp[j + 1][k + 1][m] = (dp[j + 1][k + 1][m] + dp[j][k][l]) % P;
                }
            }
        }
        count = (count + dp[n][i][0]) % P;
    }
    println!("{}", count);
}
