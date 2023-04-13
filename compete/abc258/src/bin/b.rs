// -*- coding:utf-8-unix -*-

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        a: [Chars; n],
    }
    let ways = vec![
        (n - 1, n - 1),
        (n - 1, 0),
        (n - 1, 1),
        (0, n - 1),
        (0, 1),
        (1, n - 1),
        (1, 0),
        (1, 1),
    ];
    let mut max_score = 0i64;
    for i in 0..n {
        for j in 0..n {
            for way in &ways {
                let mut score = 0i64;
                for k in 0..n {
                    let i = (i + way.0 * k) % n;
                    let j = (j + way.1 * k) % n;
                    score = score * 10 + (a[i][j] as i64 - '0' as i64);
                }
                max_score = max_score.max(score);
            }
        }
    }
    println!("{}", max_score);
}
