use std::collections::HashSet;

use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        uv: [(Usize1, Usize1)],
        xy: [(Usize1, Usize1)],
        pq: [(Usize1, Usize1)],
    }

    let mut uf = UnionFind::new(n);
    for &(u, v) in &uv {
        uf.union(u, v);
    }

    let mut set = HashSet::new();
    for &(x, y) in &xy {
        set.insert((uf.find(x), uf.find(y)));
        set.insert((uf.find(y), uf.find(x)));
    }

    for &(p, q) in &pq {
        let yes = !set.contains(&(uf.find(p), uf.find(q)));
        println!("{}", if yes { "Yes" } else { "No" });
    }
}
