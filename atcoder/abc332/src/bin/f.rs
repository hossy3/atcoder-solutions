use ac_library::{LazySegtree, MapMonoid, Max};
use itertools::Itertools;
use proconio::{input, marker::Usize1};

type Mint = ac_library::ModInt998244353;

struct Abc332f;
impl MapMonoid for Abc332f {
    type M = Max<u32>;
    type F = (Mint, Mint);

    fn identity_map() -> Self::F {
        (Mint::new(1), Mint::new(0))
    }

    fn mapping(&f: &(Mint, Mint), &x: &u32) -> u32 {
        (Mint::new(x) * f.0 + f.1).val()
    }

    fn composition(&f: &(Mint, Mint), &g: &(Mint, Mint)) -> (Mint, Mint) {
        (f.0 * g.0, f.0 * g.1 + f.1)
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        lrx: [(Usize1, Usize1, usize); m],
    }

    let mut v = vec![0; n];
    for i in 0..n {
        v[i] = Mint::new(a[i]).val();
    }

    let mut segtree: LazySegtree<Abc332f> = v.into();

    for &(l, r, x) in &lrx {
        let p = Mint::new(1) / Mint::new(r - l + 1);
        segtree.apply_range(l..=r, (Mint::new(1) - p, Mint::new(x) * p));
    }

    let mut results = vec![0; n];
    for i in 0..n {
        results[i] = segtree.get(i);
    }
    println!("{}", results.iter().join(" "));
}
