// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        h1: i64,
        h2: i64,
        h3: i64,
        w1: i64,
        w2: i64,
        w3: i64,
    }
    if (h1 + h2 + h3) != (w1 + w2 + w3) {
        println!("{}", 0);
        return;
    }
    let mut score: i64 = 0;
    for m11 in 1..=((h1 - 2).min(w1 - 2)) {
        for m12 in 1..=((h1 - m11 - 1).min(w2 - 2)) {
            let m13 = h1 - m11 - m12;
            if m13 < 0 || m13 > w3 - 2 {
                continue;
            }
            for m21 in 1..=((h2 - 2).min(w1 - m11 - 1)) {
                let m31 = w1 - m11 - m21;
                if m31 < 0 || m31 > h3 - 2 {
                    continue;
                }
                for m22 in 1..=((h2 - m21 - 1).min(w2 - m12 - 1)) {
                    let m23 = h2 - m21 - m22;
                    if m23 < 0 || m23 > w3 - m13 - 1 {
                        continue;
                    }
                    let m32 = w2 - m12 - m22;
                    if m32 < 0 || m32 > h3 - m31 - 1 {
                        continue;
                    }
                    score += 1;
                }
            }
        }
    }
    println!("{}", score);
}
