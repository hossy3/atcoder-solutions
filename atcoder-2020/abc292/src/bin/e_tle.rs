use std::collections::BinaryHeap;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(Usize1, Usize1); m],
    }

    let mut graph = vec![vec![false; n]; n];
    for &(u, v) in &uv {
        graph[u][v] = true;
    }

    let mut heap = BinaryHeap::<(usize, usize)>::new();
    for &(u, v) in &uv {
        for v0 in 0..n {
            if v != v0 && graph[v][v0] && !graph[u][v0] {
                heap.push((u, v0));
            }
        }
    }

    let mut count = 0;
    while let Some((u, v)) = heap.pop() {
        if graph[u][v] || u == v {
            continue;
        }
        count += 1;
        graph[u][v] = true;
        for v0 in 0..n {
            if v != v0 && graph[v][v0] && !graph[u][v0] {
                heap.push((u, v0));
            }
        }
    }

    println!("{}", count);
}
