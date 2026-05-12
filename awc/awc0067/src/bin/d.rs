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

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [usize; n],
        uvw: [(Usize1, Usize1, usize); m],
    }

    let graph = build_ungraph_weight(n, &uvw);
    let mut distances = vec![];
    for i in 0..n {
        distances.push(shortest_all_graph_weight(i, &graph));
    }
    // eprintln!("{:?}", &distances);

    let mut v = vec![];
    for i in 0..n {
        let mut count = 0usize; // 不満の個数
        for j in 0..n {
            let d = distances[i][j].unwrap_or(usize::MAX);
            if d > s[i] {
                count += 1;
            }
        }
        v.push(count);
    }
    // eprintln!("{:?}", &v);

    let mut result = 0usize;
    for i in 0..(n - 1) {
        for j in (i + 1)..n {
            if v[i] != v[j] {
                continue;
            }
            let d = distances[i][j].unwrap_or(usize::MAX);
            if d > s[i] || d > s[j] {
                continue;
            }
            // eprintln!("{}, {}", i, j);
            result += 1;
        }
    }
    println!("{result}");
}
