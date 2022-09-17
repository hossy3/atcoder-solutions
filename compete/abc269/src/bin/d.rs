use std::collections::{BTreeMap, BTreeSet};

use petgraph::unionfind::UnionFind;
use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    }
    let mut m = BTreeMap::new();
    let mut uf = UnionFind::new(n);

    for (i, &(x, y)) in xy.iter().enumerate() {
        m.insert((x, y), i);
        for (a, b) in &[(-1, -1), (0, -1), (-1, 0), (1, 0), (0, 1), (1, 1)] {
            if let Some(&j) = m.get(&(x + a, y + b)) {
                uf.union(i, j);
            }
        }
    }
    let mut s = BTreeSet::new();
    for i in 0..n {
        s.insert(uf.find(i));
    }
    println!("{}", s.len());
}
