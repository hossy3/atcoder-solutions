use std::iter;

use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
        t: Chars,
    }

    let n = s.len();
    let count = iter::zip(&s, &t).filter(|(s, t)| s != t).count();
    println!("{count}");

    'outer: for _ in 0..count {
        for i in 0..n {
            if s[i] > t[i] {
                s[i] = t[i];
                println!("{}", s.iter().join(""));
                continue 'outer;
            }
        }

        for i in (0..n).rev() {
            if s[i] < t[i] {
                s[i] = t[i];
                println!("{}", s.iter().join(""));
                continue 'outer;
            }
        }
    }
}
