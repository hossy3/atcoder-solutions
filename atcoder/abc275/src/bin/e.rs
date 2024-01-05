use std::mem::swap;

use proconio::input;

fn modinv(a: usize, m: usize) -> usize {
    let m = m as i64;
    let mut a = a as i64;
    let mut b = m;
    let mut u = 1i64;
    let mut v = 0i64;
    while b > 0 {
        let t = a / b;
        a -= t * b;
        swap(&mut a, &mut b);
        u -= t * v;
        swap(&mut u, &mut v);
    }
    u %= m;
    if u < 0 {
        u += m;
    }
    u as usize
}

fn modmul(n: usize, k: usize, m: usize) -> usize {
    let mut result = 1;
    for _ in 0..k {
        result = (result * n) % m;
    }
    result
}

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
    }
    const MOD: usize = 998244353;
    let mut dp = vec![vec![0; n + 1]; 2];
    dp[0][0] = 1;
    for i in 0..k {
        let prev = i % 2;
        let cur = 1 - prev;
        for j in 0..=n {
            dp[cur][j] = 0;
        }
        for j in 0..n {
            let c = dp[prev][j];
            if c == 0 {
                continue;
            }
            for x in (j + 1)..=(j + m) {
                let x = if x > n { n * 2 - x } else { x };
                dp[cur][x] = (dp[cur][x] + c) % MOD;
            }
        }
        dp[cur][n] = (dp[cur][n] + dp[prev][n] * m) % MOD;
    }
    let x = dp[k % 2][n];
    let y = modmul(m, k, MOD);
    let result = (x * modinv(y, MOD)) % MOD;
    println!("{}", result);
}
