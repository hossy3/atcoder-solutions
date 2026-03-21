use std::collections::{HashSet, VecDeque};

use proconio::{input, marker::Usize1};

fn build_ungraph(n: usize, uvw: &[(usize, usize, usize)], k: usize) -> Vec<Vec<usize>> {
    let mut graph = vec![vec![]; n]; // node, edge
    for &(u, v, w) in uvw {
        if w >= k {
            graph[u].push(v);
            graph[v].push(u);
        }
    }
    graph
}

/// 重みなしグラフの s から e までの最短距離を 01-bfs で解く
fn shortest_graph(graph: &[Vec<usize>], s: usize, e: usize) -> Option<usize> {
    let mut set = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((0, s));

    while let Some((step, i)) = queue.pop_front() {
        if i == e {
            return Some(step);
        }
        for &j in &graph[i] {
            if set.insert(j) {
                queue.push_back((step + 1, j));
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
        uvw: [(Usize1, Usize1, usize); m],
    }

    let graph = build_ungraph(n, &uvw, k);
    if let Some(result) = shortest_graph(&graph, 0, n - 1) {
        println!("{result}");
    } else {
        println!("-1");
    }
}
