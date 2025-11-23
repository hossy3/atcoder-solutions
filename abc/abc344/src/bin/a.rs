use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let l = s.iter().position(|&c| c == '|').unwrap();
    let r = s.iter().rposition(|&c| c == '|').unwrap();
    let result0 = s[..l].iter().join("");
    let result1 = s[(r + 1)..].iter().join("");
    println!("{result0}{result1}");
}
