use ac_library::{LazySegtree, MapMonoid, Max, Monoid};
use proconio::input;

struct MaxMax;
impl MapMonoid for MaxMax {
    type M = Max<i64>;
    type F = i64;

    fn identity_map() -> Self::F {
        i64::MIN
    }
    fn mapping(&f: &Self::F, &x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        f.max(x)
    }
    fn composition(&f: &Self::F, &g: &Self::F) -> Self::F {
        f.max(g)
    }
}

fn main() {
    input! {
        w: usize,
        n: usize,
        lrv: [(usize, usize, i64); n],
    }

    let mut segtree: LazySegtree<MaxMax> = vec![i64::MIN; w + 1].into();
    segtree.set(0, 0);

    for (l, r, v) in lrv {
        for i in (0..=(w - l)).rev() {
            let x = segtree.get(i);
            if x >= 0 {
                segtree.apply_range((i + l)..=w.min(i + r), x + v);
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
