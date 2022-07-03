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
    for query in queries {
        if query.0 == 1 {
            index = (index + n - query.1) % n;
        } else {
            println!("{}", s[(index + query.1 - 1) % n]);
        }
    }
}
