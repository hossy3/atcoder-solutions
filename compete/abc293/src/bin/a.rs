use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
    }
    let len = s.len() / 2;
    for i in 0..len {
        s.swap(2 * i, 2 * i + 1);
    }
    let result = s.iter().join("");
    println!("{}", result);
}
