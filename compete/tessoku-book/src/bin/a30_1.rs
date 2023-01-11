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

fn combination_mod(n: usize, k: usize, fact: &[usize], fact_inv: &[usize], m: usize) -> usize {
    (((fact[n] * fact_inv[k]) % m) * fact_inv[n - k]) % m
}

fn main() {
    input! {
        n: usize,
        r: usize,
    }
    const MOD: usize = 1000000007;

    let mut fact = vec![1; n + 1];
    for i in 1..=n {
        fact[i] = (fact[i - 1] * i) % MOD;
    }

    let mut fact_inv = vec![0; n + 1];
    fact_inv[n] = modinv(fact[n], MOD);
    for i in 0..n {
        fact_inv[n - i - 1] = (fact_inv[n - i] * (n - i)) % MOD;
    }

    let result = combination_mod(n, r, &fact, &fact_inv, MOD);
    println!("{}", result);
}
