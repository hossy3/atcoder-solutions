use pathfinding::prelude::dijkstra_all;

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
        ab: [(Usize1, Usize1); n - 1],
    }

    let graph = build_ungraph(n, &ab);
    let reachables = dijkstra_all(&0, |&i| graph[i].iter().map(|&j| (j, 1usize)));
    let index = *reachables
        .iter()
        .max_by_key(|(_, &(_, step))| step)
        .unwrap()
        .0;

    let reachables = dijkstra_all(&index, |&i| graph[i].iter().map(|&j| (j, 1usize)));
    let step = reachables.iter().map(|(_, &(_, step))| step).max().unwrap();
    let result = step + 1; // add a road
    println!("{result}");
}
