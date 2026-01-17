use std::collections::{HashSet, VecDeque};

use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn build_digraph(n: usize, uvc: &[(usize, usize, usize)]) -> Vec<Vec<(usize, usize)>> {
    let mut graph = vec![vec![]; n]; // node, edge
    for &(u, v, c) in uvc {
        graph[u].push((v, c));
    }
    graph
}

fn main() {
    input! {
        n: usize,
        m: usize,
        l: usize,
        s: usize,
        t: usize,
        uvc: [(Usize1, Usize1, usize); m],
    }
    let graph = build_digraph(n, &uvc);

    let mut queue = VecDeque::new();
    queue.push_back((0, 0, 0)); // 位置, 現在のコスト, 歩数
    let mut set = HashSet::new();
    set.insert(queue[0]);
    let mut results = vec![false; n]; // 到着可能

    while let Some((u, cost, len)) = queue.pop_front() {
        for &(u, c) in &graph[u] {
            let cost = cost + c;
            if cost > t {
                continue;
            }
            let len = len + 1;
            let tuple = (u, cost, len);
            if !set.insert(tuple) {
                continue;
            }
            if len == l {
                if s <= cost && cost <= t {
                    results[u] = true;
                }
            } else {
                queue.push_back(tuple);
            }
        }
    }

    let result = results
        .iter()
        .enumerate()
        .filter(|&(_, &b)| b)
        .map(|(i, _)| i + 1)
        .join(" ");
    println!("{result}");
}
