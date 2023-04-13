// -*- coding:utf-8-unix -*-

use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
        b: [usize; k],
    }
    let mut h = HashSet::<usize>::new();
    let mut score = 0;
    for i in 0..n {
        if a[i] > score {
            h.clear();
            score = a[i];
        }
        if a[i] == score {
            h.insert(i + 1);
        }
    }
    let yes = b.iter().any(|x| h.contains(x));

    println!("{}", if yes { "Yes" } else { "No" });
}
