use std::collections::HashMap;

use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(Usize1, Usize1); m],
    }
    if n != m + 1 {
        println!("No");
        return;
    }

    let mut map = HashMap::new();
    let mut uf = UnionFind::new(n);
    for &(u, v) in &uv {
        if uf.equiv(u, v) {
            println!("No");
            return;
        }

        if let Some(&j) = map.get(&u) {
            if j == 2 {
                println!("No");
                return;
            }
            map.insert(u, j + 1);
        } else {
            map.insert(u, 1);
        }

        if let Some(&j) = map.get(&v) {
            if j == 2 {
                println!("No");
                return;
            }
            map.insert(v, j + 1);
        } else {
            map.insert(v, 1);
        }
        uf.union(u, v);
    }

    println!("Yes");
}
