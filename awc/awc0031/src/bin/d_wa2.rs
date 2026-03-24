use ac_library::{LazySegtree, MapMonoid, Max, Monoid};
use proconio::input;

/// 遅延セグメント木 区間加算 区間最大値
struct MaxAdd;
impl MapMonoid for MaxAdd {
    type M = Max<isize>;
    type F = isize;

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
        mut l: [usize; n],
        mut r: [isize; m],
    }

    l.sort_unstable();
    l.reverse();
    r.sort_unstable();

    let mut segtree: LazySegtree<MaxAdd> = r.into();

    for &l in &l {
        let i = segtree.max_right(0, |x| x == 0 || x == isize::MIN);
        if i < m {
            segtree.apply_range(i..((i + l).min(m)), -1);
        }
    }

    let yes = segtree.all_prod() == 0;
    println!("{}", if yes { "Yes" } else { "No" });
}
