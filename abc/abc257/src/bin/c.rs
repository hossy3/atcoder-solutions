// -*- coding:utf-8-unix -*-

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
        w: [i32; n],
    }
    let mut v = vec![(0i32, '0'); n];
    for i in 0..n {
        v[i] = (w[i], s[i]);
    }
    v.sort();

    let mut score = 0;
    for i in 0..n {
        if v[i].1 == '1' {
            score += 1;
        }
    }

    let mut s = score;
    for i in 0..n {
        s += if v[i].1 == '0' { 1 } else { -1 };
        if i == n - 1 || v[i].0 != v[i + 1].0 {
            score = score.max(s);
        }
    }
    println!("{}", score);
}
