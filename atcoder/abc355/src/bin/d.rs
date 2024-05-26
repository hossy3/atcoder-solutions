use std::collections::{BTreeMap, BTreeSet};

use ac_library::{LazySegtree, MapMonoid, Max, Monoid};
use proconio::input;

struct MaxAdd;
impl MapMonoid for MaxAdd {
    type M = Max<usize>;
    type F = usize;

    fn identity_map() -> Self::F {
        0
    }
    fn mapping(&f: &Self::F, &x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        f + x
    }
    fn composition(&f: &Self::F, &g: &Self::F) -> Self::F {
        f + g
    }
}

// 座標圧縮用
fn build_map(lr: &[(usize, usize)]) -> BTreeMap<usize, usize> {
    let mut set = BTreeSet::new();
    for &(l, r) in lr {
        set.insert(l);
        set.insert(r);
    }

    let mut map = BTreeMap::new();
    for (i, &x) in set.iter().enumerate() {
        map.insert(x, i);
    }
    map
}

fn main() {
    input! {
        n: usize,
        mut lr: [(usize, usize); n],
    }

    lr.sort();
    let map = build_map(&lr);

    let m = map.len();
    let mut segtree: LazySegtree<MaxAdd> = vec![0usize; m].into();
    let mut result = 0usize;
    for &(l, r) in lr.iter().rev() {
        let l0 = *map.get(&l).unwrap();
        let r0 = *map.get(&r).unwrap();
        result += segtree.get(r0);
        segtree.apply_range(l0.., 1);
    }
    println!("{result}");
}
