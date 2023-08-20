use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: Chars,
    }
    let mut v = VecDeque::new();
    for &c in &n {
        v.push_back(c);
    }

    while let Some(c) = v.back() {
        if *c != '0' {
            break;
        }
        v.pop_back();
    }
    while v.len() > 1 {
        let c0 = v.front().unwrap();
        let c1 = v.back().unwrap();
        if *c0 != *c1 {
            break;
        }
        v.pop_front();
        v.pop_back();
    }
    let yes = v.len() <= 1;
    println!("{}", if yes { "Yes" } else { "No" });
}
