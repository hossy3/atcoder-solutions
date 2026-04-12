use std::{
    cmp::Reverse,
    collections::{BTreeSet, BinaryHeap, HashSet},
};

use ac_library::Dsu;
use proconio::input;

/// 非負の重み付きグラフの s から e までの最短距離をダイクストラ法で解く
fn shortest_graph(graph: &[Vec<(usize, usize)>], s: usize, e: usize) -> Option<usize> {
    let mut set = HashSet::new();
    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), s));

    while let Some((Reverse(step), i)) = heap.pop() {
        if i == e {
            return Some(step);
        }
        if set.insert(i) {
            for &(j, w) in &graph[i] {
                if !set.contains(&j) {
                    heap.push((Reverse(step + w), j));
                }
            }
        }
    }
    None
}

fn f(a: &[Vec<usize>]) -> bool {
    let n = a.len() + 1;

    let mut set = BTreeSet::new();
    for i in 0..(n - 1) {
        for j in 0..(n - i - 1) {
            set.insert((a[i][j], i, i + j + 1));
        }
    }

    let mut graph = vec![vec![]; n]; // 無向グラフ
    let mut dsu = Dsu::new(n);

    for (x, i, j) in set {
        if dsu.same(i, j) {
            let dist = shortest_graph(&graph, i, j);
            if dist != Some(x) {
                return false;
            }
        } else {
            graph[i].push((j, x));
            graph[j].push((i, x));
            dsu.merge(i, j);
        }
    }

    true
}

fn main() {
    input! {
        n: usize,
    }

    let mut a = vec![];
    for i in 1..n {
        input! {
            a0: [usize; n - i],
        }
        a.push(a0);
    }

    let yes = f(&a);
    println!("{}", if yes { "Yes" } else { "No" });
}
