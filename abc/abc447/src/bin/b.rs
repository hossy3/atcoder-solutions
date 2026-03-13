use std::collections::HashMap;

use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut map = HashMap::new();
    for &x in &s {
        *map.entry(x).or_insert(0usize) += 1;
    }
    let max = *map.values().max().unwrap();

    let mut results = vec![];
    for &x in &s {
        if let Some(&count) = map.get(&x) {
            if count < max {
                results.push(x);
            }
        }
    }

    println!("{}", results.iter().join(""));
}
