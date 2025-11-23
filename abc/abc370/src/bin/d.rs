use std::collections::BTreeSet;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        h: usize,
        w: usize,
        q: usize,
        rc: [(Usize1, Usize1); q],
    }

    let mut mrc = vec![BTreeSet::new(); h]; // mrc[row] = cols
    let mut mcr = vec![BTreeSet::new(); w]; // mcr[col] = rows
    for r in 0..h {
        for c in 0..w {
            mrc[r].insert(c);
            mcr[c].insert(r);
        }
    }

    for (r, c) in rc {
        if mrc[r].contains(&c) {
            mrc[r].remove(&c);
            mcr[c].remove(&r);
            continue;
        }

        if let Some(&c) = mrc[r].range(..c).last() {
            mrc[r].remove(&c);
            mcr[c].remove(&r);
        }
        if let Some(&c) = mrc[r].range((c + 1)..).next() {
            mrc[r].remove(&c);
            mcr[c].remove(&r);
        }
        if let Some(&r) = mcr[c].range(..r).last() {
            mrc[r].remove(&c);
            mcr[c].remove(&r);
        }
        if let Some(&r) = mcr[c].range((r + 1)..).next() {
            mrc[r].remove(&c);
            mcr[c].remove(&r);
        }
    }

    let result: usize = mrc.iter().map(|cols| cols.len()).sum();
    println!("{result}");
}
