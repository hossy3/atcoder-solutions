use std::collections::HashMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        xy: [(usize, usize); n],
        s: Chars,
    }
    let mut map = HashMap::new();
    for (&(x, y), &c) in xy.iter().zip(s.iter()) {
        let z = map.entry(y).or_insert((1_000_000_000, 0));
        if c == 'L' {
            z.1 = z.1.max(x);
        } else {
            z.0 = z.0.min(x);
        }
    }
    let yes = map.iter().any(|(_, &(a, b))| a < b);
    println!("{}", if yes { "Yes" } else { "No" });
}
