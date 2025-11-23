use itertools::Itertools;
use proconio::{input, marker::Chars};

fn f(s: &[char], t: &[char]) -> usize {
    let n = s.len().min(t.len());
    for i in 0..n {
        if s[i] != t[i] {
            return i;
        }
    }
    n
}

fn main() {
    input! {
        n: usize,
        t: Chars,
        s: [Chars; n],
    }
    let mut results = vec![];
    for (i, s) in s.iter().enumerate() {
        let j = f(s, &t);
        if s.len() == t.len() {
            if j == s.len() || s[(j + 1)..] == t[(j + 1)..] {
                results.push(i + 1);
            }
        } else if s.len() + 1 == t.len() {
            if s[j..] == t[(j + 1)..] {
                results.push(i + 1);
            }
        } else if s.len() == t.len() + 1 {
            if s[(j + 1)..] == t[j..] {
                results.push(i + 1);
            }
        }
    }
    println!("{}", results.len());
    println!("{}", results.iter().join(" "));
}
