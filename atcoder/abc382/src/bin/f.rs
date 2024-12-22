use ac_library::{LazySegtree, MapMonoid, Min, Monoid};
use proconio::{input, marker::Usize1};

struct MinMin;
impl MapMonoid for MinMin {
    type M = Min<usize>;
    type F = usize;

    fn identity_map() -> Self::F {
        usize::MAX
    }
    fn mapping(&f: &Self::F, &x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        f.min(x)
    }
    fn composition(&f: &Self::F, &g: &Self::F) -> Self::F {
        f.min(g)
    }
}

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        rcl: [(Usize1, Usize1, usize); n],
    }

    let mut ricl = vec![];
    for (i, &(r, c, l)) in rcl.iter().enumerate() {
        ricl.push((r, i, c, l));
    }
    ricl.sort();
    ricl.reverse();

    let mut results = vec![0; n];
    let mut segtree: LazySegtree<MinMin> = vec![h; w].into();
    for &(_, i, c, l) in &ricl {
        let x = segtree.prod(c..(c + l));
        results[i] = x;
        segtree.apply_range(c..(c + l), x - 1);
    }

    for i in 0..n {
        println!("{}", results[i]);
    }
}
