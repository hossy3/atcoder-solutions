use std::{collections::BinaryHeap, cmp::Reverse};

use proconio::input;

fn dijkstra(i: usize, graph: &[Vec<(usize, usize)>], b: usize, c: usize) -> Vec<usize> {
    let n = graph.len();
    let cost_max = 1usize << 60;
    let mut results = vec![cost_max; n];
    results[i] = 0;

    let mut heap = BinaryHeap::new();
    for &(node, cost) in &graph[i] {
        heap.push((Reverse(cost * b + c), node));
    }

    while let Some((Reverse(cost), node)) = heap.pop() {
        if results[node] < cost_max {
            continue;
        }
        results[node] = cost;
        for &(node, cost_edge) in &graph[node] {
            heap.push((Reverse(cost + cost_edge * b + c), node));
        }
    }

    results
}

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        c: usize,
        d: [[usize; n]; n],
    }

    let mut d0 = vec![vec![]; n];
    let mut d1 = vec![vec![]; n];
    for i in 0..n {
        for j in 0..n {
            d0[i].push((j, d[i][j]));
            d1[j].push((i, d[i][j]));
        }
    }

    let v0 = dijkstra(0, &d0, a, 0);
    let v1 = dijkstra(n - 1, &d1, b, c);

    let result = (0..n).map(|i| v0[i] + v1[i]).min().unwrap();
    println!("{result}");
}
