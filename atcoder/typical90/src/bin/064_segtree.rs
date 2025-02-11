use ac_library::{Monoid, Segtree};
use num_traits::{Signed, Zero};
use proconio::{input, marker::Usize1};
use std::{convert::Infallible, marker::PhantomData};

struct AbsAdd<S>(Infallible, PhantomData<fn() -> S>);
impl<S> Monoid for AbsAdd<S>
where
    S: Copy + Signed + Zero,
{
    type S = S;
    fn identity() -> Self::S {
        S::zero()
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        a.abs() + b.abs()
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n],
        lrv: [(Usize1, Usize1, i64); q]
    }

    let diffs: Vec<_> = (0..(n - 1)).map(|i| a[i + 1] - a[i]).collect();
    let mut segtree = Segtree::<AbsAdd<_>>::from(diffs);

    for (l, r, v) in lrv {
        if l > 0 {
            segtree.set(l - 1, segtree.get(l - 1) + v);
        }
        if r < n - 1 {
            segtree.set(r, segtree.get(r) - v);
        }
        let score = segtree.all_prod();
        println!("{score}");
    }
}
