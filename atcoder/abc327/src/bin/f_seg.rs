use std::collections::BTreeMap;

use ac_library::{Monoid, Segtree};
use proconio::input;

pub struct MaxAdd;
impl Monoid for MaxAdd {
    type S = (i32, i32);
    fn identity() -> Self::S {
        (0, 0)
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        (a.0 + b.0, a.1.max(a.0 + b.1))
    }
}

fn main() {
    input! {
        n: usize,
        d: usize,
        w: usize,
        tx: [(usize, usize); n],
    }

    let t_max = *tx.iter().map(|(t, _)| t).max().unwrap_or(&1);
    let x_max = *tx.iter().map(|(_, x)| x).max().unwrap_or(&1);
    let mut queue = BTreeMap::new();
    for &(t, x) in &tx {
        let (t0, t1) = (t - 1, t + d - 1);
        let (x0, x1) = (x - 1, x + w - 1);
        queue
            .entry(t0)
            .or_insert(vec![])
            .push((x0, x1.min(x_max), 1));
        if t1 < t_max {
            queue
                .entry(t1)
                .or_insert(vec![])
                .push((x0, x1.min(x_max), -1));
        }
    }

    let mut result = 0;
    let mut segtree: Segtree<MaxAdd> = vec![(0, 0); x_max + 1].into();
    for (_, v) in queue {
        for (x0, x1, diff) in v {
            let mut y = segtree.get(x0);
            y.0 += diff;
            y.1 = y.0.max(0);
            segtree.set(x0, y);

            let mut y = segtree.get(x1);
            y.0 -= diff;
            y.1 = y.0.max(0);
            segtree.set(x1, y);
        }
        result = result.max(segtree.all_prod().1);
    }
    println!("{result}");
}
