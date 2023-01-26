use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet};
use std::ops::Bound::{Excluded, Unbounded};

use proconio::input;

// LIS = Longest increasing subsequence

fn main() {
    input! {
        n: usize,
        mut xy: [(usize, usize); n],
    }
    xy.sort_by_key(|&(x, y)| (x, Reverse(y))); // disallow (1,2), (1,3)
    let mut m = BTreeMap::new();
    for &(_, y) in &xy {
        let mut value = *m.get(&y).unwrap_or(&1);
        if let Some((_, c)) = m.range((Unbounded, Excluded(y))).last() {
            value = value.max(c + 1);
        }
        m.insert(y, value);

        let mut s = BTreeSet::new();
        for (&k, v) in m.range((Excluded(y), Unbounded)) {
            if *v <= value {
                s.insert(k);
            }
        }
        for k in &s {
            m.remove(k);
        }
    }

    let result = m.values().max().unwrap();
    println!("{}", result);
}
