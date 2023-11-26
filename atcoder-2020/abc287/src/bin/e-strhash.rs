use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
    }

    const MOD: usize = 1000000007;
    const SIZE: usize = 26;
    const LEN_MAX: usize = 500_000;

    let mut pow = Vec::with_capacity(LEN_MAX + 1);
    pow.push(1);
    for i in 0..LEN_MAX {
        pow.push((pow[i] * SIZE) % MOD);
    }

    let mut v = Vec::with_capacity(n);
    let mut s0 = HashSet::new(); // exist once
    let mut s1 = HashSet::new(); // exist twice or more
    for _ in 0..n {
        input! {
            s: Chars,
        }
        let mut v0 = Vec::with_capacity(s.len());
        v0.push(0usize);

        for i in 0..s.len() {
            let x = (v0[i] + (s[i] as u8 - b'a') as usize * pow[i + 1]) % MOD;
            v0.push(x);
            if s0.contains(&(i, x)) {
                s1.insert((i, x));
            } else {
                s0.insert((i, x));
            }
        }
        v.push(v0);
    }

    for v0 in v {
        let mut i = 0usize;
        while i + 1 < v0.len() && s1.contains(&(i, v0[i + 1])) {
            i += 1;
        }
        println!("{}", i);
    }
}
