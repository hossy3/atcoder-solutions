use std::collections::BinaryHeap;

use ac_library::Dsu;
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

    let graph = build_digraph(n, &uv); // v to u

    // 接続できるかどうか
    let mut connected = vec![true; n];
    let mut dsu = Dsu::new(n);
    let mut unconnected = BinaryHeap::new();
    for v in 0..n {
        for &u in &graph[v] {
            dsu.merge(u, v);
        }
        unconnected.push(v); // 仮候補
        while let Some(x) = unconnected.pop() {
            if !dsu.same(0, x) {
                connected[v] = false;
                unconnected.push(x);
                break;
            }
        }
    }

    // 累積和で影響を数える
    let mut cum = vec![0i64; n];
    for v in (0..n).rev() {
        let v0 = *graph[v].iter().min().unwrap_or(&v);
        cum[v0] += 1;
        cum[v] -= 1;
    }
    let mut results = vec![0i64; n + 1];
    for i in 0..n {
        results[i + 1] = results[i] + cum[i];
    }

    for i in 0..n {
        if connected[i] {
            println!("{}", results[i + 1]);
        } else {
            println!("-1");
        }
    }
}
