use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        abcd: [(Usize1, Usize1, usize, usize)],
    }

    let mut g = vec![vec![]; n];
    for &(a, b, c, d) in &abcd {
        g[a].push((Reverse(c), d, b));
        g[b].push((Reverse(c), d, a));
    }

    let mut visited = vec![false; n];
    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), 0, 0));

    while let Some((Reverse(c), d, i)) = heap.pop() {
        if visited[i] {
            continue;
        }
        visited[i] = true;
        if i == n - 1 {
            println!("{} {}", c, d);
            return;
        }
        for &(Reverse(c0), d0, i) in &g[i] {
            heap.push((Reverse(c + c0), d + d0, i));
        }
    }
}
