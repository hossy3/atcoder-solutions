use std::collections::BTreeMap;
use std::ops::Bound::{Excluded, Included};

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

        let mut h_new = hl + 1;
        for (_, &h) in m.range((Excluded(l), Included(r))) {
            h_new = h_new.max(h + 1);
        }
        for (_, h) in m.range_mut((Excluded(l), Included(r))) {
            *h = h_new;
        }
        m.insert(l, h_new);
        m.insert(r + 1, hr);

        println!("{}", h_new);
    }
}
