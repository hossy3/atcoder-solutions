use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn build_ungraph_weight(n: usize, uvw: &[(usize, usize, usize)]) -> Vec<Vec<(usize, usize)>> {
    let mut graph = vec![vec![]; n]; // node, (edge, weight)
    for &(u, v, w) in uvw {
        graph[u].push((v, w * 2));
        graph[v].push((u, w * 2));
    }
    graph
}

/// 非負の重み付きグラフの s からすべてのノードへの最短距離をダイクストラ法で解く
fn shortest_all_graph_weight(s: usize, graph: &[Vec<(usize, usize)>]) -> Vec<Option<usize>> {
    use std::{cmp::Reverse, collections::BinaryHeap};

    let n = graph.len();
    let mut v = vec![None; n];

    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), s));
    v[s] = Some(0);

    while let Some((Reverse(step), i)) = heap.pop() {
        if step > v[i].unwrap_or(usize::MAX) {
            continue;
        }

        for &(j, w) in &graph[i] {
            let step = step + w;
            let step0 = v[j].unwrap_or(usize::MAX);
            if step < step0 {
                v[j] = Some(step);
                heap.push((Reverse(step), j));
            }
        }
    }
    v
}

fn main() {
    input! {
        n: usize,
        m: usize,
        y: usize,
        uvt: [(Usize1, Usize1, usize); m],
        x: [usize; n],
    }

    let mut graph = build_ungraph_weight(n + 1, &uvt);
    for (i, &x) in x.iter().enumerate() {
        graph[i].push((n, y + x * 2));
        graph[n].push((i, y + x * 2));
    }

    let results = shortest_all_graph_weight(0, &graph);
    println!(
        "{}",
        results[1..n].iter().map(|&x| x.unwrap() / 2).join(" ")
    );
}
