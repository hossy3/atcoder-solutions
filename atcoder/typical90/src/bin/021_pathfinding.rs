use pathfinding::directed::strongly_connected_components::strongly_connected_components;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1)]
    }
    let nodes: Vec<_> = (0..n).collect();
    let mut graph = vec![vec![]; n];
    for (a, b) in ab {
        graph[a].push(b);
    }
    let scc = strongly_connected_components(&nodes, |&i| graph[i].iter().copied());
    let result: usize = scc.iter().map(|v| v.len() * (v.len() - 1) / 2).sum();
    println!("{result}");
}
