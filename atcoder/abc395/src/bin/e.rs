use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

use proconio::{input, marker::Usize1};

fn build_digraph(n: usize, uv: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut graph = vec![vec![]; n]; // node, edge
    for &(u, v) in uv {
        graph[u].push(v);
    }
    graph
}

fn main() {
    input! {
        n: usize,
        m: usize,
        x: usize,
        uv: [(Usize1, Usize1); m],
    }

    let vu: Vec<_> = uv.iter().map(|&(x, y)| (y, x)).collect();
    let graph = [build_digraph(n, &uv), build_digraph(n, &vu)];

    let mut set = HashSet::new();
    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), 0usize, 0usize));
    while let Some((Reverse(cost), dir, pos)) = heap.pop() {
        if pos == n - 1 {
            println!("{cost}");
            break;
        }
        if !set.insert((dir, pos)) {
            continue;
        }
        heap.push((Reverse(cost + x), 1 - dir, pos));
        for &v in &graph[dir][pos] {
            heap.push((Reverse(cost + 1), dir, v));
        }
    }
}
