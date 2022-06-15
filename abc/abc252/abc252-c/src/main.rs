// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [usize; n],
    }
    let mut va = vec![0usize; 10 * n];
    for i in 0..n {
        let mut x = s[i];
        for j in 0..10 {
            let y = x % 10;
            va[i * 10 + y] = 9 - j;
            x = x / 10;
        }
    }
    let mut scores = vec![0; 10];
    for i in 0..10 {
        let mut ary = vec![0; 10];
        let mut counts = [0; 10];
        for j in 0..n {
            counts[va[j * 10 + i]] += 1;
        }
        for j in 0..10 {
            if counts[j] > 0 {
                ary[j] = j + (counts[j] - 1) * 10;
            }
        }
        let score = ary.iter().max().unwrap();
        scores[i] = *score;
    }
    let score = scores.iter().min().unwrap();
    println!("{}", score);
}
