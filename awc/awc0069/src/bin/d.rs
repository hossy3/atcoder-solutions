use std::collections::BTreeSet;

use itertools::Itertools;
use proconio::input;

fn f(k: usize, d: usize, xp: &[(usize, usize)]) -> isize {
    let mut set0 = BTreeSet::new(); // すべての村
    let mut set1 = BTreeSet::new(); // 電波塔を設置できる村

    for &(x, p) in xp.iter().sorted_unstable() {
        set0.insert(x);
        if p >= k {
            set1.insert(x);
        }
    }

    let mut count = 0isize;
    let mut cur = 0usize;
    for &x0 in &set0 {
        if count > 0 && x0.abs_diff(cur) <= d {
            continue;
        }
        let Some(&x1) = set1.range(..=(x0 + d)).last() else {
            return -1;
        };
        if x1.abs_diff(x0) > d {
            return -1;
        }
        cur = x1;
        count += 1;
    }

    count
}

fn main() {
    input! {
        n: usize,
        k: usize,
        d: usize,
        xp: [(usize, usize); n],
    }

    let result = f(k, d, &xp);
    println!("{result}");
}
