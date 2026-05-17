use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        n: usize,
    }
    println!("{}", s[n..(s.len() - n)].iter().join(""));
}
