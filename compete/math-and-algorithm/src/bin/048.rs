use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        k: usize,
    }
    let mut visited = vec![false; k];
    let mut heap = BinaryHeap::new();
    for i in 1..=9 {
        heap.push((Reverse(i), i % k));
    }
    while let Some((Reverse(step), i)) = heap.pop() {
        if visited[i] {
            continue;
        }
        if i == 0 {
            println!("{}", step);
            break;
        }
        visited[i] = true;
        for j in 0..=9 {
            let i = (i * 10 + j) % k;
            if !visited[i] {
                heap.push((Reverse(step + j), i));
            }
        }
    }
}
