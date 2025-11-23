use proconio::{input, marker::Usize1};
use std::{cmp::Reverse, collections::BinaryHeap};

fn dijkstra_all(s: usize, graph: &[Vec<(usize, usize)>]) -> Vec<Option<usize>> {
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
        abx: [(usize, usize, Usize1); n - 1],
    }

    let mut graph = vec![vec![]; n];
    for (i, &(a, b, x)) in abx.iter().enumerate() {
        graph[i].push((i + 1, a));
        graph[i].push((x, b));
    }
    let v = dijkstra_all(0, &graph);
    let result = v[n - 1].unwrap();
    println!("{result}");
}
