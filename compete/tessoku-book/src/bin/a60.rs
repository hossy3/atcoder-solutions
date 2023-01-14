use std::collections::BTreeMap;
use std::ops::Bound::{Excluded, Unbounded};

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut map = BTreeMap::new();
    let mut v = Vec::with_capacity(n);
    for (i, &a) in a.iter().enumerate() {
        while let Some((&k, _)) = map.range((Unbounded, Excluded(a))).next() {
            map.remove(&k);
        }
        if let Some((_, &j)) = map.iter().next() {
            v.push(j);
        } else {
            v.push(-1);
        }
        map.insert(a, (i + 1) as i64);
    }
    let result = v.iter().join(" ");
    println!("{}", result);
}
