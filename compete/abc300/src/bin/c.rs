use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h],
    }
    let n = h.min(w);
    let mut v = vec![0; n];
    for i in 1..(h - 1) {
        for j in 1..(w - 1) {
            if c[i][j] == '#'
                && c[i - 1][j - 1] == '#'
                && c[i - 1][j + 1] == '#'
                && c[i + 1][j - 1] == '#'
                && c[i + 1][j + 1] == '#'
            {
                for k in 2.. {
                    if i + k >= h || j + k >= w || c[i + k][j + k] == '.' {
                        v[k - 2] += 1;
                        break;
                    }
                }
            }
        }
    }
    let result = v.iter().join(" ");
    println!("{}", result);
}
