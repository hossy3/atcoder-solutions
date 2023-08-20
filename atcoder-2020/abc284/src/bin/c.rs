use std::collections::HashSet;

use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(Usize1, Usize1); m],
    }

    let mut uf = UnionFind::new(n);
    for &(u, v) in &uv {
        uf.union(u, v);
    }

    let mut s = HashSet::new();
    for i in 0..n {
        s.insert(uf.find(i));
    }
    let count = s.len();
    println!("{}", count);
}
