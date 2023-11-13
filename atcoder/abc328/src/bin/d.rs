use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let mut v = vec![];
    let mut i = 0;
    while i < s.len() {
        if s[i..].starts_with(&['A', 'B', 'C']) {
            i += 3;
        } else if v.ends_with(&['A']) & s[i..].starts_with(&['B', 'C']) {
            v.pop();
            i += 2;
        } else if v.ends_with(&['A', 'B']) & s[i..].starts_with(&['C']) {
            v.pop();
            v.pop();
            i += 1;
        } else {
            v.push(s[i]);
            i += 1;
        }
    }
    let result = v.iter().join("");
    println!("{result}");
}
