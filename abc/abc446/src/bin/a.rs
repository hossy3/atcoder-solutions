use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
    }
    s[0] = s[0].to_ascii_lowercase();
    println!("Of{}", s.iter().join(""));
}
