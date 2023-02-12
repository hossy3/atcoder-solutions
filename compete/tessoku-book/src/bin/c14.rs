use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::{input, marker::Usize1};

fn dijkstra(i: usize, graph: &[Vec<(usize, usize)>]) -> Vec<usize> {
    let n = graph.len();
    let cost_max = 1usize << 60;
    let mut results = vec![cost_max; n];
    results[i] = 0;

    let mut heap = BinaryHeap::new();
    for &(node, cost) in &graph[i] {
        heap.push((Reverse(cost), node));
    }

    while let Some((Reverse(cost), node)) = heap.pop() {
        if results[node] < cost_max {
            continue;
        }
        results[node] = cost;
        for &(node, cost_edge) in &graph[node] {
            heap.push((Reverse(cost + cost_edge), node));
        }
    }

    results
}

fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(Usize1, Usize1, usize); m],
    }

    let mut graph = vec![vec![]; n];
    for i in 0..m {
        let (a, b, c) = abc[i];
        graph[a].push((b, c));
        graph[b].push((a, c));
    }

    let v0 = dijkstra(0, &graph);
    let v1 = dijkstra(n - 1, &graph);
    let count = (0..n).filter(|&i| v0[i] + v1[i] == v1[0]).count();
    println!("{}", count);
}
