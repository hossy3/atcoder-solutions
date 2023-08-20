use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        s: [String; n],
    }
    let mut set = BTreeSet::new();
    for s in s {
        set.insert(s);
        if set.len() == k {
            break;
        }
    }
    for s in set {
        println!("{}", s);
    }
}
