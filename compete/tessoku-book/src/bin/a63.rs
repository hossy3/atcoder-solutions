use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1)],
    }

    let mut g = vec![vec![]; n];
    for &(a, b) in &ab {
        g[a].push(b);
        g[b].push(a);
    }

    let mut v = vec![-1; n];
    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), 0));
    while let Some((Reverse(step), i)) = heap.pop() {
        if v[i] < 0 {
            v[i] = step;
            for &i in &g[i] {
                heap.push((Reverse(step + 1), i));
            }
        }
    }

    for x in &v {
        println!("{}", x);
    }
}
