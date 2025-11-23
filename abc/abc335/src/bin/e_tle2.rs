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

fn solve(graph: &[Vec<usize>], a: &[usize]) -> usize {
    let n = a.len();
    let mut scores = vec![0usize; a.len()];
    scores[0] = 1;
    let mut heap = BinaryHeap::new();
    heap.push((Reverse(a[0]), 1, 0)); // (x, score, index)

    while let Some((Reverse(x), score, i)) = heap.pop() {
        if i == n - 1 {
            return score;
        }
        for &j in &graph[i] {
            let x0 = a[j];
            if x0 < x {
                continue;
            }
            let score0 = if x0 == x { score } else { score + 1 };
            if score0 < score || (x0 == x && scores[j] == score) {
                continue;
            }
            scores[j] = score0;
            heap.push((Reverse(x0), score0, j));
        }
    }

    scores[n - 1]
}

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        uv: [(Usize1, Usize1); m],
    }

    let graph = build_ungraph(n, &uv);
    let result = solve(&graph, &a);
    println!("{result}");
}
