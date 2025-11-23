use ac_library::{LazySegtree, MapMonoid, Max, Monoid};
use itertools::Itertools;
use proconio::{input, marker::Chars};

struct MaxAdd;
impl MapMonoid for MaxAdd {
    type M = Max<i64>;
    type F = i64;

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
        s: Chars,
    }

    let mut segtree: LazySegtree<MaxAdd> = vec![0i64; n].into();
    for (i, &x) in s.iter().enumerate().rev() {
        let x = (i as i64 + 1) * (x as i64 - '0' as i64);
        segtree.apply_range(0..(n - i), x);
    }

    let mut results = vec![0i64; n * 2];
    for i in 0..n {
        results[i] = segtree.get(i);
    }
    for i in 0..(2 * n - 1) {
        if results[i] >= 10 {
            results[i + 1] += results[i] / 10;
            results[i] %= 10;
        }
    }
    while let Some(&x) = results.last() {
        if x == 0 {
            results.pop();
        } else {
            break;
        }
    }

    let result = results.iter().rev().join("");
    println!("{result}");
}
