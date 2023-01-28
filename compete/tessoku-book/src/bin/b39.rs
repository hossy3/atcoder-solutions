use std::collections::BinaryHeap;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        d: usize,
        xy: [(Usize1, usize); n],
    }

    let mut v = vec![vec![]; d];
    for (i, &(x, y)) in xy.iter().enumerate() {
        v[x].push((y, i));
    }

    let mut result = 0;
    let mut heap = BinaryHeap::new();
    for i in 0..d {
        for y in &v[i] {
            heap.push(y);
        }
        if let Some(&(y, _)) = heap.pop() {
            result += y;
        }
    }
    println!("{}", result);
}
