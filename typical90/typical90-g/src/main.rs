// -*- coding:utf-8-unix -*-

use proconio::input;

// 007 - CP Classes（★3）
// https://atcoder.jp/contests/typical90/tasks/typical90_g

fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
        q: usize,
        bs: [i64; q],
    };
    a.sort();
    for b in bs {
        let i = match a.binary_search(&b) {
            Ok(i) | Err(i) => i,
        };
        let score = if i == 0 {
            (b - a[0]).abs()
        } else if i == n {
            (b - a[i - 1]).abs()
        } else {
            (b - a[i]).abs().min((b - a[i - 1]).abs())
        };
        println!("{}", score);
    }
}
