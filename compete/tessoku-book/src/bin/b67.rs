use std::collections::BinaryHeap;

use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};

// Maximum Spanning Tree

fn main() {
    input! {
        n: usize,
        abc: [(Usize1, Usize1, usize)],
    }

    let mut heap = BinaryHeap::new();
    for (a, b, c) in abc {
        heap.push((c, a, b));
    }

    let mut uf = UnionFind::new(n);
    let mut len = 0;
    while let Some((c, a, b)) = heap.pop() {
        if uf.union(a, b) {
            len += c;
        }
    }
    println!("{}", len);
}
