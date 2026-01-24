use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

use proconio::{input, marker::Usize1};

fn build_ungraph(
    n: usize,
    uvab: &[(usize, usize, usize, usize)],
) -> Vec<Vec<(usize, usize, usize)>> {
    let mut graph = vec![vec![]; n]; // node, edge
    for &(u, v, a, b) in uvab {
        graph[u].push((v, a, b));
        graph[v].push((u, a, b));
    }
    graph
}

fn main() {
    input! {
        n: usize,
        m: usize,
        s: usize,
        uvab: [(Usize1, Usize1, usize, usize); m],
        cd: [(usize, usize); n],
    }

    let graph = build_ungraph(n, &uvab);
    let mut results = vec![usize::MAX; n]; // 都市への移動にかかる最小の時間
    results[0] = 0;
    let mut rest = n - 1; // 到達していない都市の数
    let a_sum: usize = uvab.iter().map(|&(_, _, a, _)| a).sum();

    let mut queue = BinaryHeap::new();
    queue.push((Reverse(0), 0, s)); // (時間, 位置, 銀貨)
    let mut set = HashSet::new();

    while let Some((Reverse(time), u, money)) = queue.pop() {
        if !set.insert((u, money)) {
            continue;
        }
        if results[u] > time {
            results[u] = time;
            rest -= 1;
            if rest == 0 {
                break;
            }
        }

        // 移動
        for &(v, a, b) in &graph[u] {
            if money >= a {
                queue.push((Reverse(time + b), v, money - a));
            }
        }

        // 交換
        let (c, d) = cd[u];
        if money < a_sum {
            queue.push((Reverse(time + d), u, money + c));
        }
    }

    for &result in &results[1..] {
        println!("{result}");
    }
}
