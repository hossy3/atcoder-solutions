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

fn f(n: usize, uvw: &[(usize, usize, usize)]) -> usize {
    let graph = build_ungraph_weight(n, &uvw);
    let mut odd_nodes = vec![];
    for (i, nodes) in graph.iter().enumerate() {
        if nodes.len() % 2 == 1 {
            odd_nodes.push(i);
        }
    }

    let score = uvw.iter().map(|&(_, _, w)| w).sum::<usize>();

    let len = odd_nodes.len();
    if len == 0 {
        return score; // 奇数辺のノードが存在しない場合は一筆書きできる
    }

    let mut m = vec![vec![usize::MAX; len]; len];
    for i in 0..len {
        m[i][i] = 0;
    }
    for (i, &u) in odd_nodes.iter().enumerate() {
        let shortest = shortest_all_graph_weight(u, &graph);
        for (j, &v) in odd_nodes.iter().enumerate() {
            m[i][j] = shortest[v].unwrap(); // 連結
        }
    }

    // DP で辺を追加したときの最小コストを探す
    let mut state = vec![usize::MAX; 1 << len];
    state[0] = 0;
    for bits in 0..(1 << len) {
        if state[bits] == usize::MAX {
            continue;
        }
        // 各ペアを追加
        for i in 0..(len - 1) {
            if bits & (1 << i) > 0 {
                continue;
            }
            for j in (i + 1)..len {
                if bits & (1 << j) > 0 {
                    continue;
                }
                let x = state[bits] + m[i][j];
                let bits = bits | (1 << i) | (1 << j);
                state[bits] = state[bits].min(x);
            }
        }
    }

    let score = score + state[(1 << len) - 1];
    score
}

fn main() {
    input! {
        n: usize,
        m: usize,
        uvw: [(Usize1, Usize1, usize); m],
    }
    let result = f(n, &uvw);
    println!("{result}");
}
