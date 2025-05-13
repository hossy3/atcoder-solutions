use std::collections::HashMap;

use proconio::input;

const MOD: usize = 998244353;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    }

    let mut hash = HashMap::new();
    hash.insert((0usize, 0usize, 0usize, 0usize), 1usize);

    for _ in 0..(a + b + c + d) {
        let mut hash0 = HashMap::new();
        for (&(a0, b0, c0, d0), &count) in &hash {
            if a0 < a {
                let key = (a0 + 1, b0, c0, d0);
                let x = *hash0.get(&key).unwrap_or(&0);
                let x = (x + count) % MOD;
                hash0.insert(key, x);
            }

            if b0 < b {
                let key = (a0, b0 + 1, c0, d0);
                let x = *hash0.get(&key).unwrap_or(&0);
                let x = (x + count) % MOD;
                hash0.insert(key, x);
            }

            if c0 < c && a0 == a {
                let key = (a0, b0, c0 + 1, d0);
                let x = *hash0.get(&key).unwrap_or(&0);
                let x = (x + count) % MOD;
                hash0.insert(key, x);
            }

            if d0 < d && a0 == a && b0 == b {
                let key = (a0, b0, c0, d0 + 1);
                let x = *hash0.get(&key).unwrap_or(&0);
                let x = (x + count) % MOD;
                hash0.insert(key, x);
            }
        }

        hash = hash0;
    }

    let Some(result) = hash.get(&(a, b, c, d)) else {
        unreachable!()
    };
    println!("{result}");
}
