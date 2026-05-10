use std::collections::BTreeSet;

use ac_library::FenwickTree;
use proconio::{input, marker::Usize1};

/// Ordered Disjoint Tree (ODT) を初期設定する
fn ordered_disjoint_tree_init(c: &[usize]) -> (BTreeSet<(usize, usize)>, FenwickTree<isize>) {
    let n = c.len();
    let mut set: BTreeSet<(usize, usize)> = BTreeSet::new();
    let mut fenwick = FenwickTree::new(n + 1, 0isize);
    for (i, &c) in c.iter().enumerate() {
        if i == 0 || c != set.last().unwrap().1 {
            set.insert((i, c));
            fenwick.add(i, 1);
        }
    }
    (set, fenwick)
}

/// [l, r) の範囲を x に置き換える
fn ordered_disjoint_tree_assign(
    set: &mut BTreeSet<(usize, usize)>,
    fenwick: &mut FenwickTree<isize>,
    l: usize,
    r: usize,
    x: usize,
) {
    let (_, xl) = *set.range((0, 0)..(l, 0)).last().unwrap_or(&(0, usize::MAX));
    let (_, xr) = *set.range((0, 0)..(r + 1, 0)).last().unwrap();

    while let Some(&(i, c)) = set.range((l, 0)..(r + 1, 0)).last() {
        set.remove(&(i, c));
        fenwick.add(i, -1);
    }

    if xl != x {
        set.insert((l, x));
        fenwick.add(l, 1);
    }
    if xr != x {
        set.insert((r, xr));
        fenwick.add(r, 1);
    }
}

/// [l, r) の同じ値が連続する区間数を返す
fn ordered_disjoint_tree_count(fenwick: &FenwickTree<isize>, l: usize, r: usize) -> isize {
    if l + 1 <= r {
        fenwick.sum((l + 1)..r) + 1
    } else {
        0
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        c: [usize; n],
    }

    let (mut set, mut fenwick) = ordered_disjoint_tree_init(&c);

    for _ in 0..q {
        input! {
            k: u8,
        }
        match k {
            1 => {
                input! {
                    l: Usize1,
                    r: Usize1,
                    x: usize,
                }
                ordered_disjoint_tree_assign(&mut set, &mut fenwick, l, r + 1, x);
            }
            2 => {
                input! {
                    l: Usize1,
                    r: Usize1,
                }
                let result = ordered_disjoint_tree_count(&fenwick, l, r + 1);
                println!("{result}");
            }
            _ => unreachable!(),
        }
    }
}
