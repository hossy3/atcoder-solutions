use std::collections::{BTreeSet, BTreeMap};

use itertools::Itertools;
use proconio::input;

// coordinate compression (座標圧縮)

fn main() {
    input! {
        a: [usize],
    }

    let mut s = BTreeSet::new();
    for &a in &a {
        s.insert(a);
    }

    let mut m = BTreeMap::new();
    for (i, &x) in s.iter().enumerate() {
        m.insert(x, i + 1);
    }

    let result = a.iter().map(|x| m[x]).join(" ");
    println!("{}", result);
}
