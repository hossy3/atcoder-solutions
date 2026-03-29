use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn build_digraph(n: usize, uv: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut graph = vec![vec![]; n]; // node, edge
    for &(u, v) in uv {
        graph[u].push(v);
    }
    graph
}

fn main() {
    input! {
        n: usize,
        m: usize,
        mut t: [isize; n],
        uv: [(Usize1, Usize1); m],
    }

    let graph = build_digraph(n, &uv);
    let mut stack = vec![];

    for &(_, v) in &uv {
        t[v] -= 1;
        if t[v] == -1 {
            stack.push(v);
        }
    }

    while let Some(u) = stack.pop() {
        for &v in &graph[u] {
            t[v] -= 1;
            if t[v] == -1 {
                stack.push(v);
            }
        }
    }

    let results = (0..n)
        .filter(|&i| t[i] < 0)
        .map(|i| i + 1)
        .collect::<Vec<_>>();
    if results.len() > 0 {
        println!("{}", results.iter().join(" "));
    } else {
        println!("-1");
    }
}
