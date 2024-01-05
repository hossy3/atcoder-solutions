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

fn modmul(a: &[usize], m: usize) -> usize {
    a.iter().fold(1, |acc, x| (acc * x) % m)
}

fn main() {
    input! {
        r: usize,
        g: usize,
        b: usize,
        k: usize,
    }

    const MOD: usize = 998_244_353;

    let r = r - k;
    let g = g - k;
    let n = r + g + b + k;

    let mut fact = vec![1; n + 1];
    for i in 1..=n {
        fact[i] = (fact[i - 1] * i) % MOD;
    }

    let mut fact_inv = vec![0; n + 1];
    fact_inv[n] = modinv(fact[n], MOD);
    for i in 0..n {
        fact_inv[n - i - 1] = (fact_inv[n - i] * (n - i)) % MOD;
    }

    let mut count = modmul(
        &[fact[g + b + k], fact_inv[g], fact_inv[b], fact_inv[k]],
        MOD,
    );
    count = modmul(&[count, fact[r + b + k], fact_inv[r], fact_inv[b + k]], MOD);
    println!("{}", count);
}
