use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    if s[s.len() - 1] == 'e' {
        println!("{}r", s.iter().join(""));
    } else {
        println!("{}er", s.iter().join(""));
    }
}
