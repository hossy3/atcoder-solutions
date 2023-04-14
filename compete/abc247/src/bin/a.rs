use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    println!("0{}", s[0..3].iter().join(""));
}
