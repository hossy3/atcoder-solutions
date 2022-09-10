use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
        e: usize,
    }
    let mut s = HashSet::new();
    s.insert(a);
    s.insert(b);
    s.insert(c);
    s.insert(d);
    s.insert(e);
    let count = s.len();
    println!("{}", count);
}
