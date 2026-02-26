use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

use proconio::{input, marker::Usize1};

fn shortest_ungraph(graph: &[Vec<(usize, usize)>], s: usize, e: usize) -> Option<usize> {
    let mut set = HashSet::new();
    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), s));

    while let Some((Reverse(step), i)) = heap.pop() {
        if !set.insert(i) {
            continue;
        }
        if i == e {
            return Some(step);
        }
        for &(j, c) in &graph[i] {
            heap.push((Reverse(step + c), j));
        }
    }
    None
}

fn build_ungraph(n: usize, uvc: &[(usize, usize, usize)]) -> Vec<Vec<(usize, usize)>> {
    let mut graph = vec![vec![]; n]; // node, edge
    for &(u, v, c) in uvc {
        graph[u].push((v, c));
        graph[v].push((u, c));
    }
    graph
}

fn main() {
    input! {
        n: usize,
        m: usize,
        t: Usize1,
        abc: [(Usize1, Usize1, usize); m],
    }
    let graph = build_ungraph(n, &abc);
    if let Some(len) = shortest_ungraph(&graph, 0, t) {
        println!("{}", len * 2);
    } else {
        println!("-1");
    }
}
