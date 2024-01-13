use std::cmp::Reverse;

use ac_library::{Additive, Segtree};
use proconio::input;

fn f(n: usize, lr: &Vec<(usize, usize)>) -> i64 {
    let mut lr: Vec<(usize, usize)> = lr.iter().map(|&(l, r)| (l - 1, r - 1)).collect();
    lr.sort_by_key(|&(l, r)| (r, Reverse(l)));

    let mut result = 0;
    let mut segtree = Segtree::<Additive<i64>>::new(n);
    for &(l, r) in &lr {
        result += segtree.prod(..=l);
        segtree.set(l + 1, segtree.get(l + 1) + 1);
        segtree.set(r, segtree.get(r) - 1);
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
        assert_eq!(f(6, &mut vec![(2, 5), (1, 4), (1, 3)]), 2);
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        mut lr: [(usize, usize); m],
    }
    let result = f(n, &mut lr);
    println!("{result}");
}
