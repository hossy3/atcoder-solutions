use std::cmp::Reverse;

use ac_library::{Additive, Segtree};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        mut lr: [(Usize1, Usize1); m],
    }

    lr.sort_by_key(|&(l, r)| (r, Reverse(l)));
    let mut result = 0i64;
    let mut segtree = Segtree::<Additive<i64>>::new(n);
    for &(l, r) in &lr {
        result += segtree.prod(..=l);
        segtree.set(l + 1, segtree.get(l + 1) + 1);
        segtree.set(r, segtree.get(r) - 1);
    }

    println!("{result}");
}
