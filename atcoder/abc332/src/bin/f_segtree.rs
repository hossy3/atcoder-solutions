use ac_library::{Monoid, Segtree};
use itertools::Itertools;
use proconio::{input, marker::Usize1};

type Mint = ac_library::ModInt998244353;

pub struct Abc332f;
impl Monoid for Abc332f {
    type S = (Mint, Mint);
    fn identity() -> Self::S {
        (Mint::new(1), Mint::new(0))
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        (a.0 * b.0, b.0 * a.1 + b.1)
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        lrx: [(Usize1, Usize1, usize); m],
    }

    let mut v = vec![vec![]; n + 1];
    for (i, &(l, r, x)) in lrx.iter().enumerate() {
        let p = Mint::new(1) / Mint::new(r - l + 1);
        v[l].push((i, (Mint::new(1) - p, p * Mint::new(x))));
        v[r + 1].push((i, (Mint::new(1), Mint::new(0))));
    }

    let mut results = vec![];
    let mut segtree: Segtree<Abc332f> = Segtree::new(m);
    for (v0, a0) in std::iter::zip(v, a) {
        for (i, x) in v0 {
            segtree.set(i, x);
        }
        let x = segtree.all_prod();
        results.push((x.0 * Mint::new(a0) + x.1).val());
    }

    println!("{}", results.iter().join(" "));
}
