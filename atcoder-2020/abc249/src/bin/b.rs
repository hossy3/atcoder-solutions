use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn solve(s: &[char]) -> bool {
    if !s.iter().any(|&c| b'a' <= c as u8 && c as u8 <= b'z') {
        return false;
    }
    if !s.iter().any(|&c| b'A' <= c as u8 && c as u8 <= b'Z') {
        return false;
    }
    let mut set = HashSet::<char>::new();
    for c in s {
        if set.contains(c) {
            return false;
        }
        set.insert(*c);
    }
    true
}

fn main() {
    input! {
        s: Chars,
    }
    let yes = solve(&s);
    println!("{}", if yes { "Yes" } else { "No" });
}
