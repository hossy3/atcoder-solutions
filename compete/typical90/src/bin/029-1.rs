use std::collections::BTreeMap;
use std::ops::Bound::Included;

use itertools::Itertools;
use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        w: usize,
        lr: [(Usize1, Usize1)],
    }
    let mut m = BTreeMap::new();
    m.insert(0usize, 0usize);
    m.insert(w, 0);

    for &(l, r) in &lr {
        let (_, &hl) = m.range((Included(0), Included(l))).last().unwrap();
        let (_, &hr) = m.range((Included(0), Included(r + 1))).last().unwrap();

        // Rust 1.53-: m.retain(|&k, _| k < l || k > r);
        let v = m
            .range((Included(l), Included(r)))
            .map(|(&k, _)| k)
            .collect_vec();
        let mut height = hl + 1;
        for k in &v {
            if let Some(h) = m.remove(k) {
                height = height.max(h + 1);
            }
        }

        m.insert(l, height);
        m.insert(r + 1, hr);

        println!("{}", height);
    }
}
