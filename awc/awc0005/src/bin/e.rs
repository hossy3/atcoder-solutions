use ac_library::{Max, Segtree};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
        lr: [(Usize1, Usize1); q],
    }

    let segtree: Segtree<Max<_>> = a.into();
    for (l, r) in lr {
        let max = segtree.prod(l..=r);
        println!("{}", max);
    }
}
