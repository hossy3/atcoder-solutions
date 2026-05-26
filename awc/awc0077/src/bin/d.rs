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
fn shortest_graph_weight(
    graph: &[Vec<(usize, usize)>],
    s: usize,
    e: usize,
    k: usize,
    f: &[usize],
) -> Option<usize> {
    use std::{cmp::Reverse, collections::BinaryHeap};

    let n = graph.len();
    let mut v = vec![vec![None; k + 1]; n];

    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), Reverse(f[s]), s));
    v[s][f[s]] = Some(0);

    while let Some((Reverse(step), Reverse(f0), i)) = heap.pop() {
        if step > v[i][f0].unwrap_or(usize::MAX) {
            continue;
        }
        if i == e {
            return Some(step);
        }

        for &(j, w) in &graph[i] {
            let step = step + w * f0;
            let f0 = f0.min(f[j]);
            let step0 = v[j][f0].unwrap_or(usize::MAX);
            if step < step0 {
                v[j][f0] = Some(step);
                heap.push((Reverse(step), Reverse(f0), j));
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
        f: [usize; n],
        uvt: [(Usize1, Usize1, usize); m],
    }

    let graph = build_ungraph_weight(n, &uvt);
    if let Some(result) = shortest_graph_weight(&graph, 0, n - 1, k, &f) {
        println!("{result}");
    } else {
        println!("-1");
    }
}
