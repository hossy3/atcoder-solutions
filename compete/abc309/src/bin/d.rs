use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

fn dijkstra(i: usize, graph: &[Vec<usize>]) -> usize {
    let mut v = vec![std::usize::MAX; graph.len()];
    let mut queue = VecDeque::new();
    v[i] = 0;
    queue.push_back((0, i));

    while let Some((step, i)) = queue.pop_front() {
        for &i in &graph[i] {
            if v[i] == std::usize::MAX {
                let step = step + 1;
                v[i] = step;
                queue.push_back((step, i));
            }
        }
    }

    *v.iter().max().unwrap_or(&0)
}

fn main() {
    input! {
        n1: usize,
        n2: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    }

    let mut graph1 = vec![vec![]; n1];
    let mut graph2 = vec![vec![]; n2];
    for &(a, b) in &ab {
        if a < n1 && b < n1 {
            graph1[a].push(b);
            graph1[b].push(a);
        } else if a >= n1 && b >= n1 {
            graph2[a - n1].push(b - n1);
            graph2[b - n1].push(a - n1);
        } else {
            panic!();
        }
    }

    let step1 = dijkstra(0, &graph1);
    let step2 = dijkstra(n2 - 1, &graph2);
    let result = step1 + step2 + 1;
    println!("{}", result);
}
