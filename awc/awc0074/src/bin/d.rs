use proconio::{input, marker::Usize1};

fn build_ungraph_weight(n: usize, uvw: &[(usize, usize, usize)]) -> Vec<Vec<(usize, usize)>> {
    let mut graph = vec![vec![]; n]; // node, (edge, weight)
    for &(u, v, w) in uvw {
        graph[u].push((v, w));
        graph[v].push((u, w));
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

/// 非負の重み付きグラフの s から e までの一番安全な距離をダイクストラ法で解く
fn solve_graph_weight(
    graph: &[Vec<(usize, usize)>],
    s: usize,
    e: usize,
    d: &[Option<usize>],
) -> Option<usize> {
    use std::collections::BinaryHeap;

    let n = graph.len();
    let mut v = vec![None; n];

    let w = d[s].unwrap_or(usize::MAX).min(d[e].unwrap_or(usize::MAX));
    let mut heap = BinaryHeap::new();
    heap.push((w, s));
    v[s] = Some(w);

    while let Some((w, i)) = heap.pop() {
        if i == e {
            return Some(w);
        }

        for &(j, _) in &graph[i] {
            if v[j].is_some() {
                continue;
            }
            let w = w.min(d[j].unwrap_or(usize::MAX));
            v[j] = Some(w);
            heap.push((w, j));
        }
    }

    None
}

fn main() {
    input! {
        n: usize,
        m: usize,
        s: Usize1,
        t: Usize1,
        p: Usize1,
        uvw: [(Usize1, Usize1, usize); m],
    }

    let graph = build_ungraph_weight(n, &uvw);
    let d = shortest_all_graph_weight(p, &graph); // 野犬からの距離
    let result = solve_graph_weight(&graph, s, t, &d).unwrap();
    if result < usize::MAX {
        println!("{result}");
    } else {
        println!("INF");
    }
}
