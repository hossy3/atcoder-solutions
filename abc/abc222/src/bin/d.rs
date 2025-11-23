use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
    }

    const MOD: i64 = 998244353;
    let mut dp = vec![vec![0i64; 3002]; n + 1];
    dp[0][0] = 1;
    for i in 0..n {
        let a = a[i];
        let b = b[i];
        let mut acc = vec![0i64; 3002];
        for j in 0..=b {
            let a = j.max(a);
            if a <= b {
                let c = dp[i][j];
                acc[a] += c;
                acc[b + 1] -= c;
            }
        }
        for j in 0..=b {
            acc[j + 1] = (acc[j] + acc[j + 1]) % MOD;
        }
        dp[i + 1] = acc;
    }

    let result = dp[n].iter().sum::<i64>() % MOD;
    println!("{}", result);
}
