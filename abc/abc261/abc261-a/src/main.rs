// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        l1: i64,
        r1: i64,
        l2: i64,
        r2: i64,
    }
    let l = l1.max(l2);
    let r = r1.min(r2);
    let score = (r - l).max(0);
    println!("{}", score);
}
