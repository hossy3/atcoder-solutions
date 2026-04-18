use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

use proconio::{input, marker::Usize1};

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

fn build_ungraph_weight(n: usize, uvw: &[(usize, usize, usize)]) -> Vec<Vec<(usize, usize)>> {
    let mut graph = vec![vec![]; n]; // node, (edge, weight)
    for &(u, v, w) in uvw {
        graph[u].push((v, w));
        graph[v].push((u, w));
    }
    graph
}

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        mut uvw: [(Usize1, Usize1, usize); m],
        lc: [(Usize1, usize); k],
    }

    let mut lc0 = vec![0usize; n];
    for &(l, c) in &lc {
        lc0[l] = c;
    }
    for i in 0..m {
        uvw[i].2 = uvw[i].2 * 2 + lc0[uvw[i].0] + lc0[uvw[i].1];
    }

    let graph = build_ungraph_weight(n, &uvw);
    let result = (shortest_graph(&graph, 0, n - 1).unwrap() + lc0[0] + lc0[n - 1]) / 2;
    println!("{result}");
}
