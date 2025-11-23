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

fn main() {
    input! {
         n: usize,
         lr: [(usize, usize); n],
         q: usize,
    }

    let mut v = vec![];
    for i in 0..=500_000 {
        v.push(i);
    }
    v.push(999_999); // 番兵
    let mut segtree: LazySegtree<MaxAdd> = v.into();
    for &(l, r) in &lr {
        let l0 = segtree.max_right(0, |x| x < l);
        let r0 = segtree.max_right(0, |x| x <= r);
        segtree.apply_range(l0..r0, 1);
    }

    for _ in 0..q {
        input! {
             x: usize,
        }
        let result = segtree.get(x);
        println!("{result}");
    }
}
