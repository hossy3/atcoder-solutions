use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        s: Chars,
    }
    for (i, &c) in s.iter().enumerate() {
        if c != 'o' {
            println!("{}", s[i..].iter().join(""));
            return;
        }
    }
    println!();
}
