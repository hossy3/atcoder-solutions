use proconio::{input, marker::Usize1};

fn build_ungraph_weight(n: usize, uvw: &[(usize, usize, usize)]) -> Vec<Vec<(usize, usize)>> {
    let mut graph = vec![vec![]; n]; // node, (edge, weight)
    for &(u, v, w) in uvw {
        graph[u].push((v, w));
        graph[v].push((u, w));
    }
    graph
}

/// 非負の重み付きグラフの s から e までの最短距離をダイクストラ法で解く
fn shortest_graph_weight(graph: &[Vec<(usize, usize)>], s: usize, e: usize) -> Option<usize> {
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
        if i == e {
            return Some(step);
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

    None
}

fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(Usize1, Usize1, usize); m],
    }

    let graph = build_ungraph_weight(n, &abc);
    if let Some(result) = shortest_graph_weight(&graph, 0, n - 1) {
        println!("{result}");
    } else {
        println!("-1");
    }
}
