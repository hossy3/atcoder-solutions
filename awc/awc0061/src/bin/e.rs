use std::{collections::BTreeMap, iter};

use ac_library::{Max, Segtree};
use proconio::input;

/// 座標圧縮 (Compress coordinates)
fn compress_coordinates(a: &[isize]) -> Vec<usize> {
    let mut sorted = a.to_vec();
    sorted.sort_unstable();
    sorted.dedup();
    let mut map = std::collections::HashMap::new();
    for (i, &x) in sorted.iter().enumerate() {
        map.insert(x, i);
    }
    a.iter().map(|&x| map[&x]).collect()
}

fn main() {
    input! {
        n: usize,
        d: isize,
        h: [isize; n],
    }

    let h0 = compress_coordinates(&h);
    let mut map = BTreeMap::new();
    for (&h, &h0) in iter::zip(h.iter(), h0.iter()) {
        map.insert(h, h0);
    }

    let mut segtree = Segtree::<Max<usize>>::new(n);
    for (i, &h0) in h0.iter().enumerate() {
        let h_l = map.range((h[i] - d)..).next().unwrap_or((&0, &n)).1;
        let h_r = map.range(..=(h[i] + d)).last().unwrap_or((&0, &0)).1;
        let x = segtree.prod(h_l..=h_r);
        segtree.set(h0, x + 1);
    }

    let result = segtree.all_prod();
    println!("{result}");
}
