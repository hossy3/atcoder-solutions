use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

fn dijkstra_all(s: usize, graph: &[Vec<usize>]) -> Vec<Option<usize>> {
    let n = graph.len();
    let mut v = vec![None; n];
    let mut queue = VecDeque::new();
    v[s] = Some(0usize);
    queue.push_back((0, s));

    while let Some((step, i)) = queue.pop_front() {
        for &j in &graph[i] {
            if v[j].is_none() {
                v[j] = Some(step + 1);
                queue.push_back((step + 1, j));
            }
        }
    }
    v
}

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
    let dist0 = dijkstra_all(0, &graph);
    let mut i0 = 0; // 0から一番遠い点
    for i in 0..n {
        if dist0[i].unwrap() >= dist0[i0].unwrap() {
            i0 = i;
        }
    }

    let dist1 = dijkstra_all(i0, &graph);
    let mut i1 = 0; // i0から一番遠い点
    for i in 0..n {
        if dist1[i].unwrap() >= dist1[i1].unwrap() {
            i1 = i;
        }
    }

    let dist2 = dijkstra_all(i1, &graph);

    // i0 と i1 が木の直径の両端。どちらかが答えになる
    for i in 0..n {
        let Some(x0) = dist1[i] else { unreachable!() }; 
        let Some(x1) = dist2[i] else { unreachable!() }; 
        let result = if x0 > x1 || (x0 == x1 && i0 > i1) {
            i0 + 1
        } else {
            i1 + 1
        };
        println!("{result}");
    }
}
