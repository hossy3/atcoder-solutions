use std::collections::BTreeMap;
use std::ops::Bound::{Excluded, Unbounded};

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
        while let Some((&i, &h)) = m.range((Unbounded, Excluded(r + 1))).last() {
            if i <= l {
                break;
            }
            height = height.max(h + 1);
            m.remove(&i);
        }

        m.insert(l, height);
        m.insert(r + 1, hr);

        println!("{}", height);
    }
}
