// -*- coding:utf-8-unix -*-

use proconio::input;
use std::collections::BinaryHeap;

fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(usize, usize, i64); m],
    }

    let mut graph = vec![vec![] as Vec<(i64, usize, usize)>; n + 1]; // -cost, node, edge
    for i in 0..m {
        let (a, b, c) = abc[i];
        graph[a].push((-c, b, i + 1));
        graph[b].push((-c, a, i + 1));
    }

    let cost_max = 1i64 << 60;
    let mut results = vec![(cost_max, 0usize); n + 1]; // cost, edge
    results[1].0 = 0;

    let mut heap = BinaryHeap::<(i64, usize, usize)>::new(); // -cost, node, edge
    for &(cost, node, edge) in &graph[1] {
        heap.push((cost, node, edge));
    }
    while let Some((cost, node, edge)) = heap.pop() {
        if results[node].0 < cost_max {
            continue;
        }
        results[node].0 = -cost;
        results[node].1 = edge;
        for &(cost_edge, node, edge) in &graph[node] {
            heap.push((cost + cost_edge, node, edge));
        }
    }

    for i in 2..n {
        print!("{} ", results[i].1);
    }
    println!("{}", results[n].1);
}
