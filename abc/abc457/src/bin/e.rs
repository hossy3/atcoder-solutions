use std::collections::BTreeSet;

use ac_library::{Min, Segtree};
use proconio::{input, marker::Usize1};

fn f(
    set_lr: &BTreeSet<(usize, usize, usize)>,
    set_rl: &BTreeSet<(usize, usize, usize)>,
    segtree: &Segtree<Min<usize>>,
    s: usize,
    t: usize,
) -> bool {
    let Some(&(l0, r0, i0)) = set_lr.range(..=(s, t, usize::MAX)).last() else {
        return false;
    };
    if l0 != s {
        return false;
    }
    let Some(&(r1, l1, i1)) = set_rl.range((t, s, 0)..).next() else {
        return false;
    };
    if r1 != t {
        return false;
    }
    if r0 + 1 < l1 {
        return false;
    }

    // 1枚で満たせるときは、邪魔にならない2枚目があるかを確認する
    if i0 == i1 {
        if let Some(&(l0, _, _)) = set_lr.range(..(s, t, i0)).last() {
            if l0 == s {
                return true;
            }
        };
        if segtree.prod((l0 + 1)..=r0) <= r0 {
            return true;
        }
        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup(
        lr: &[(usize, usize)],
    ) -> (
        BTreeSet<(usize, usize, usize)>,
        BTreeSet<(usize, usize, usize)>,
        Segtree<Min<usize>>,
    ) {
        let mut set_lri = BTreeSet::new();
        let mut set_rli = BTreeSet::new();
        for (i, &(l, r)) in lr.iter().enumerate() {
            set_lri.insert((l, r, i));
            set_rli.insert((r, l, i));
        }
        let mut segtree: Segtree<Min<usize>> = Segtree::new(6);
        for &(l, r) in lr {
            segtree.set(l, segtree.get(l).min(r));
        }
        (set_lri, set_rli, segtree)
    }

    #[test]
    fn test_name() {
        let (set_lri, set_rli, segtree) = setup(&[(1, 5), (2, 4)]);
        assert_eq!(f(&set_lri, &set_rli, &segtree, 1, 5), true);
        assert_eq!(f(&set_lri, &set_rli, &segtree, 1, 4), false);
        assert_eq!(f(&set_lri, &set_rli, &segtree, 2, 4), false);
        assert_eq!(f(&set_lri, &set_rli, &segtree, 2, 5), false);

        let (set_lri, set_rli, segtree) = setup(&[(1, 1), (2, 2)]);
        assert_eq!(f(&set_lri, &set_rli, &segtree, 1, 1), false);
        assert_eq!(f(&set_lri, &set_rli, &segtree, 1, 2), true);
        assert_eq!(f(&set_lri, &set_rli, &segtree, 2, 1), false);
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        lr: [(Usize1, Usize1); m],
        q: usize,
        st: [(Usize1, Usize1); q],
    }

    let mut set_lri = BTreeSet::new();
    let mut set_rli = BTreeSet::new();
    for (i, &(l, r)) in lr.iter().enumerate() {
        set_lri.insert((l, r, i));
        set_rli.insert((r, l, i));
    }

    let mut segtree: Segtree<Min<usize>> = Segtree::new(n);
    for &(l, r) in &lr {
        segtree.set(l, segtree.get(l).min(r));
    }

    for (s, t) in st {
        let yes = f(&set_lri, &set_rli, &segtree, s, t);
        println!("{}", if yes { "Yes" } else { "No" });
    }
}
