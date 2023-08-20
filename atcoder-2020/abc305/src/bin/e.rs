use std::collections::BinaryHeap;

use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn build_ungraph(n: usize, uv: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut graph = vec![vec![]; n]; // node, edge
    for &(u, v) in uv {
        graph[u].push(v);
        graph[v].push(u);
    }
    graph
}

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        ab: [(Usize1, Usize1); m],
        ph: [(Usize1, i64); k],
    }

    let graph = build_ungraph(n, &ab);
    let mut v = vec![-1; n];
    let mut queue = BinaryHeap::new();
    for &(p, h) in &ph {
        v[p] = h;
        queue.push((h, p));
    }

    while let Some((h, p)) = queue.pop() {
        if v[p] > h {
            continue;
        }
        let h = h - 1;
        for &p in &graph[p] {
            if v[p] < h {
                v[p] = h;
                queue.push((h, p));
            }
        }
    }

    let results = (0..n).filter(|&i| v[i] >= 0).map(|i| i + 1).collect_vec();
    println!("{}", results.len());
    println!("{}", results.iter().join(" "));
}
