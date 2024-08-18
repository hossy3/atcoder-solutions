use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut x: Chars,
    }
    while let Some(&c) = x.last() {
        if c != '0' {
            break;
        }
        x.pop();
    }
    let c = *x.last().unwrap();
    if c == '.' {
        x.pop();
    }
    println!("{}", x.iter().join(""));
}
