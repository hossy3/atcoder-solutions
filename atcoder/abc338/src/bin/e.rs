use ac_library::{LazySegtree, MapMonoid, Max, Monoid};
use proconio::{input, marker::Usize1};

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

fn f(ab: &[(usize, usize)]) -> bool {
    let n = ab.len();
    let mut segtree: LazySegtree<MaxAdd> = vec![0usize; 2 * n].into();
    for &(a, b) in ab {
        let (a, b) = if a < b { (a, b) } else { (b, a) };
        if segtree.get(a) != segtree.get(b) {
            return true;
        }
        segtree.apply_range(a..b, 1);
    }
    false
}

fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1); n],
    }
    let yes = f(&ab);
    println!("{}", if yes { "Yes" } else { "No" });
}
