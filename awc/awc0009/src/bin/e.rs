use ac_library::{Max, Min, Segtree};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n],
        lr: [(Usize1, Usize1); q],
    }

    let segtree_max: Segtree<Max<_>> = a.iter().copied().collect();
    let segtree_min: Segtree<Min<_>> = a.iter().copied().collect();
    for (l, r) in lr {
        let max = segtree_max.prod(l..=r);
        let min = segtree_min.prod(l..=r);
        println!("{}", max - min);
    }
}
