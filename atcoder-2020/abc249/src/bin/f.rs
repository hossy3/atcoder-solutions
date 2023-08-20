use std::collections::BinaryHeap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i64,
        ty: [(usize, i64); n],
    }

    let mut score = -(1 << 60);
    let mut heap = BinaryHeap::<i64>::new();
    let mut accepted = 0;
    let mut skippable = k;
    for i in 0..n {
        let (t, y) = ty[n - 1 - i];
        if t == 1 {
            score = score.max(y + accepted);
            skippable -= 1;
            if skippable < 0 {
                break;
            }
        } else if t == 2 {
            if y >= 0 {
                accepted += y;
            } else {
                heap.push(y);
            }
        }
        if heap.len() as i64 > skippable {
            accepted += heap.pop().unwrap();
        }
    }
    if skippable >= 0 {
        score = score.max(accepted);
    }
    println!("{}", score);
}
