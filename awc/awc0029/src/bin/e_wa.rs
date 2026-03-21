use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

use proconio::{input, marker::Usize1};

// 巡回セールスマン (TSP), bit DP

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
    }
    graph
}

fn shortest_ungraph(graph: &[Vec<(usize, usize)>], s: usize, e: usize) -> Option<usize> {
    let mut set = HashSet::new();
    let mut heap = BinaryHeap::new();
    set.insert(s);
    heap.push((Reverse(0), s));

    while let Some((Reverse(step), i)) = heap.pop() {
        if i == e {
            return Some(step);
        }
        for &(j, w) in &graph[i] {
            if !set.contains(&j) {
                set.insert(j);
                heap.push((Reverse(step + w), j));
            }
        }
    }
    None
}

fn main() {
    input! {
        n: usize,
        m: usize,
        uvw: [(Usize1, Usize1, usize); m],
        s: Usize1,
        k: usize,
        mut t: [Usize1; k],
    }

    t.insert(0, s);
    let graph = build_digraph(n, &uvw);
    let mut dist = vec![vec![0usize; k + 1]; k + 1];
    for i in 0..=k {
        for j in 0..=k {
            if i == j {
                continue;
            }
            if let Some(x) = shortest_ungraph(&graph, t[i], t[j]) {
                dist[i][j] = x;
            } else {
                println!("-1");
                return;
            }
        }
    }

    let mut state = vec![vec![usize::MAX; k + 1]; 1 << (k + 1)];
    state[1][0] = 0;

    for visited in 1..(1 << (k + 1)) {
        // 現在地
        for i in 0..=k {
            if !visited.bit_test(i) {
                continue;
            }

            // 次の移動先
            for j in 0..=k {
                if visited.bit_test(j) {
                    continue;
                }
                let x = state[visited][i].saturating_add(dist[i][j]);
                let visited = visited | (1 << j);
                state[visited][j] = state[visited][j].min(x);
            }
        }
    }

    let all_visited = (1 << (k + 1)) - 1;
    let result = (0..=k)
        .map(|i| state[all_visited][i].saturating_add(dist[i][0]))
        .min()
        .unwrap();
    println!("{result}");
}
