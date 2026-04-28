use std::{collections::HashMap, mem::swap};

use itertools::Itertools;
use proconio::input;

fn hilbert_order(mut x: usize, mut y: usize) -> usize {
    const MAX: usize = 1 << 31;

    let mut d = 0;
    let mut s = MAX >> 1;
    while s > 0 {
        let rx = (x & s) > 0;
        let ry = (y & s) > 0;
        let r = (if rx { 3 } else { 0 }) ^ (if ry { 1 } else { 0 });
        d += s * s * r;
        s = s >> 1;
        if ry {
            continue;
        }
        if rx {
            x = MAX - 1 - x;
            y = MAX - 1 - y;
        }
        swap(&mut x, &mut y);
    }
    d
}

fn main() {
    input! {
        n: usize,
        q: usize,
        mut xv: [(usize, usize); n],
        lr: [(usize, usize); q],
    }

    xv.sort_by_key(|&(x, _)| x);
    let xv = xv;
    let ilr: Vec<_> = lr
        .iter()
        .map(|&(l, r)| {
            (
                xv.partition_point(|&(x, _)| x < l),
                xv.partition_point(|&(x, _)| x <= r),
            )
        })
        .enumerate()
        .sorted_by_cached_key(|&(_, (l0, r0))| hilbert_order(l0, r0))
        .collect();

    let mut l = 0usize;
    let mut r = 0usize; // 開区間 (r を含まない)

    let mut counts = HashMap::new(); // その数字の出現数
    for &(_, v) in &xv {
        counts.insert(v, 0);
    }

    let mut count = 0isize;
    let mut results = vec![0usize; q];

    for &(i, (l0, r0)) in &ilr {
        // 幅を増やす
        if l0 < l {
            for l in l0..l {
                if let Some(c) = counts.get_mut(&xv[l].1) {
                    count += *c;
                    *c += 1;
                }
            }
        }
        if r < r0 {
            for r in r..r0 {
                if let Some(c) = counts.get_mut(&xv[r].1) {
                    count += *c;
                    *c += 1;
                }
            }
        }

        // 幅を減らす
        if r0 < r {
            for r in r0..r {
                if let Some(c) = counts.get_mut(&xv[r].1) {
                    *c -= 1;
                    count -= *c;
                }
            }
        }
        if l < l0 {
            for l in l..l0 {
                if let Some(c) = counts.get_mut(&xv[l].1) {
                    *c -= 1;
                    count -= *c;
                }
            }
        }

        l = l0;
        r = r0;

        results[i] = (r - l) * (r + 1 - l) / 2 - count as usize;
    }

    for result in results {
        println!("{result}");
    }
}
