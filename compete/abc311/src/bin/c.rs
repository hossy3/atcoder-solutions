use std::collections::HashSet;

use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    }
    let mut set = HashSet::new();
    set.insert(0);
    let mut i = a[0];
    while !set.contains(&i) {
        set.insert(i);
        i = a[i];
    }

    let mut v = vec![];
    let mut j = i;
    v.push(i + 1);
    while a[j] != i {
        j = a[j];
        v.push(j + 1);
    }
    println!("{}", v.len());
    println!("{}", v.iter().join(" "));
}
