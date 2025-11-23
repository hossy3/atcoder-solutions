use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        mut s: [Chars; n],
    }
    let result = s
        .iter()
        .sorted_by_key(|&s| s.len())
        .map(|v| v.iter().join(""))
        .join("");
    println!("{result}");
}
