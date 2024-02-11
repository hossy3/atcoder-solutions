use proconio::input;
use std::collections::HashSet;

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
