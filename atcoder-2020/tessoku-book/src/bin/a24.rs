use std::collections::{BTreeMap, BTreeSet};
use std::ops::Bound::{Excluded, Unbounded};

use proconio::input;

// LIS = Longest increasing subsequence

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut m = BTreeMap::new();
    for a in &a {
        let mut value = *m.get(a).unwrap_or(&1);
        if let Some((_, x)) = m.range((Unbounded, Excluded(*a))).last() {
            value = value.max(x + 1);
        }
        m.insert(*a, value);

        let mut s = BTreeSet::new();
        for (&k, v) in m.range((Excluded(*a), Unbounded)) {
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
