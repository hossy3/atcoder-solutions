// -*- coding:utf-8-unix -*-

use proconio::{input, marker::Chars};

fn f(chars: &[char]) -> usize {
    let max = chars.len();
    let c = chars[0];
    for i in 1..max {
        if chars[i] != c {
            return i;
        }
    }
    return max;
}

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }
    let mut i = 0;
    let mut j = 0;
    let yes = loop {
        if i == s.len() && j == t.len() {
            break true;
        }
        if i == s.len() && j < t.len() {
            break false;
        }
        if i < s.len() && j == t.len() {
            break false;
        }
        if s[i] != t[j] {
            break false;
        }

        let i0 = f(&s[i..]);
        let j0 = f(&t[j..]);
        if i0 == 1 && j0 > 1{
            break false;
        }
        if i0 > 1 && i0 > j0 {
            break false;
        }
        i += i0;
        j += j0;
    };
    println!("{}", if yes { "Yes" } else { "No" });
}
