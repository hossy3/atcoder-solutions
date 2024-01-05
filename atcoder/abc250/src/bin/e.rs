// -*- coding:utf-8-unix -*-

use std::collections::{BTreeSet, BTreeMap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
        b: [i32; n],
        q: usize,
        xy: [(usize, usize); q],
    }
    let mut map = BTreeMap::<i32, usize>::new(); // compression
    let mut a_counts = vec![0usize; n];
    for i in 0..n {
        if !map.contains_key(&a[i]) {
            map.insert(a[i], map.len());
        }
        a_counts[i] = map.len();
    }
    let mut b_counts = vec![0usize; n];
    let mut b_maxvals = vec![0usize; n];
    {
        let mut maxval = 0usize;
        let mut set = BTreeSet::<i32>::new();
        for i in 0..n {
            if !set.contains(&b[i]) {
                set.insert(b[i]);
                maxval = maxval.max(*map.get(&b[i]).unwrap_or(&q));
            }
            b_counts[i] = set.len();
            b_maxvals[i] = maxval;
        }
    }

    for (x, y) in xy {
        let yes = (a_counts[x - 1] == b_counts[y - 1]) && (b_counts[y - 1] == b_maxvals[y - 1] + 1); 
        println!("{}", if yes { "Yes" } else { "No" });
    }
}
