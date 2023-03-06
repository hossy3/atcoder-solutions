use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let result = s.iter().map(|&c| c.to_uppercase()).join("");
    println!("{}", result);
}
