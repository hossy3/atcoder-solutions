use std::cmp::Reverse;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
        r: usize,
        t: usize,
        ps: [(usize, usize); n],
    }

    let psi = ps
        .iter()
        .enumerate()
        .filter(|&(_, &(p, s))| l <= p && p <= r && s >= t)
        .map(|(i, &(p, s))| (p, Reverse(s), i))
        .sorted()
        .collect::<Vec<_>>();
    if let Some((_, _, i)) = psi.iter().next() {
        println!("{}", i + 1);
    } else {
        println!("-1");
    }
}
