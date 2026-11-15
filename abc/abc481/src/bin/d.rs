use proconio::{input, marker::Usize1};

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
        s: Usize1,
        t: Usize1,
        ab: [(Usize1, Usize1); m],
    }

    // グラフを組み立てる
    let mut graph = vec![vec![]; n]; // node, (edge, weight)
    for &(a, b) in &ab {
        graph[a].push((b, 1));
    }
    for i in 0..n {
        graph[i].sort();
        if graph[i].len() > 0 {
            graph[i][0].1 = 0; // 番号が最小の惑星に、燃料を消費せずに移動
        }
    }

    if let Some(result) = shortest_graph_weight(&graph, s, t) {
        println!("{result}");
    } else {
        println!("-1");
    }
}
