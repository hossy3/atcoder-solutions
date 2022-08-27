use petgraph::{algo::kosaraju_scc, prelude::DiGraph};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        _: usize,
        ab: [(Usize1, Usize1)]
    }
    let g: DiGraph<usize, (), _> = DiGraph::from_edges(&ab);
    let scc = kosaraju_scc(&g);
    let count: usize = scc.iter().map(|v| v.len() * (v.len() - 1) / 2).sum();
    println!("{}", count);
}
