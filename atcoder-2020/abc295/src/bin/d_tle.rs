use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: Chars,
    }

    let mut count = 0usize;
    for i in 0..(n.len()) {
        let mut s = HashSet::new();
        for &c in &n[i..] {
            if s.remove(&c) {
                if s.is_empty() {
                    count += 1;
                }
            } else {
                s.insert(c);
            }
        }
    }
    println!("{}", count);
}
