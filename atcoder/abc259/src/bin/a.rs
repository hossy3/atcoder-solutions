// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        _: i64,
        m: i64,
        x: i64,
        t: i64,
        d: i64,
    }
    let y = t - 0.max(x - m) * d;
    println!("{}", y);
}
