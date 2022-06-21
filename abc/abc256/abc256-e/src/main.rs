// -*- coding:utf-8-unix -*-

use proconio::input;
use std::collections::HashSet;

fn min_cost_in_loop(i: usize, xs: &[usize], cs: &[usize]) -> usize {
    let mut c: usize = cs[i];
    let mut j = xs[i] - 1;
    while j != i {
        c = c.min(cs[j]);
        j = xs[j] - 1;
    }
    c
}

fn main() {
    input! {
        n: usize,
        xs: [usize; n],
        cs: [usize; n],
    }

    let mut bs = vec![false; n]; // 0 origin
    let mut c = 0usize;
    for i in 0..n {
        if bs[i] {
            continue;
        }

        let mut set = HashSet::<usize>::new();
        bs[i] = true;
        set.insert(i);
        let mut i = xs[i] - 1;
        while !bs[i] {
            bs[i] = true;
            set.insert(i);
            i = xs[i] - 1;
        }
        if set.contains(&i) {
            c += min_cost_in_loop(i, &xs, &cs);
        }
    }

    println!("{}", c);
}
