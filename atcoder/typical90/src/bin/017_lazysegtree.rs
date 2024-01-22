use std::cmp::Reverse;

use ac_library::{LazySegtree, MapMonoid, Max, Monoid};
use proconio::input;

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

fn f(n: usize, lr: &[(usize, usize)]) -> i64 {
    let mut lr: Vec<(usize, usize)> = lr.iter().map(|&(l, r)| (l - 1, r - 1)).collect();
    lr.sort_by_key(|&(l, r)| (r, Reverse(l)));

    let mut result = 0;
    let mut segtree: LazySegtree<MaxAdd> = vec![0i64; n].into();
    for &(l, r) in &lr {
        result += segtree.get(l);
        segtree.apply_range((l + 1)..r, 1);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        // 交差
        assert_eq!(f(4, &vec![(1, 3), (2, 4)]), 1);
        assert_eq!(f(4, &vec![(2, 4), (1, 3)]), 1);

        // 離れている
        assert_eq!(f(4, &vec![(1, 2), (3, 4)]), 0);
        assert_eq!(f(4, &vec![(3, 4), (1, 2)]), 0);
        assert_eq!(f(4, &vec![(1, 2), (3, 4)]), 0);
        assert_eq!(f(4, &vec![(3, 4), (1, 2)]), 0);

        // 一点接触
        assert_eq!(f(3, &vec![(1, 2), (1, 3)]), 0);
        assert_eq!(f(3, &vec![(1, 3), (1, 2)]), 0);
        assert_eq!(f(3, &vec![(1, 3), (2, 3)]), 0);
        assert_eq!(f(3, &vec![(2, 3), (1, 3)]), 0);

        // 実例
        assert_eq!(f(6, &vec![(2, 5), (1, 4), (1, 3)]), 2);
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        lr: [(usize, usize); m],
    }
    let result = f(n, &lr);
    println!("{result}");
}
