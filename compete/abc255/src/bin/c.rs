// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        x: i64,
        a: i64,
        d: i64,
        n: i64,
    }

    let mut result = (x - a).abs();
    let y = (x - (a + d * (n - 1))).abs();
    result = result.min(y);
    if d != 0 {
        let z = (x - a) / d;
        if 0 < z && z < n - 1 {
            let y = (x - (a + d * z)).abs();
            result = result.min(y);
        }
        if 0 <= z && z < n - 2 {
            let y = (x - (a + d * (z + 1))).abs();
            result = result.min(y);
        }
    }

    println!("{}", result);
}
