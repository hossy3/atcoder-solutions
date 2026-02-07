use ac_library::{LazySegtree, MapMonoid, Max, Monoid};
use proconio::input;

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
        d: usize,
        a: isize,
        mut xh: [(usize, isize); n],
    }

    xh.sort_unstable();

    let segtree: Vec<_> = xh.iter().map(|&(_, h)| h).collect();
    let mut segtree: LazySegtree<MaxAdd> = segtree.into();

    let mut result = 0;
    for l in 0..n {
        let h = segtree.get(l);
        if h > 0 {
            let i = h / a + if h % a == 0 { 0 } else { 1 };
            let xr = xh[l].0 + 2 * d;
            let r = xh.partition_point(|&(x, _)| x <= xr);
            segtree.apply_range(l..r, -a * i);
            result += i;
            // eprintln!("l: {l}, r: {r}, i: {i}, xr: {xr}");
        }
    }
    println!("{result}");
}
