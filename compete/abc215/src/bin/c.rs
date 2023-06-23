use std::collections::HashSet;

use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
        k: usize,
    }
    s.sort();
    let mut set = HashSet::new();
    for a in s.iter().permutations(s.len()) {
        let str = a.iter().join("");
        set.insert(str.clone());
        if set.len() == k {
            println!("{}", str);
            return;
        }
    }
}
