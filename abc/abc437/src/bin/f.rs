use itertools::Itertools;
use proconio::{input, marker::Usize1};

use ac_library::{Max, Min, Segtree};

fn main() {
    input! {
        n: usize,
        q: usize,
        xy: [(isize, isize); n],
    }

    let mut segtree0 = Segtree::<Min<_>>::from(xy.iter().map(|&(x, y)| x + y).collect_vec());
    let mut segtree1 = Segtree::<Max<_>>::from(xy.iter().map(|&(x, y)| x + y).collect_vec());
    let mut segtree2 = Segtree::<Min<_>>::from(xy.iter().map(|&(x, y)| x - y).collect_vec());
    let mut segtree3 = Segtree::<Max<_>>::from(xy.iter().map(|&(x, y)| x - y).collect_vec());

    for _ in 0..q {
        input! {
            t: u32,
        }
        if t == 1 {
            input! {
                i: Usize1,
                x: isize,
                y: isize,
            }
            segtree0.set(i, x + y);
            segtree1.set(i, x + y);
            segtree2.set(i, x - y);
            segtree3.set(i, x - y);
        } else if t == 2 {
            input! {
                l: Usize1,
                r: Usize1,
                x: isize,
                y: isize,
            }
            let r0 = (x + y) - segtree0.prod(l..=r);
            let r1 = segtree1.prod(l..=r) - (x + y);
            let r2 = (x - y) - segtree2.prod(l..=r);
            let r3 = segtree3.prod(l..=r) - (x - y);
            let result = r0.max(r1).max(r2).max(r3);
            println!("{result}");
        }
    }
}
