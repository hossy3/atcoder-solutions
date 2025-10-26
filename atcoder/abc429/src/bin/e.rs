use std::collections::VecDeque;

use proconio::{
    input,
    marker::{Chars, Usize1},
};

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
        uv: [(Usize1, Usize1); m],
        s: Chars,
    }

    let graph = build_ungraph(n, &uv);

    let mut queue = VecDeque::new();
    let mut results = vec![vec![]; n]; // 各頂点にたどり着く、安全な頂点からの歩数
    for (i, &c) in s.iter().enumerate() {
        if c == 'S' {
            queue.push_back((i, i, 0)); // (次の場所, 先頭, 距離)
        }
    }

    while let Some((i, root, dist)) = queue.pop_front() {
        if results[i].len() >= 2 || results[i].iter().any(|&(j, _)| root == j) {
            continue;
        }
        results[i].push((root, dist));
        for &j in &graph[i] {
            queue.push_back((j, root, dist + 1));
        }
    }

    for (i, &c) in s.iter().enumerate() {
        if c == 'D' {
            let result = results[i][0].1 + results[i][1].1;
            println!("{result}");
        }
    }
}
