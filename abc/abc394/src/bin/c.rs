use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
    }

    let n = s.len();
    for i in (0..(n - 1)).rev() {
        if s[i] == 'W' && s[i + 1] == 'A' {
            s[i] = 'A';
            s[i + 1] = 'C';
        }
    }
    let result = s.iter().join("");
    println!("{result}");
}
