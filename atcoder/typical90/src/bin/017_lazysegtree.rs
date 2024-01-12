use std::cmp::Reverse;

use ac_library::{LazySegtree, MapMonoid, Max};
use proconio::{input, marker::Usize1};

struct MaxAdd;
impl MapMonoid for MaxAdd {
    type M = Max<i64>;
    type F = i64;

    fn identity_map() -> Self::F {
        0
    }

    fn mapping(&f: &i64, &x: &i64) -> i64 {
        f + x
    }

    fn composition(&f: &i64, &g: &i64) -> i64 {
        f + g
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        mut lr: [(Usize1, Usize1); m],
    }

    lr.sort_by_key(|&(l, r)| (r, Reverse(l)));
    let mut result = 0i64;
    let mut segtree: LazySegtree<MaxAdd> = vec![0i64; n].into();
    for &(l, r) in &lr {
        result += segtree.get(l);
        segtree.apply_range((l + 1)..r, 1);
    }

    println!("{result}");
}
