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

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    }
    const MOD: usize = 1_000_000_007;

    let graph = build_ungraph(n, &ab);
    let mut v = vec![(std::usize::MAX, 0usize); n];
    v[0] = (0, 1);
    let mut queue = VecDeque::new();
    queue.push_back((0, 0));
    while let Some((dist, i)) = queue.pop_front() {
        let dist = dist + 1;
        for &j in &graph[i] {
            if v[j].0 > dist {
                v[j].0 = dist;
                v[j].1 = (v[j].1 + v[i].1) % MOD;
                queue.push_back((dist, j));
            } else if v[j].0 == dist {
                v[j].1 = (v[j].1 + v[i].1) % MOD;
            }
        }
    }

    let result = v[n - 1].1;
    println!("{}", result);
}
