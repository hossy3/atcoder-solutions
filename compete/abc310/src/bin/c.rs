use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }
    let mut set = HashSet::new();
    for s in s {
        let mut s0 = s.clone();
        s0.reverse();
        set.insert(s.min(s0));
    }
    let count = set.len();
    println!("{}", count);
}
