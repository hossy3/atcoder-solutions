use std::{collections::HashMap, str::FromStr};

use num_bigint::BigUint;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [String; n],
    }

    const N0: usize = 1000000007;
    const N1: usize = 998244353;

    let mut v: Vec<(usize, usize)> = Vec::with_capacity(n);
    for str in a {
        let x: Result<BigUint, _> = FromStr::from_str(&str);
        if let Ok(x) = x {
            let x0 = x.clone() % N0;
            let x1 = x % N1;
            v.push((
                x0.to_string().parse().unwrap(),
                x1.to_string().parse().unwrap(),
            ));
        }
    }
    let mut map = HashMap::new();
    for x in &v {
        *map.entry(x).or_insert(0usize) += 1;
    }

    let mut result = 0;
    for &(i0, i1) in &v {
        for &(j0, j1) in &v {
            let x = ((i0 * j0) % N0, (i1 * j1) % N1);
            if let Some(y) = map.get(&x) {
                result += y;
            }
        }
    }
    println!("{result}");
}
