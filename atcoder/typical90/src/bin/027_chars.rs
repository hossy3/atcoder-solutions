use proconio::{input, marker::Chars};
use std::collections::HashSet;

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
