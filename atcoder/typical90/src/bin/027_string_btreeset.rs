use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }
    let mut h = BTreeSet::new();
    for (i, s) in s.iter().enumerate() {
        if h.insert(s) {
            println!("{}", i + 1);
        }
    }
}
