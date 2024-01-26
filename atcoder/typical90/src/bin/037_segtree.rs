use ac_library::{Max, Segtree};
use proconio::input;

fn main() {
    input! {
        w: usize,
        n: usize,
        lrv: [(usize, usize, i64); n],
    }

    let mut segtree = Segtree::<Max<i64>>::new(w + 1);
    segtree.set(0, 0);

    for (l, r, v) in lrv {
        for i in (l..=w).rev() {
            let l0 = i.saturating_sub(r);
            let r0 = i - l;
            let x = segtree.prod(l0..=r0);
            if x >= 0 {
                segtree.set(i, segtree.get(i).max(x + v));
            }
        }
    }

    let result = segtree.get(w);
    if result >= 0 {
        println!("{result}");
    } else {
        println!("-1");
    }
}
