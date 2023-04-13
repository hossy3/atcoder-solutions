// -*- coding:utf-8-unix -*-

use proconio::input;
use std::collections::BinaryHeap;

fn main() {
    input! {
        n: usize,
        l: i64,
        a: [i64; n],
    }

    let mut heap = BinaryHeap::<i64>::new();
    for x in &a {
        heap.push(-x); // small value is prior
    }

    let rest: i64 = l - a.iter().sum::<i64>();
    if rest > 0 {
        heap.push(-rest);
    }

    let mut score = 0;
    while heap.len() >= 2 {
        let x0 = -heap.pop().unwrap();
        let x1 = -heap.pop().unwrap();
        let x = x0 + x1;
        score += x;
        heap.push(-x);
    }
    println!("{}", score);
}
