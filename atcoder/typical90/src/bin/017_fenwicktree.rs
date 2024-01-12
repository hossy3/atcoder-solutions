use std::cmp::Reverse;

use ac_library::FenwickTree;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        mut lr: [(Usize1, Usize1); m],
    }

    lr.sort_by_key(|&(l, r)| (r, Reverse(l)));
    let mut result = 0i64;
    let mut tree = FenwickTree::new(n, 0i64);
    for &(l, r) in &lr {
        result += tree.sum(..=l);
        tree.add(l + 1, 1);
        tree.add(r, -1);
    }

    println!("{result}");
}
