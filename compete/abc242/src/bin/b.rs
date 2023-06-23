use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
    }
    s.sort();
    let result = s.iter().join("");
    println!("{}", result);
}
