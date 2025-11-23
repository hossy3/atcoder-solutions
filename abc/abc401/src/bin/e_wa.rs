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
        uv: [(Usize1, Usize1); m],
    }

    let graph = build_digraph(n, &uv);
    let mut results = vec![0usize; n];
    for v in (0..n).rev() {
        for &u in &graph[v] {
            results[u] += 1;
        }
    }

    for &result in &results {
        println!("{}", result);
    }
}
