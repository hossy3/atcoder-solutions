use std::mem::swap;

use proconio::input;

const MOD: usize = 998244353;

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
        m: usize,
        k: usize,
        uv: [(usize, usize); m],
    }

    let mut nodes = vec![0; n];
    for (u, v) in uv {
        nodes[u - 1] = 1 - nodes[u - 1];
        nodes[v - 1] = 1 - nodes[v - 1];
    }
    let odd_count = nodes.iter().filter(|&&x| x == 1).count();
    let even_count = n - odd_count;

    let mut fact = vec![1; n + 1];
    for i in 1..=n {
        fact[i] = (fact[i - 1] * i) % MOD;
    }

    let mut fact_inv = vec![0; n + 1];
    fact_inv[n] = modinv(fact[n], MOD);
    for i in 0..n {
        fact_inv[n - i - 1] = (fact_inv[n - i] * (n - i)) % MOD;
    }

    let mut count = 0;
    for i in (0..=k).step_by(2) {
        if odd_count >= i && even_count >= k - i {
            count += combination_mod(odd_count, i, &fact, &fact_inv, MOD)
                * combination_mod(even_count, k - i, &fact, &fact_inv, MOD);
            count %= MOD;
        }
    }
    println!("{}", count);
}
