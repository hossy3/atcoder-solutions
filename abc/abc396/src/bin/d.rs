use std::collections::HashSet;

use proconio::{input, marker::Usize1};

trait BitTest {
    fn bit_test(&self, i: usize) -> bool;
}

impl BitTest for usize {
    fn bit_test(&self, i: usize) -> bool {
        self & (1 << i) != 0
    }
}

fn build_digraph(n: usize, uvw: &[(usize, usize, usize)]) -> Vec<Vec<(usize, usize)>> {
    let mut graph = vec![vec![]; n]; // node, edge
    for &(u, v, w) in uvw {
        graph[u].push((v, w));
        graph[v].push((u, w));
    }
    graph
}

fn main() {
    input! {
        n: usize,
        m: usize,
        uvw: [(Usize1, Usize1, usize); m],
    }

    let graph = build_digraph(n, &uvw);

    let mut set = HashSet::new(); // 現在の頂点番号と、すべての行った頂点番号の組
    let mut stack = vec![(0usize, 1usize, 0usize)]; // 頂点番号と、すべての行った場所と、スコアの組
    set.insert((0usize, 1usize, 0usize));

    let mut result = usize::MAX;
    while let Some((u, visited, score)) = stack.pop() {
        if u == n - 1 {
            result = result.min(score);
            continue;
        }
        for &(v, w) in &graph[u] {
            if visited.bit_test(v) {
                continue;
            }
            let visited = visited | (1 << v);
            let score = score ^ w;
            if !set.insert((v, visited, score)) {
                continue;
            }
            stack.push((v, visited, score));
        }
    }
    println!("{result}");
}
