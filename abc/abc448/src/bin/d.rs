use std::collections::BTreeSet;

use proconio::{input, marker::Usize1};

fn build_ungraph(n: usize, uv: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut graph = vec![vec![]; n]; // node, edge
    for &(u, v) in uv {
        graph[u].push(v);
        graph[v].push(u);
    }
    graph
}

fn f(
    a: &[usize],
    u: usize,
    parent: usize,
    graph: &[Vec<usize>],
    mut yes: bool,
    state: &mut BTreeSet<(usize, usize)>,
    results: &mut [bool],
) {
    if !yes {
        if state.range((a[u], 0)..(a[u] + 1, 0)).next().is_some() {
            yes = true;
        }
    }
    if yes {
        results[u] = true;
    }
    state.insert((a[u], u));
    for &v in &graph[u] {
        if v != parent {
            f(a, v, u, graph, yes, state, results);
        }
    }
    state.remove(&(a[u], u));
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        uv: [(Usize1, Usize1); n - 1],
    }

    let mut results = vec![false; n];
    let graph = build_ungraph(n, &uv);
    f(&a, 0, 0, &graph, false, &mut BTreeSet::new(), &mut results);

    for yes in results {
        println!("{}", if yes { "Yes" } else { "No" });
    }
}
