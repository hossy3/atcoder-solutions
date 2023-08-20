// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    }
    let mut score = 0.max(n - 3);
    if n > 0 && a[n - 1] >= 4 {
        score += 1;
    }
    if n > 1 && a[n - 1] + a[n - 2] >= 4 {
        score += 1;
    }
    if n > 2 && a[n - 1] + a[n - 2] + a[n - 3] >= 4 {
        score += 1;
    }
    println!("{}", score);
}
