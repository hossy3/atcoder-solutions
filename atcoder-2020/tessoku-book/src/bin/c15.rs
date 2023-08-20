use std::collections::BTreeMap;
use std::ops::Bound::{Included, Unbounded};

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i64,
        lr: [(i64, i64); n],
    }

    let mut ml = BTreeMap::new();
    {
        let mut v = lr.clone();
        for i in 0..n {
            v[i].1 += k;
        }
        v.sort_by_key(|x| x.1);
        let mut t = 0;
        let mut count = 0usize;
        for (l, r) in v {
            if l >= t {
                t = r;
                count += 1;
                ml.insert(t, count);
            }
        }
    }

    let mut mr = BTreeMap::new();
    {
        let mut v = lr.clone();
        for i in 0..n {
            v[i].0 -= k;
        }
        v.sort_by_key(|x| -x.0);
        let mut t = 86400;
        let mut count = 0usize;
        for (l, r) in v {
            if r <= t {
                t = l;
                count += 1;
                mr.insert(t, count);
            }
        }
    }

    for (l, r) in lr {
        let count = 1
            + ml.range((Unbounded, Included(&l)))
                .last()
                .and_then(|x| Some(*x.1))
                .unwrap_or(0)
            + mr.range((Included(&r), Unbounded))
                .next()
                .and_then(|x| Some(*x.1))
                .unwrap_or(0);
        println!("{}", count);
    }
}
