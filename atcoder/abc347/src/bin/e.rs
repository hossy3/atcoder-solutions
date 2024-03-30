use std::collections::HashSet;

use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, q): (usize, usize),
        x: [Usize1; q],
    }

    let mut a = vec![(0usize, usize::MAX); n]; // val, add start
    let mut cum = vec![0usize; q + 1];
    let mut set = HashSet::new();

    for (i, &x) in x.iter().enumerate() {
        if set.insert(x) {
            a[x].1 = i;
        } else {
            a[x].0 += cum[i] - cum[a[x].1];
            a[x].1 = usize::MAX;
            set.remove(&x);
        }
        cum[i + 1] = cum[i] + set.len();
    }
    for i in 0..n {
        if a[i].1 != usize::MAX {
            a[i].0 += cum[q] - cum[a[i].1];
        }
    }
    let result = a.iter().map(|x| x.0).join(" ");
    println!("{result}");
}
