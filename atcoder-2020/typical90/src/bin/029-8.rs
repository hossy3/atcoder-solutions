use std::collections::BTreeSet;
use std::ops::Bound::{Excluded, Unbounded};

use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        w: usize,
        lr: [(Usize1, Usize1)],
    }

    let mut v = vec![0usize; w + 1]; // vec<height>
    let mut s = BTreeSet::new(); // available indices
    s.insert(0usize);
    s.insert(w);

    for &(l, r) in &lr {
        let hl = v[*s.range((Unbounded, Excluded(l + 1))).last().unwrap()];
        let hr = v[*s.range((Unbounded, Excluded(r + 2))).last().unwrap()];

        let mut height = hl + 1;
        while let Some(&i) = s.range((Unbounded, Excluded(r + 1))).last() {
            if i <= l {
                break;
            }
            height = height.max(v[i] + 1);
            s.remove(&i);
        }

        v[l] = height;
        s.insert(l);
        v[r + 1] = hr;
        s.insert(r + 1);

        println!("{}", height);
    }
}
