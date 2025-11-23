use std::collections::HashSet;

use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        r: isize,
        c: isize,
        s: Chars,
    }

    let mut results = vec![];
    let mut set = HashSet::new();
    let mut r0 = 0isize; // offset
    let mut c0 = 0isize; // offset

    set.insert((r0, c0));

    for &x in &s {
        match x {
            'N' => {
                r0 += 1;
            }
            'W' => {
                c0 += 1;
            }
            'S' => {
                r0 -= 1;
            }
            'E' => {
                c0 -= 1;
            }
            _ => unreachable!(),
        }
        set.insert((r0, c0));

        if set.contains(&(r + r0, c + c0)) {
            results.push(1);
        } else {
            results.push(0);
        }
    }

    let result = results.iter().join("");
    println!("{result}");
}
