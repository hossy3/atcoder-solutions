use std::collections::HashSet;

use ac_library::Dsu;
use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn build_ungraph(n: usize, uvw: &[(usize, usize, i64)]) -> Vec<Vec<(usize, i64)>> {
    let mut graph = vec![vec![]; n];
    for &(u, v, w) in uvw {
        graph[u].push((v, w));
        graph[v].push((u, -w));
    }
    graph
}

fn main() {
    input! {
        n: usize,
        m: usize,
        uvw: [(Usize1, Usize1, i64); m],
    }

    let graph = build_ungraph(n, &uvw);

    let mut stack = vec![];

    let mut dsu = Dsu::new(n);
    for &(u, v, _) in &uvw {
        dsu.merge(u, v);
    }

    let mut results: Vec<Option<i64>> = vec![None; n];

    let mut set = HashSet::new();
    for i in 0..n {
        let u = dsu.leader(i);
        if set.insert(u) {
            results[u] = Some(0);
            stack.push((u, 0));
        }
    }

    while let Some((u, x)) = stack.pop() {
        for &(v, w) in &graph[u] {
            if results[v].is_some() {
                continue;
            }
            let x = x + w;
            results[v] = Some(x);
            stack.push((v, x));
        }
    }

    let result = results.iter().map(|&x| x.unwrap()).join(" ");
    println!("{result}");
}
