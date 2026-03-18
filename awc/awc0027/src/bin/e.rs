use std::collections::{BTreeMap, BTreeSet};

use ac_library::FenwickTree;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: isize,
        k: isize,
        a: [isize; n],
    }

    let mut v = vec![0isize; n];
    for (i, &x) in a.iter().enumerate() {
        v[i] = x + m;
    }
    for i in 0..(n - 1) {
        v[i + 1] += v[i];
    }

    let mut set = BTreeSet::new(); // 座標圧縮
    for &x in &v {
        set.insert(x);
    }
    let mut map = BTreeMap::new();
    for (i, &x) in set.iter().enumerate() {
        map.insert(x, i);
    }

    let mut fenwick = FenwickTree::new(set.len() + 1, 0isize);
    for &x in &v {
        fenwick.add(*map.get(&x).unwrap(), 1);
    }

    let mut result = 0isize;
    for (i, &x) in v.iter().enumerate() {
        let offset = if i == 0 { 0 } else { v[i - 1] };
        if let Some((_, l)) = map.range(..=(k + offset)).last() {
            result += fenwick.sum(..=l);
        }
        fenwick.add(*map.get(&x).unwrap(), -1);
    }
    println!("{result}");
}
