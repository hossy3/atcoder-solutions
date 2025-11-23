// -*- coding:utf-8-unix -*-

use proconio::input;
use std::cmp;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; k],
        xy: [(i64, i64); n],
    }
    let mut strength_all = 0;
    for i in 0..n {
        let p0 = xy[i];
        let mut strength = 1_000_000_000_000;
        for j in 0..k {
            let p1 = xy[a[j] - 1];
            strength = cmp::min(strength, (p0.0 - p1.0).pow(2) + (p0.1 - p1.1).pow(2));
        }

        strength_all = cmp::max(strength_all, strength);
    }
    println!("{}", (strength_all as f64).sqrt());
}
