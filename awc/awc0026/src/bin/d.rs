use std::collections::{BTreeMap, BTreeSet};

use proconio::input;

// 座標圧縮用
fn build_map(lr: &[(usize, usize)]) -> BTreeMap<usize, usize> {
    let mut set = BTreeSet::new();
    for &(l, r) in lr {
        set.insert(l);
        set.insert(r);
    }

    let mut map = BTreeMap::new();
    for (i, &x) in set.iter().enumerate() {
        map.insert(x, i);
    }
    map
}

fn main() {
    input! {
        n: usize,
        k: isize,
        lr: [(usize, usize); n],
    }

    let map = build_map(&lr);
    let v = map.iter().map(|(&k, _)| k).collect::<Vec<_>>();
    let mut imos = vec![0isize; map.len() + 1];

    for &(l, r) in &lr {
        let l = map[&l];
        let r = map[&r];
        imos[l] += 1;
        imos[r] -= 1;
    }

    for i in 0..(map.len()) {
        imos[i + 1] += imos[i];
    }

    let mut result = 0usize;
    for i in 0..(map.len()) {
        if imos[i] >= k {
            result += v[i + 1] - v[i];
        }
    }
    println!("{result}");
}
