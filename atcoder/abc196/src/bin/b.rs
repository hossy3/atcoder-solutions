use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        x: Chars,
    }
    if let Some(i) = x.iter().position(|&c| c == '.') {
        println!("{}", x[0..i].iter().join(""));
    } else {
        println!("{}", x.iter().join(""));
    }
}
