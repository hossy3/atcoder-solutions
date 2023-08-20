// -*- coding:utf-8-unix -*-

use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        m: usize,
        ss: [i64; n - 1],
        xs: [i64; m],
    }

    let mut bs = vec![0; n];
    for i in 1..n {
        bs[i] = ss[i - 1] - bs[i - 1];
    }

    let mut map = HashMap::new();
    for i in 0..n {
        for j in 0..m {
            let k: i64 = if i % 2 == 0 { 1 } else { -1 };
            let c = k * (xs[j] - bs[i]);
            if let Some(&v) = map.get(&c) {
                map.insert(c, v + 1);
            } else {
                map.insert(c, 1);
            }
        }
    }

    if let Some(&score) = map.values().max() {
        println!("{}", score);
    }
}
