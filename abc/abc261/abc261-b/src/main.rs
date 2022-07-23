// -*- coding:utf-8-unix -*-

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        a: [Chars; n],
    }
    let mut yes = true;
    'outer: for i in 0..(n - 1) {
        for j in (i + 1)..n {
            let c = (a[i][j], a[j][i]);
            if !(c == ('W', 'L') || c == ('L', 'W') || c == ('D', 'D')) {
                yes = false;
                break 'outer;
            }
        }
    }
    println!("{}", if yes { "correct" } else { "incorrect" });
}
