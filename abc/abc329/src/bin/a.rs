use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let result = s.iter().join(" ");
    println!("{result}");
}
