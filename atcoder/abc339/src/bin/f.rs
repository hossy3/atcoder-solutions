use std::{collections::HashMap, str::FromStr};

use num_bigint::BigUint;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [String; n],
    }

    let m = BigUint::from(67_280_421_310_721_u128)
        * BigUint::from(1000000007_u128)
        * BigUint::from(998244353_u128);
    let mut v: Vec<BigUint> = Vec::with_capacity(n);
    for str in a {
        let x: Result<BigUint, _> = FromStr::from_str(&str);
        if let Ok(x) = x {
            v.push(x % m.clone());
        }
    }
    let mut map = HashMap::new();
    for x in &v {
        *map.entry(x).or_insert(0usize) += 1;
    }

    let mut result = 0;
    for i in &v {
        for j in &v {
            let x = (i * j) % m.clone();
            if let Some(y) = map.get(&x) {
                result += y;
            }
        }
    }
    println!("{result}");
}
