use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }
    let mut h = HashSet::<String>::new();
    for i in 0..n {
        if h.insert(s[i].clone()) {
            println!("{}", i + 1);
        }
    }
}
