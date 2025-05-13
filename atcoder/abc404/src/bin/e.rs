use ac_library::{LazySegtree, MapMonoid, Min, Monoid};
use proconio::input;

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

// 何手で豆を先頭に移動できるか
fn f(c: &[usize]) -> usize {
    let n = c.len();

    let mut segtree: LazySegtree<MinMin> = vec![usize::MAX; n + 1].into();
    segtree.set(n, 0);
    for i in (1..=n).rev() {
        let x = segtree.get(i);
        let j = i.saturating_sub(c[i - 1]);
        segtree.apply_range(j..i, x + 1);
    }

    segtree.get(0)
}

fn main() {
    input! {
        n: usize,
        c: [usize; n - 1],
        a: [usize; n - 1],
    }

    let mut v = vec![0];
    for (i, &x) in a.iter().enumerate() {
        if x == 1 {
            v.push(i + 1);
        }
    }

    let mut result = 0usize;
    for pair in v.windows(2) {
        let (l, r) = (pair[0], pair[1]);
        result += f(&c[l..r]);
    }
    println!("{result}");
}
