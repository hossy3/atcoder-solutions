use std::collections::{BTreeSet, HashMap};

use ac_library::{LazySegtree, MapMonoid, Monoid};
use proconio::input;

pub struct Abc435EM;
impl Monoid for Abc435EM {
    type S = (u32, u32); // (現在の値, 最大の値)
    fn identity() -> Self::S {
        (0, 0)
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        (a.0 + b.0, a.1 + b.1)
    }
}

struct Abc435E;
impl MapMonoid for Abc435E {
    type M = Abc435EM;
    type F = u32;

    fn identity_map() -> Self::F {
        0
    }
    fn mapping(&f: &Self::F, &x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        if f == 1 {
            (x.1, x.1)
        } else {
            x
        }
    }
    fn composition(&f: &Self::F, &g: &Self::F) -> Self::F {
        f.max(g)
    }
}

fn main() {
    input! {
        n: u32,
        q: usize,
        lr: [(u32, u32); q],
    }

    let mut set = BTreeSet::new(); // l, r を座標圧縮する
    for &(l, r) in &lr {
        set.insert(l);
        set.insert(r);
    }
    let v0: Vec<_> = set.iter().collect();

    let mut v = vec![(0u32, 1u32)];
    let mut map = HashMap::new();
    map.insert(v0[0], 0);

    for i in 1..(v0.len()) {
        let x = v0[i] - v0[i - 1] - 1;
        if x > 0 {
            v.push((0, x));
        }
        map.insert(v0[i], v.len());
        v.push((0, 1));
    }

    let mut segtree: LazySegtree<Abc435E> = v.into();
    for &(l, r) in &lr {
        let Some(l) = map.get(&l) else { unreachable!() };
        let Some(r) = map.get(&r) else { unreachable!() };
        segtree.apply_range(l..=r, 1);
        let result = n - segtree.all_prod().0;
        println!("{result}");
    }
}
