use std::collections::BTreeMap;
use std::ops::Bound::{Excluded, Included, Unbounded};

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
        let (_, &hl) = m.range((Unbounded, Excluded(l + 1))).last().unwrap();
        let (_, &hr) = m.range((Unbounded, Excluded(r + 2))).last().unwrap();

        let mut height = hl + 1;
        if l < r {
            // Rust 1.53-: m.retain(|&k, _| k < l || k > r);
            let v = m
                .range((Included(l + 1), Excluded(r + 1)))
                .map(|(&k, _)| k)
                .collect_vec();
            for k in &v {
                if let Some(h) = m.remove(k) {
                    height = height.max(h + 1);
                }
            }
        }

        m.insert(l, height);
        m.insert(r + 1, hr);

        println!("{}", height);
    }
}
