use std::collections::HashSet;

use ac_library::Dsu;
use itertools::Itertools;
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
        k: usize,
        ab: [(Usize1, Usize1); m],
        cd: [(Usize1, Usize1); k],
    }

    // つながっている友達関係をグループ化する
    let mut dsu = Dsu::new(n);
    for &(a, b) in &ab {
        dsu.merge(a, b);
    }

    let ab = build_ungraph(n, &ab); // 友達関係
    let cd = build_ungraph(n, &cd); // ブロック関係

    let mut results = vec![0usize; n];
    for v in dsu.groups() {
        let set: HashSet<_> = v.iter().copied().collect();
        for &i in &v {
            let mut count = set.len() - 1; // 自分自身は友達にならない
            for u in &ab[i] {
                if set.contains(u) {
                    count -= 1; // すでに友達関係
                }
            }
            for u in &cd[i] {
                if set.contains(u) {
                    count -= 1; // ブロック関係があるので友達にならない
                }
            }
            results[i] = count;
        }
    }

    println!("{}", results.iter().join(" "));
}
