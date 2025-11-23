use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let mut set = HashSet::new();
    for i in 0..(s.len()) {
        for j in i..(s.len()) {
            let x = s[i..=j].to_vec();
            set.insert(x);
        }
    }
    let result = set.len();
    println!("{result}");
}
