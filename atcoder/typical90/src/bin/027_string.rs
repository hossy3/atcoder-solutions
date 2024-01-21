use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }
    let mut h = HashSet::new();
    for (i, s) in s.iter().enumerate() {
        if h.insert(s) {
            println!("{}", i + 1);
        }
    }
}
