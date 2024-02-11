use itertools::Itertools;
use proconio::{input, marker::Chars};
use std::collections::BTreeSet;

fn to_index(c: char) -> usize {
    c as usize - 'a' as usize
}

fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars,
    }

    let mut indexes = vec![BTreeSet::new(); 26];
    for (i, &c) in s.iter().enumerate() {
        indexes[to_index(c)].insert(i);
    }

    let mut results = vec![];
    let mut cur = 0;
    for i in 0..k {
        for c in 'a'..='z' {
            if let Some(j) = indexes[to_index(c)].range(cur..=(n + i - k)).next() {
                results.push(c);
                cur = j + 1;
                break;
            };
        }
    }

    let result = results.iter().join("");
    println!("{result}");
}
