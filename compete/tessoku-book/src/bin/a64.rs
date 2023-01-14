use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        abc: [(Usize1, Usize1, i64)],
    }

    let mut g = vec![vec![]; n];
    for &(a, b, c) in &abc {
        g[a].push((b, c));
        g[b].push((a, c));
    }

    let mut v = vec![-1; n];
    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), 0));
    while let Some((Reverse(step), i)) = heap.pop() {
        if v[i] < 0 {
            v[i] = step;
            for &(i, c) in &g[i] {
                heap.push((Reverse(step + c), i));
            }
        }
    }

    for x in &v {
        println!("{}", x);
    }
}
