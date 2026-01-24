use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }
    let mut set = HashSet::new();
    for s in s {
        set.insert(s.clone());
    }
    let result = set.len();
    println!("{result}");
}
