use std::collections::HashSet;

use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(Usize1, Usize1); m],
        k: usize,
        xy: [(Usize1, Usize1); k],
        q: usize,
        pq: [(Usize1, Usize1); q],
    }

    let mut uf = UnionFind::new(n);
    for &(u, v) in &uv {
        uf.union(u, v);
    }
    let mut set = HashSet::new();
    for &(x, y) in &xy {
        let (x, y) = (uf.find(x), uf.find(y));
        set.insert((x, y));
        set.insert((y, x));
    }
    for &(p, q) in &pq {
        let (p, q) = (uf.find(p), uf.find(q));
        let yes = !set.contains(&(p, q));
        println!("{}", if yes { "Yes" } else { "No" });
    }
}
