use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

use proconio::{input, marker::Usize1};

fn build_ungraph(n: usize, uvw: &[(usize, usize, usize)], k: usize) -> Vec<Vec<usize>> {
    let mut graph = vec![vec![]; n]; // node, edge
    for &(u, v, w) in uvw {
        if w >= k {
            graph[u].push(v);
            graph[v].push(u);
        }
    }
    graph
}

fn shortest_ungraph(graph: &[Vec<usize>], s: usize, e: usize) -> Option<usize> {
    let mut set = HashSet::new();
    let mut heap = BinaryHeap::new();
    set.insert(s);
    heap.push((Reverse(0), s));

    while let Some((Reverse(step), i)) = heap.pop() {
        if i == e {
            return Some(step);
        }
        for &j in &graph[i] {
            if !set.contains(&j) {
                set.insert(j);
                heap.push((Reverse(step + 1), j));
            }
        }
    }
    None
}

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        uvw: [(Usize1, Usize1, usize); m],
    }

    let graph = build_ungraph(n, &uvw, k);
    if let Some(result) = shortest_ungraph(&graph, 0, n - 1) {
        println!("{result}");
    } else {
        println!("-1");
    }
}
