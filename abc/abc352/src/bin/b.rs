use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }
    let mut v = vec![];
    let mut i = 0;
    for c in s {
        i = i + t[i..].iter().position(|&c0| c0 == c).unwrap() + 1;
        v.push(i);
    }
    let result = v.iter().join(" ");
    println!("{result}");
}
