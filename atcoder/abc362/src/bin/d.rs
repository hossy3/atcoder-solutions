use std::{cmp::Reverse, collections::BinaryHeap};

use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn dijkstra_all_with_cost(s: usize, graph: &[Vec<(usize, usize)>]) -> Vec<Option<usize>> {
    let n = graph.len();
    let mut v = vec![None; n];
    let mut heap = BinaryHeap::<(Reverse<usize>, usize)>::new();
    v[s] = Some(0usize);
    heap.push((Reverse(0), s));

    while let Some((Reverse(c), i)) = heap.pop() {
        if let Some(c0) = v[i] {
            if c0 < c {
                continue;
            }
        }
        for &(i0, c0) in &graph[i] {
            let c0 = c + c0;
            if let Some(c1) = v[i0] {
                if c1 <= c0 {
                    continue;
                }
            }
            v[i0] = Some(c0);
            heap.push((Reverse(c0), i0));
        }
    }
    v
}

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        uvb: [(Usize1, Usize1, usize); m],
    }

    let mut graph = vec![vec![]; n]; // node, (edge, cost)
    for (u, v, b) in uvb {
        graph[u].push((v, a[v] + b));
        graph[v].push((u, a[u] + b));
    }
    let reachable = dijkstra_all_with_cost(0, &graph);
    let result = (1..n).map(|i| a[0] + reachable[i].unwrap_or(0)).join(" ");
    println!("{result}");
}
