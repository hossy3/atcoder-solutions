use ac_library::SccGraph;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1)]
    }
    let mut graph = SccGraph::new(n);
    for (a, b) in ab {
        graph.add_edge(a, b);
    }
    let scc = graph.scc();
    let result: usize = scc.iter().map(|v| v.len() * (v.len() - 1) / 2).sum();
    println!("{result}");
}
