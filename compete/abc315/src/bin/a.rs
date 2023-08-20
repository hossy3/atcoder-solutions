use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let result = s
        .iter()
        .filter(|&&c| c != 'a' && c != 'e' && c != 'i' && c != 'o' && c != 'u')
        .into_iter()
        .join("");
    println!("{result}");
}
