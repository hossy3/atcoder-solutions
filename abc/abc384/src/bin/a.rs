use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        _: usize,
        c1: char,
        c2: char,
        s: Chars,
    }
    let result = s.iter().map(|&c| if c == c1 { c1 } else { c2 }).join("");
    println!("{result}");
}
