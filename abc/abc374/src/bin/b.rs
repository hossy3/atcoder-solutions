use std::iter;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    for (i, (s0, t0)) in iter::zip(&s, &t).enumerate() {
        if s0 != t0 {
            println!("{}", i + 1);
            return;
        }
    }

    let result = if s.len() == t.len() {
        0
    } else {
        s.len().min(t.len()) + 1
    };
    println!("{result}");
}
