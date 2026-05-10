use std::collections::BTreeSet;

use itertools::Itertools;
use proconio::{input, marker::Usize1};

/// [l, r) の範囲を x に置き換える
fn ordered_disjoint_tree_assign(set: &mut BTreeSet<(usize, usize)>, l: usize, r: usize, x: usize) {
    let (_, xl) = *set.range((0, 0)..(l, 0)).last().unwrap_or(&(0, usize::MAX));
    let (_, xr) = *set.range((0, 0)..(r + 1, 0)).last().unwrap();

    while let Some(&(i, c)) = set.range((l, 0)..(r + 1, 0)).last() {
        set.remove(&(i, c));
    }

    if xl != x {
        set.insert((l, x));
    }
    if xr != x {
        set.insert((r, xr));
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        lrc: [(Usize1, Usize1, usize); m],
    }

    let mut set = BTreeSet::new();
    set.insert((0, 0));

    for (l, r, c) in lrc {
        ordered_disjoint_tree_assign(&mut set, l, r + 1, c);
    }

    let mut results = vec![];
    let mut cur = 0;
    for i in 0..n {
        if let Some(&(_, c)) = set.range((i, 0)..(i + 1, 0)).next() {
            cur = c;
        }
        results.push(cur);
    }
    println!("{}", results.iter().join(" "));
}
