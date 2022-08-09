use std::collections::BinaryHeap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        ab: [(usize, usize); n],
    }
    let mut q = BinaryHeap::new();
    for (a, b) in ab {
        q.push(a - b);
        q.push(b);
    }
    let mut score = 0;
    for _ in 0..k {
        score += q.pop().unwrap_or(0);
    }
    println!("{}", score);
}
