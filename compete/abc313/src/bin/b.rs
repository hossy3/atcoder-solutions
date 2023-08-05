use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn build_digraph(n: usize, uv: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut graph = vec![vec![]; n]; // node, edge
    for &(u, v) in uv {
        graph[v].push(u);
    }
    graph
}

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    }
    let graph = build_digraph(n, &ab);
    let yes = graph.iter().filter(|&v| v.len() == 0).count() == 1;
    if yes {
        let result = graph.iter().find_position(|&v| v.len() == 0).unwrap().0 + 1;
        println!("{}", result);
    } else {
        println!("{}", -1);
    }
}
