use std::collections::{BTreeMap, BTreeSet};

use petgraph::unionfind::UnionFind;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut set = BTreeSet::new();
    for &x in &a {
        set.insert(x);
    }

    let mut map = BTreeMap::new();
    for x in set {
        map.insert(x, map.len());
    }

    let mut uf = UnionFind::new(map.len());
    for i in 0..(n / 2) {
        uf.union(*map.get(&a[i]).unwrap(), *map.get(&a[n - i - 1]).unwrap());
    }

    let mut set = BTreeSet::new();
    for i in 0..map.len() {
        set.insert(uf.find(i));
    }

    let result = map.len() - set.len();
    println!("{}", result);
}
