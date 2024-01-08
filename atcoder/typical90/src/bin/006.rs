use std::collections::BTreeMap;

use itertools::Itertools;
use proconio::{input, marker::Chars};

fn update_map(map: &mut BTreeMap<char, usize>, s: &[char], i: usize) {
    for c in 'a'..='z' {
        if let Some(&j) = map.get(&c) {
            if i < j {
                continue;
            }
            if let Some(y) = s[(i + 1)..].iter().position(|&x| x == c) {
                map.insert(c, i + 1 + y);
            } else {
                map.remove(&c);
            };
        }
    }
}

fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars,
    }

    let mut map = BTreeMap::new(); // (char, index)
    for c in 'a'..='z' {
        if let Some(i) = s.iter().position(|&x| x == c) {
            map.insert(c, i);
        }
    }

    let mut results = vec![];
    for i in 0..k {
        for c in 'a'..='z' {
            if let Some(&j) = map.get(&c) {
                if n - j < k - i {
                    continue;
                }
                results.push(c);
                update_map(&mut map, &s, j);
                break;
            }
        }
    }

    let result = results.iter().join("");
    println!("{result}");
}
