use proconio::{input, marker::Usize1};

fn build_ungraph_weight(n: usize, uv: &[(usize, usize)]) -> Vec<Vec<(usize, usize)>> {
    let mut graph = vec![vec![]; n]; // node, (edge, weight)
    for &(u, v) in uv {
        graph[u].push((v, 1));
        graph[v].push((u, 1));
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
        k: usize,
        uv: [(Usize1, Usize1); m],
    }

    let mut graph = build_ungraph_weight(n, &uv);
    for i in 0..n {
        if graph[i].len() >= k {
            for (_, w) in graph[i].iter_mut() {
                *w += 1;
            }
        }
    }

    if let Some(result) = shortest_graph_weight(&graph, 0, n - 1) {
        let result = if graph[0].len() >= k {
            result - 1
        } else {
            result
        };
        println!("{result}");
    } else {
        println!("-1");
    }
}
