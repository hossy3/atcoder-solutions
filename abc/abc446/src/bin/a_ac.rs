use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
    }
    s[0] = (s[0] as u8 + 'a' as u8 - 'A' as u8) as char;
    println!("Of{}", s.iter().join(""));
}
