use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        a: usize,
        b: usize,
        s: Chars,
    }
    println!("{}", s[a..(s.len() - b)].iter().join(""));
}
