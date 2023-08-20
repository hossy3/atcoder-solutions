// -*- coding:utf-8-unix -*-

use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        mut sts: [(String, i32); n],
    }
    let mut s = HashSet::<&String>::new();
    let mut max_index = 0;
    let mut max_score = -1;
    for i in 0..n {
        if let None = s.get(&sts[i].0) {
            if sts[i].1 > max_score {
                max_score = sts[i].1;
                max_index = i;
            }
            s.insert(&sts[i].0);
        }
    }
    println!("{}", max_index + 1);
}
