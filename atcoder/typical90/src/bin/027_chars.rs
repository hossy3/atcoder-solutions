use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }
    let mut h = HashSet::new();
    for (i, s) in s.iter().enumerate() {
        if h.insert(s) {
            println!("{}", i + 1);
        }
    }
}
