use std::{collections::BTreeMap, mem};

use itertools::Itertools;
use proconio::input;

type Mint = ac_library::ModInt998244353;

fn combination(n: usize, r: usize) -> Mint {
    let r = r.min(n - r);
    (1..=r).fold(Mint::new(1), |acc, x| {
        acc * Mint::new(n - x + 1) / Mint::new(x)
    })
}

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }

    let mut map = BTreeMap::new();
    for (i, &x) in a.iter().enumerate() {
        map.entry(x).or_insert(vec![]).push(i);
    }
    for (_, v) in map.iter_mut() {
        v.sort();
    }

    let mut results = vec![Mint::new(0); n];
    results[0] = Mint::new(n);
    
    // 同じ値
    for (_, v) in map.iter() {
        let len = v.len();
        for i in 1..len {
            results[i] += combination(len, i + 1);
        }
    }

    // 違う値
    for (&x0, v0) in map.iter() {
        let mut dp = vec![0; n];
        for &i in v0 {
            dp[i] = 1;
        }
        for i in 1..n {
            dp[i] += dp[i - 1];
        }

        for (&x1, _) in map.iter() {
            if x0 == x1 {
                continue;
            }
            let mut dp0 = dp.clone();
            let k = x1 - x0;
            let mut x1 = x1;
            let mut j = 1;
            loop {
                let Some(v1) = map.get(&x1) else { break; };
                let mut v2 = vec![false; n];
                for &i in v1 {
                    v2[i] = true;
                }
                let mut dp1 = vec![0; n];
                for i in 1..n {
                    dp1[i] = dp1[i - 1];
                    if v2[i] {
                        dp1[i] += dp0[i];
                    }
                }
                if dp1[n - 1] == 0 {
                    break;
                }
                results[j] += dp1[n - 1];
                mem::swap(&mut dp0, &mut dp1);
                x1 += k;
                j += 1;
            }
        }
    }

    let result = results.iter().join(" ");
    println!("{result}");
}
