use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let i = s.iter().rposition(|&c| c == '.').unwrap();
    let result = s[(i + 1)..].iter().join("");
    println!("{result}");
}
