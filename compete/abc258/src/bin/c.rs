// -*- coding:utf-8-unix -*-

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        q: usize,
        s: Chars,
        queries: [(usize, usize); q],
    }
    let mut index = 0usize;
    for (t, x) in queries {
        if t == 1 {
            index = (index + n - x) % n;
        } else {
            println!("{}", s[(index + x - 1) % n]);
        }
    }
}
