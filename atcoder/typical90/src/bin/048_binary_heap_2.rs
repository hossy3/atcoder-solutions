use proconio::input;
use std::collections::BinaryHeap;

fn main() {
    input! {
        n: usize,
        k: usize,
        ab: [(usize, usize); n],
    }
    let mut q = BinaryHeap::new();
    for (a, b) in ab {
        q.push((b, a - b));
    }
    let mut score = 0;
    for _ in 0..k {
        let (x, y) = q.pop().unwrap();
        score += x;
        if y > 0 {
            q.push((y, 0));
        }
    }
    println!("{score}");
}
