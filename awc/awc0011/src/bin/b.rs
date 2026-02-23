use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        _w: usize,
        k: usize,
        c1: char,
        c2: char,
        s: [Chars; h],
    }

    for s in &s {
        let mut v = vec![];
        for &c in s {
            if c == '#' {
                v.append(&mut vec![c1; k]);
            } else {
                v.append(&mut vec![c2; k]);
            }
        }
        let result = v.iter().join("");
        for _ in 0..k {
            println!("{result}");
        }
    }
}
