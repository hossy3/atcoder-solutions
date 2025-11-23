use std::collections::{BTreeSet, BinaryHeap};

use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        uv: [(Usize1, Usize1); n - 1],
    }

    let mut g = vec![vec![]; n];
    for &(u, v) in &uv {
        g[u].push(v);
        g[v].push(u);
    }

    let i = (0..n).find(|&i| g[i].len() == 1).unwrap();
    let i = g[i][0]; // star center
    let mut set = BTreeSet::new();
    set.insert(i);

    let mut queue = BinaryHeap::new();
    queue.push(i);
    while let Some(i) = queue.pop() {
        for &j in &g[i] {
            for &k in &g[j] {
                if i == k {
                    continue;
                }
                for &l in &g[k] {
                    if j == l {
                        continue;
                    }
                    if set.insert(l) {
                        queue.push(l);
                    }
                }
            }
        }
    }

    let mut v = Vec::with_capacity(set.len());
    for &i in &set {
        v.push(g[i].len());
    }
    v.sort();
    println!("{}", v.iter().join(" "));
}
