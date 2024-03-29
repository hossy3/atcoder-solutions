use ac_library::{LazySegtree, MapMonoid, Max, Monoid};
use proconio::{input, marker::Usize1};

struct MaxMax;
impl MapMonoid for MaxMax {
    type M = Max<usize>;
    type F = usize;

    fn identity_map() -> Self::F {
        0
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
        lr: [(Usize1, Usize1)],
    }

    let mut segtree: LazySegtree<MaxMax> = vec![0usize; w].into();
    for &(l, r) in &lr {
        let h = segtree.prod(l..=r) + 1;
        segtree.apply_range(l..=r, h);
        println!("{h}");
    }
}
