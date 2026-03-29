use std::cmp::Reverse;

use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        mut p: [isize; n],
        uvw: [(Usize1, Usize1, isize); m],
    }

    for (u, v, w) in uvw {
        p[u] -= w;
        p[v] += w;
    }

    let Some(i) = (0..n).position_max_by_key(|&i| (p[i], Reverse(i))) else {
        unreachable!()
    };
    println!("{}", i + 1);
}
