use std::collections::BTreeMap;
use std::ops::Bound::Included;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        w: usize,
        lr: [(usize, usize)],
    }
    let mut m = BTreeMap::new();
    m.insert(1usize, 0usize);
    m.insert(w + 1, 0);

    for &(l, r) in &lr {
        let (_, &l0) = m.range((Included(1), Included(l))).last().unwrap();
        let (_, &r0) = m.range((Included(1), Included(r + 1))).last().unwrap();

        // Rust 1.53-: m.retain(|&k, _| k < l || k > r);
        let v = m
            .range((Included(l), Included(r)))
            .map(|(&x, &y)| (x, y))
            .collect_vec();
        let mut h_max = l0;
        for &(k, h) in &v {
            m.remove(&k);
            h_max = h_max.max(h);
        }

        h_max += 1;
        m.insert(l, h_max);
        m.insert(r + 1, r0);

        println!("{}", h_max);
    }
}
