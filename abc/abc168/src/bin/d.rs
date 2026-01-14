use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

fn build_ungraph(n: usize, uv: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut graph = vec![vec![]; n]; // node, edge
    for &(u, v) in uv {
        graph[u].push(v);
        graph[v].push(u);
    }
    graph
}

fn shortest_parent_all_ungraph(s: usize, graph: &[Vec<usize>]) -> Vec<Option<usize>> {
    let n = graph.len();
    let mut v = vec![None; n];
    let mut queue = VecDeque::new();
    v[s] = Some(0usize);
    queue.push_back(0);

    while let Some(i) = queue.pop_front() {
        for &j in &graph[i] {
            if v[j] == None {
                v[j] = Some(i);
                queue.push_back(j);
            }
        }
    }
    v
}

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    }

    let graph = build_ungraph(n, &ab);
    let shortest = shortest_parent_all_ungraph(0, &graph);

    println!("Yes");
    for i in 1..n {
        let result = shortest[i].unwrap() + 1;
        println!("{result}");
    }
}
