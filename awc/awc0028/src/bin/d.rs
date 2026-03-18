use std::collections::BTreeSet;

use itertools::Itertools;
use proconio::{input, marker::Usize1};

// 依存関係
fn build_digraph(n: usize, uv: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut graph = vec![vec![]; n]; // node, edge
    for &(u, v) in uv {
        graph[u].push(v);
    }
    graph
}

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    }

    let graph = build_digraph(n, &ab); // Bを履修するにはAが必要
    let mut rest = vec![0usize; n]; // 残りいくつ履修すればそれを選べるか
    for v in &graph {
        for &x in v {
            rest[x] += 1;
        }
    }

    let mut set = BTreeSet::new();
    for (i, &x) in rest.iter().enumerate() {
        if x == 0 {
            set.insert(i);
        }
    }

    let mut results = vec![];
    while let Some(x) = set.pop_first() {
        results.push(x + 1);
        for &x in &graph[x] {
            rest[x] -= 1;
            if rest[x] == 0 {
                set.insert(x);
            }
        }
    }

    println!("{}", results.iter().join(" "));
}
