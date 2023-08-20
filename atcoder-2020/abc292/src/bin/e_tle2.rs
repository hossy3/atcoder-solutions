use std::collections::{BinaryHeap, HashSet};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(Usize1, Usize1); m],
    }

    let mut graph = vec![HashSet::<usize>::new(); n];
    for &(u, v) in &uv {
        graph[u].insert(v);
    }

    let mut heap = BinaryHeap::<(usize, usize)>::new();
    for &(u, v) in &uv {
        for v0 in 0..n {
            if u != v0 && v != v0 && graph[v].contains(&v0) && !graph[u].contains(&v0) {
                heap.push((u, v0));
            }
        }
    }

    let mut count = 0;
    while let Some((u, v)) = heap.pop() {
        if graph[u].contains(&v) || u == v {
            continue;
        }
        count += 1;
        graph[u].insert(v);
        for v0 in 0..n {
            if u != v0 && v != v0 && graph[v].contains(&v0) && !graph[u].contains(&v0) {
                heap.push((u, v0));
            }
        }
    }

    println!("{}", count);
}
