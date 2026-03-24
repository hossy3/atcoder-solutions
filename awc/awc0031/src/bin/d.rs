use ac_library::{LazySegtree, MapMonoid, Min, Monoid};
use proconio::input;

/// 遅延セグメント木 区間加算 区間最小値
struct MinAdd;
impl MapMonoid for MinAdd {
    type M = Min<isize>;
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
    r.reverse();

    let mut segtree: LazySegtree<MinAdd> = r.into();

    for &l in &l {
        let r0 = l.min(segtree.max_right(0, |x| x > 0));
        if r0 == 0 {
            continue;
        }

        // 取り除いた結果降順にならない場合のために、値を確認しておく
        let x0 = segtree.get(r0 - 1);
        let r1 = segtree.max_right(0, |x| x > x0);
        let r2 = segtree.max_right(0, |x| x >= x0);

        segtree.apply_range(0..r0, -1);

        // 取り除いた結果降順にならない場合は、順番を入れ替える
        if r2 > r0 {
            segtree.apply_range(r0..r2, -1);
            segtree.apply_range(r1..(r1 + r2 - r0), 1);
        }
    }

    let yes = segtree.get(0) == 0;
    println!("{}", if yes { "Yes" } else { "No" });
}
