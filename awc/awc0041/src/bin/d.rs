use std::{cmp::Reverse, collections::BTreeSet};

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut ab: [(usize, usize); n],
    }

    // 防御力が高い方から並び替える
    ab.sort_unstable_by_key(|&(_, b)| Reverse(b));

    let mut result = 0;
    let mut set = BTreeSet::new();
    let mut a_sum = 0;
    let mut b_min = usize::MAX;
    for (i, &(a, b)) in ab.iter().enumerate() {
        a_sum += a;
        b_min = b_min.min(b);
        set.insert((a, i));
        if set.len() > k {
            let (a, _) = set.pop_first().unwrap();
            a_sum -= a;
        }
        if set.len() == k {
            result = result.max(a_sum * b_min);
        }
    }
    println!("{result}");
}
