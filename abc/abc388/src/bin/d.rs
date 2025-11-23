use ac_library::{Additive, LazySegtree, MapMonoid, Monoid};
use itertools::Itertools;
use proconio::input;

struct AddAdd;
impl MapMonoid for AddAdd {
    type M = Additive<usize>;
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
        a: [usize; n],
    }

    let mut segtree: LazySegtree<AddAdd> = a.into();
    for i in 0..n {
        let x = segtree.get(i);
        let y = x.min(n - i - 1); // これから渡す石の数
        if y > 0 {
            segtree.apply_range((i + 1)..=(i + y), 1);
            segtree.set(i, x - y);
        }
    }
    let mut results = vec![];
    for i in 0..n {
        results.push(segtree.get(i));
    }
    println!("{}", results.iter().join(" "));
}
