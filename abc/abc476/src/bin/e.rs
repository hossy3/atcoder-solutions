use ac_library::{Max, Min, Segtree};
use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        p: [usize; n],
        lr: [(Usize1, Usize1); m],
    }

    const K: usize = 1usize << 32;
    let mut seg_min: Segtree<Min<_>> = p.iter().enumerate().map(|(i, &x)| x * K + i).collect();
    let mut seg_max: Segtree<Max<_>> = p.iter().enumerate().map(|(i, &x)| x * K + i).collect();
    for (l, r) in lr {
        let min = seg_min.prod(l..=r);
        let max = seg_max.prod(l..=r);
        let (i0, x0) = (min % K, min / K);
        let (i1, x1) = (max % K, max / K);
        seg_min.set(i0, x1 * K + i0);
        seg_min.set(i1, x0 * K + i1);
        seg_max.set(i0, x1 * K + i0);
        seg_max.set(i1, x0 * K + i1);
    }

    let result = (0..n).map(|i| seg_min.get(i) / K).join(" ");
    println!("{result}");
}
