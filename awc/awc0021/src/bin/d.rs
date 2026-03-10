use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

use proconio::{input, marker::Usize1};

fn build_ungraph_weight(n: usize, uvw: &[(usize, usize, usize)]) -> Vec<Vec<(usize, usize)>> {
    let mut graph = vec![vec![]; n]; // node, (edge, weight)
    for &(u, v, w) in uvw {
        graph[u].push((v, w));
        graph[v].push((u, w));
    }
    graph
}

fn shortest_ungraph(graph: &[Vec<(usize, usize)>], s: usize, e: usize) -> Option<usize> {
    let mut set = HashSet::new();
    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), s));

    while let Some((Reverse(step), u)) = heap.pop() {
        if !set.insert(u) {
            continue; // 登録済み
        }
        if u == e {
            return Some(step);
        }
        for &(v, t) in &graph[u] {
            heap.push((Reverse(step + t), v));
        }
    }
    None
}

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        uvt: [(Usize1, Usize1, usize); m],
        mut p: [Usize1; k],
    }

    // 先頭と末尾を追加
    p.insert(0, 0);
    p.push(n - 1);

    let graph = build_ungraph_weight(n, &uvt);
    let mut result = 0usize;
    for v in p.windows(2) {
        let (u, v) = (v[0], v[1]);
        if let Some(t) = shortest_ungraph(&graph, u, v) {
            result += t;
        } else {
            println!("-1");
            return;
        }
    }

    println!("{result}");
}
