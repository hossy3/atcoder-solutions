use std::collections::HashSet;

use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars,
    }

    let mut result = 0usize;
    let mut set = HashSet::new();
    'outer: for v in s.iter().permutations(s.len()) {
        if set.contains(&v) {
            continue;
        }
        set.insert(v.clone());
        for i in 0..=(n - k) {
            if (0..(k / 2)).all(|j| v[i + j] == v[i + k - j - 1]) {
                continue 'outer;
            }
        }
        result += 1;
    }
    println!("{result}");
}
