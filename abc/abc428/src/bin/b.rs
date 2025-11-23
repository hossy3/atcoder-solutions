use std::collections::BTreeMap;

use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars,
    }

    let mut map = BTreeMap::new();
    for i in 0..(n - k + 1) {
        let s: String = s[i..(i + k)].iter().collect();
        *map.entry(s).or_insert(0) += 1;
    }

    // 一番多い数を調べる
    let max_count = map.iter().map(|(_, &count)| count).max().unwrap();
    let mut results = vec![];
    for (s, &count) in &map {
        if count == max_count {
            results.push(s);
        }
    }

    println!("{max_count}");
    println!("{}", results.iter().join(" "));
}
