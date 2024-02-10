use ac_library::{LazySegtree, MapMonoid, Max, Monoid};
use itertools::Itertools;
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
        m: usize,
        a: [usize; n],
        b: [usize; m],
    }

    let mut segtree: LazySegtree<MaxAdd> = a.into();
    for b in b {
        let mut x = segtree.get(b);
        segtree.set(b, 0);

        let b0 = (b + x + 1).min(n);
        segtree.apply_range((b + 1)..b0, 1);

        x -= b0 - (b + 1);
        let y = x / n;
        segtree.apply_range(0..n, y);

        x -= y * n;
        let b1 = x as usize;
        segtree.apply_range(0..b1, 1);
    }

    let mut results = vec![];
    for i in 0..n {
        results.push(segtree.get(i));
    }

    let result = results.iter().join(" ");
    println!("{result}");
}
