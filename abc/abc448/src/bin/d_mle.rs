use std::collections::HashMap;

use fixedbitset::FixedBitSet;
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
        a: [usize; n],
        uv: [(Usize1, Usize1); n - 1],
    }

    let mut b = HashMap::new();
    for &a in &a {
        *b.entry(a).or_insert(0usize) += 1;
    }
    let mut map = HashMap::new();
    for (k, _) in b.iter().filter(|&(_, &v)| v > 1) {
        map.insert(k, map.len());
    }

    let mut results = vec![false; n];
    let graph = build_ungraph(n, &uv);
    let mut stack = vec![(0, 0, false, FixedBitSet::with_capacity(map.len()))];
    while let Some((u, parent, mut b, mut bitset)) = stack.pop() {
        if !b {
            if let Some(&x) = map.get(&a[u]) {
                if bitset.contains(x) {
                    b = true;
                } else {
                    bitset.insert(x);
                }
            }
        }
        if b {
            results[u] = true;
        }
        for &v in &graph[u] {
            if v != parent {
                stack.push((v, u, b, bitset.clone()));
            }
        }
    }

    for yes in results {
        println!("{}", if yes { "Yes" } else { "No" });
    }
}
