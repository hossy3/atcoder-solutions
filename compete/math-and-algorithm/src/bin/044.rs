use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::{input, marker::Usize1};

fn build_ungraph(n: usize, uv: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut graph = vec![vec![]; n]; // node, edge
    for &(u, v) in uv {
        graph[u].push(v);
        graph[v].push(u);
    }
    graph
}

fn shortest_all_ungraph(n: usize, s: usize, graph: &[Vec<usize>]) -> Vec<i64> {
    let mut v = vec![-1; n];
    let mut heap = BinaryHeap::new();
    v[s] = 0;
    heap.push((Reverse(0), s));

    while let Some((Reverse(step), i)) = heap.pop() {
        for &j in &graph[i] {
            if v[j] < 0 {
                v[j] = step + 1;
                heap.push((Reverse(step + 1), j));
            }
        }
    }
    v
}

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    }

    let graph = build_ungraph(n, &ab);
    let results = shortest_all_ungraph(n, 0, &graph);
    for x in results {
        println!("{}", x);
    }
}
