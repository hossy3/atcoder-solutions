use std::collections::HashSet;

use proconio::{input, marker::Usize1};

fn build_ungraph(n: usize, uv: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut graph = vec![vec![]; n]; // node, edge
    for &(u, v) in uv {
        graph[u].push(v);
        graph[v].push(u);
    }
    graph
}

fn f(s: usize, uv: &[Vec<usize>]) -> usize {
    let mut set = HashSet::new(); // visited
    set.insert(0);
    set.insert(s);
    let mut stack = vec![s];
    while let Some(i) = stack.pop() {
        for &j in &uv[i] {
            if !set.contains(&j) {
                set.insert(j);
                stack.push(j);
            }
        }
    }
    let result = set.len() - 1;
    result
}

fn main() {
    input! {
        n: usize,
        uv: [(Usize1, Usize1); n - 1],
    }
    let graph = build_ungraph(n, &uv);
    let mut v = vec![];
    for &i in &graph[0] {
        v.push(f(i, &graph));
    }
    let sum: usize = v.iter().sum();
    v.sort();
    let result = sum - v[v.len() - 1] + 1;
    println!("{result}");
}
