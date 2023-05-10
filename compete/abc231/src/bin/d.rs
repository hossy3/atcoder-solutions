use std::collections::HashMap;

use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    }

    let mut uf = UnionFind::new(n);
    for &(a, b) in &ab {
        uf.union(a, b);
    }

    let mut map = HashMap::new();
    for &(a, b) in &ab {
        let v = vec![a, b];
        for i in v {
            let j = uf.find(i);
            let m = map.entry(j).or_insert(HashMap::new());
            *m.entry(i).or_insert(0) += 1;
        }
    }

    let yes = map.iter().all(|(_, m)| {
        m.iter().all(|(_, &count)| count <= 2) && m.iter().any(|(_, &count)| count == 1)
    });
    println!("{}", if yes { "Yes" } else { "No" });
}
