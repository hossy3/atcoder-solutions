use std::collections::VecDeque;

use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn shortest_all_ungraph(s: usize, graph: &[Vec<usize>]) -> Vec<Option<usize>> {
    let n = graph.len();
    let mut v = vec![None; n];
    let mut queue = VecDeque::new();
    v[s] = Some(0usize);
    queue.push_back((0, s));

    while let Some((step, i)) = queue.pop_front() {
        for &j in &graph[i] {
            if v[j] == None {
                v[j] = Some(step + 1);
                queue.push_back((step + 1, j));
            }
        }
    }
    v
}

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
        ab: [(Usize1, Usize1); n - 1],
    }

    let graph = build_ungraph(n, &ab);

    let scores = shortest_all_ungraph(0, &graph);
    let index = scores.iter().position_max().unwrap();

    let scores = shortest_all_ungraph(index, &graph);
    let score = scores.iter().max().unwrap().unwrap() + 1; // add a road
    println!("{score}");
}
