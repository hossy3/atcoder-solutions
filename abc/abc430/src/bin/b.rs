use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
    }

    let mut set = HashSet::new();
    for i in 0..=(n - m) {
        for j in 0..=(n - m) {
            let mut v = Vec::with_capacity(m * m);
            for i0 in 0..m {
                for j0 in 0..m {
                    v.push(s[i + i0][j + j0]);
                }
            }
            set.insert(v);
        }
    }

    let result = set.len();
    println!("{result}");
}
