use itertools::Itertools;
use proconio::{input, marker::Usize1};
use std::collections::VecDeque;

fn dijkstra_all(s: usize, graph: &[Vec<usize>]) -> Vec<Option<usize>> {
    let n = graph.len();
    let mut v = vec![None; n];
    let mut queue = VecDeque::new();
    v[s] = Some(0usize);
    queue.push_back((0, s));

    while let Some((step, i)) = queue.pop_front() {
        for &j in &graph[i] {
            if v[j].is_none() {
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

    let scores = dijkstra_all(0, &graph);
    let index = scores.iter().position_max().unwrap();

    let scores = dijkstra_all(index, &graph);
    let score = scores.iter().max().unwrap().unwrap() + 1; // add a road
    println!("{score}");
}
