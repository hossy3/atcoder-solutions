use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let result = s.iter().map(|&c| if c == 'A' { 'A' } else { '.' }).join("");
    println!("{result}");
}
