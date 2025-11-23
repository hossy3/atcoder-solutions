use std::mem::swap;

use ac_library::{LazySegtree, MapMonoid, Min, Monoid};
use proconio::{input, marker::Usize1};

struct MinAdd;
impl MapMonoid for MinAdd {
    type M = Min<usize>;
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
        x: [Usize1; m],
    }

    let mut result = 0;
    let mut segtree: LazySegtree<MinAdd> = vec![0usize; n].into();
    for v in x.windows(2) {
        let mut i = v[0];
        let mut j = v[1];
        if i > j {
            swap(&mut i, &mut j);
        }
        let len0 = j - i;
        let len1 = n + i - j;
        result += len0.min(len1);
        if len0 < len1 {
            let x = len1 - len0; // j.. 側を通るとこれだけ遠回り
            segtree.apply_range(i..j, x);
        } else {
            let x = len0 - len1; // i.. 側を通るとこれだけ遠回り
            segtree.apply_range(j..n, x);
            segtree.apply_range(0..i, x);
        }
    }

    result += segtree.all_prod();
    println!("{result}");
}
