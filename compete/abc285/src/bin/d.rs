use std::collections::HashMap;

use petgraph::unionfind::UnionFind;
use proconio::input;

fn main() {
    input! {
        n: usize,
        st: [(String, String); n],
    }

    let mut map = HashMap::new();
    for (s, t) in &st {
        if !map.contains_key(&s) {
            map.insert(s, map.len());
        }
        if !map.contains_key(&t) {
            map.insert(t, map.len());
        }
    }

    let mut uf = UnionFind::new(map.len());
    for (s, t) in &st {
        let i = *map.get(&s).unwrap();
        let j = *map.get(&t).unwrap();
        if uf.equiv(i, j) {
            println!("No");
            return;
        }
        uf.union(i, j);
    }
    println!("Yes");
}
