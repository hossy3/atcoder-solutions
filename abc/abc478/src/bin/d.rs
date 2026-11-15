use std::cmp::Reverse;

use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        mut lrx: [(Usize1, Usize1, Usize1); q],
    }

    lrx.sort_by_key(|&(l, r, x)| (x, l, Reverse(r)));

    // 区間をマージする
    let mut v: Vec<(usize, usize, usize)> = vec![];
    for &(mut l, r, x) in &lrx {
        if let Some(&(x0, l0, r0)) = v.last() {
            if x0 == x {
                if r0 >= r {
                    continue; // この入力は使わなくても良い
                }
                if r0 + 1 >= l {
                    v.pop();
                    l = l0; // 区間を混ぜられる
                }
            }
        }
        v.push((x, l, r));
    }

    let mut sum = vec![0isize; n + 1];
    for &(_x, l, r) in &v {
        sum[l] += 1;
        sum[r + 1] -= 1;
    }
    for i in 0..n {
        sum[i + 1] += sum[i];
    }
    println!("{}", sum[..n].iter().join(" "));
}
