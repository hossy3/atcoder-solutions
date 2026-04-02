use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

use proconio::{input, marker::Usize1};

fn build_digraph_weight(n: usize, uvw: &[(usize, usize, usize)]) -> Vec<Vec<(usize, usize)>> {
    let mut graph = vec![vec![]; n]; // node, (edge, weight)
    for &(u, v, w) in uvw {
        graph[u].push((v, w));
    }
    graph
}

/// 非負の重み付きグラフの s から e までの最短距離をダイクストラ法で解く
fn shortest_graph(graph: &[Vec<(usize, usize)>], s: usize, e: usize) -> Option<usize> {
    let mut set = HashSet::new();
    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), s));

    while let Some((Reverse(step), i)) = heap.pop() {
        if i == e {
            return Some(step);
        }
        if set.insert(i) {
            for &(j, w) in &graph[i] {
                if !set.contains(&j) {
                    heap.push((Reverse(step + w), j));
                }
            }
        }
    }
    None
}

fn main() {
    input! {
        n: usize,
        m: usize,
        uvc: [(Usize1, Usize1, usize); m],
    }

    let graph = build_digraph_weight(n, &uvc);
    let result = shortest_graph(&graph, 0, n - 1).unwrap();
    println!("{result}");
}
