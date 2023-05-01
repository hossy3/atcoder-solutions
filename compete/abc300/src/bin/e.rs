use std::collections::BTreeMap;

use proconio::input;

fn modpow(n: usize, k: usize, m: usize) -> usize {
    match k {
        0 => 1,
        1 => n,
        _ => {
            let x = modpow(n, k / 2, m);
            if k % 2 == 0 {
                (x * x) % m
            } else {
                (((x * x) % m) * n) % m
            }
        }
    }
}

fn modinv(a: usize, m: usize) -> usize {
    modpow(a, m - 2, m)
}

fn f(n: usize) -> usize {
    const MOD: usize = 998244353;

    let mut map = BTreeMap::new();
    map.insert(1usize, 1usize);
    let mut k = 1;
    while let Some(_) = map.range(0..n).next() {
        let mut map0 = BTreeMap::new();
        for (i, count) in map {
            for j in 2..=6 {
                let i = if i == n { i } else { i * j };
                if i <= n {
                    let x = (map0.get(&i).unwrap_or(&0) + count) % MOD;
                    map0.insert(i, x);
                }
            }
        }
        k = (k * 5) % MOD;
        map = map0;
    }

    let result = (map.get(&n).unwrap_or(&0) * modinv(k, MOD)) % MOD;
    result
}

#[test]
fn test_func() {
    assert_eq!(f(6), 239578645);
    assert_eq!(f(7), 0);
    assert_eq!(f(300), 183676961);
    assert_eq!(f(979552051200000000), 812376310);
}

fn main() {
    input! {
        n: usize,
    }
    let result = f(n);
    println!("{}", result);
}
