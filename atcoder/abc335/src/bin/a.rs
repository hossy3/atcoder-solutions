use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let result = s[..(s.len() - 1)].iter().join("");
    println!("{result}4");
}
