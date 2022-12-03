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

fn main() {
    input! {
        n: usize,
        p: usize,
    }
    const MOD: usize = 998_244_353;
    let inv100 = modinv(100, MOD);
    let mut a = vec![0; n + 1];
    let pc = (p * inv100) % MOD;
    let pn = ((100 - p) * inv100) % MOD;
    a[1] = 1;
    for i in 2..=n {
        a[i] = (a[i - 2] * pc + a[i - 1] * pn + 1) % MOD;
    }
    println!("{}", a[n]);
}
