use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let k = s.len() / 2;
    println!("{}{}", s[..k].iter().join(""), s[(k + 1)..].iter().join(""));
}
