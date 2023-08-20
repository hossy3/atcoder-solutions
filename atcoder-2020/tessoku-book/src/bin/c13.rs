use std::{collections::HashMap, mem::swap};

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
        a: [usize; n],
    }

    const MOD: usize = 1000000007;
    let mut m = HashMap::new();
    for &a in &a {
        let x = a % MOD;
        *m.entry(x).or_insert(0) += 1;
    }

    let mut count = 0usize;
    for (&k, &v) in &m {
        let k0 = (p * modinv(k, MOD)) % MOD;
        if let Some(v0) = m.get(&k0) {
            if k == k0 {
                count += v * (v - 1) / 2;
            } else if k > k0 {
                count += v * v0;
            }
        }
    }
    println!("{}", count);
}
