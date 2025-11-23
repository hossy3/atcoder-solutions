use std::collections::HashMap;
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

    let mut m = HashMap::<usize, (usize, usize)>::new();
    for i in 0..n {
        let i = uf.find(i);
        if !m.contains_key(&i) {
            m.insert(i, (0, 0));
        }
        m[&i].0 += 1;
    }

    for &(u, _) in &uv {
        let i = uf.find(u);
        m[&i].1 += 1;
    }

    for &(_, (nodes, edges)) in &m {
        if nodes != edges {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
