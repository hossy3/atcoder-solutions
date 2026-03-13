use itertools::Itertools;
use proconio::{input, marker::Chars};

fn c2i(c: char) -> usize {
    (c as usize) - ('a' as usize)
}

fn main() {
    input! {
        h: usize,
        w: usize,
        g: [Chars; h],
    }

    let mut rows = vec![[0usize; 26]; h];
    let mut cols = vec![[0usize; 26]; w];
    for i in 0..h {
        for j in 0..w {
            let k = c2i(g[i][j]);
            rows[i][k] += 1;
            cols[j][k] += 1;
        }
    }

    let mut results = vec![];
    for i in 0..h {
        for j in 0..w {
            let k = c2i(g[i][j]);
            if rows[i][k] == 1 && cols[j][k] == 1 {
                results.push(g[i][j]);
            }
        }
    }

    println!("{}", results.iter().join(""));
}
