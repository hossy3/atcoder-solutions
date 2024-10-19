use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

use proconio::{input, marker::Usize1};

fn build_digraph(n: usize, uv: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut graph = vec![vec![]; n]; // node, edge
    for &(u, v) in uv {
        graph[u].push(v);
    }
    graph
}

fn find_shortest_loop(graph: &[Vec<usize>], s: usize) -> Option<usize> {
    let mut set = HashSet::new();
    let mut heap = BinaryHeap::new();
    for &x in &graph[s] {
        set.insert(x);
        heap.push((Reverse(1), x));
    }

    while let Some((Reverse(step), x)) = heap.pop() {
        for &x in &graph[x] {
            if x == s {
                return Some(step + 1);
            }
            if !set.contains(&x) {
                set.insert(x);
                heap.push((Reverse(step + 1), x));
            }
        }
    }
    None
}

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    }

    let graph = build_digraph(n, &ab);
    if let Some(result) = find_shortest_loop(&graph, 0) {
        println!("{result}");
    } else {
        println!("-1");
    }
}
