use std::{
    collections::{BTreeMap, BTreeSet},
    ops::Bound::{Included, Unbounded},
};

use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
        r: usize,
        x: [usize; n],
    }

    let mut map = BTreeMap::new();
    for (i, &j) in x.iter().enumerate() {
        map.insert(j, i);
    }

    let mut set = BTreeSet::new();
    let mut add = vec![vec![]; n];
    let mut rem = vec![vec![]; n];

    const MAX: usize = 1 << 62;
    let mut v = vec![MAX; n];
    v[0] = 0;

    for i in 0..n {
        for &j in &add[i] {
            set.insert(j);
        }
        for &j in &rem[i] {
            set.remove(&j);
        }
        if let Some((j, _)) = set.iter().next() {
            v[i] = *j;
        }
        if v[i] < MAX {
            let t = (v[i] + 1, i);
            if let Some((_, &l)) = map.range((Included(x[i] + l), Unbounded)).next() {
                add[l].push(t);
            }
            if let Some((_, &r)) = map.range((Included(x[i] + r + 1), Unbounded)).next() {
                rem[r].push(t);
            }
        }
    }
    let result = v[n - 1];
    println!("{}", result);
}
