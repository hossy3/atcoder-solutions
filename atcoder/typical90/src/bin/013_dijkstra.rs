use std::{cmp::Reverse, collections::BinaryHeap};

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

fn build_ungraph_with_cost(n: usize, uvc: &[(usize, usize, usize)]) -> Vec<Vec<(usize, usize)>> {
    let mut graph = vec![vec![]; n]; // node, (edge, cost)
    for &(u, v, c) in uvc {
        graph[u].push((v, c));
        graph[v].push((u, c));
    }
    graph
}

fn main() {
    input! {
        n: usize,
        abc: [(Usize1, Usize1, usize)],
    }

    let graph = build_ungraph_with_cost(n, &abc);
    let s0 = dijkstra_all_with_cost(0, &graph);
    let sn = dijkstra_all_with_cost(n - 1, &graph);

    for i in 0..n {
        let result = s0[i].unwrap() + sn[i].unwrap();
        println!("{result}");
    }
}
