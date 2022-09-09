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
        let (_, &hl) = m.range((Included(1), Included(l))).last().unwrap();
        let (_, &hr) = m.range((Included(1), Included(r + 1))).last().unwrap();

        // Rust 1.53-: m.retain(|&k, _| k < l || k > r);
        let v = m
            .range((Included(l), Included(r)))
            .map(|(&k, _)| k)
            .collect_vec();
        let mut h_next = hl + 1;
        for k in &v {
            if let Some(h) = m.remove(k) {
                h_next = h_next.max(h + 1);
            }
        }

        m.insert(l, h_next);
        m.insert(r + 1, hr);

        println!("{}", h_next);
    }
}
