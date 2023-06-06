use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
    }

    const MOD: usize = 998244353;
    let mut dp = vec![vec![0usize; 3001]; n + 1];
    dp[0][0] = 1;
    for i in 0..n {
        let a = a[i];
        let b = b[i];
        for j in 0..=b {
            let c = dp[i][j];
            for k in (j.max(a))..=b {
                dp[i + 1][k] += c;
            }
        }
        for j in 0..=b {
            dp[i + 1][j] %= MOD;
        }
    }

    let result = dp[n].iter().sum::<usize>() % MOD;
    println!("{}", result);
}
