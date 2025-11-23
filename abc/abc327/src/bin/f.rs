use std::collections::BTreeMap;

use ac_library::{LazySegtree, MapMonoid, Max};
use proconio::input;

struct MaxAdd;
impl MapMonoid for MaxAdd {
    type M = Max<i32>;
    type F = i32;

    fn identity_map() -> Self::F {
        0
    }

    fn mapping(&f: &i32, &x: &i32) -> i32 {
        f + x
    }

    fn composition(&f: &i32, &g: &i32) -> i32 {
        f + g
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
    let mut segtree: LazySegtree<MaxAdd> = vec![0; x_max + 1].into();
    for (_, v) in queue {
        for (x0, x1, diff) in v {
            segtree.apply_range(x0..x1, diff);
        }
        result = result.max(segtree.all_prod());
    }
    println!("{result}");
}
