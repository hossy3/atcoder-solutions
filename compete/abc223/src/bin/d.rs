use std::{
    cmp::Reverse,
    collections::{BTreeSet, BinaryHeap},
};

use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    }

    let mut a2b = vec![BTreeSet::new(); n]; // a to b
    let mut b2a = vec![BTreeSet::new(); n]; // a to b
    for &(a, b) in &ab {
        a2b[a].insert(b);
        b2a[b].insert(a);
    }

    let mut queue = BinaryHeap::new();
    for i in 0..n {
        if b2a[i].is_empty() {
            queue.push(Reverse(i));
        }
    }

    let mut results = Vec::with_capacity(n);
    while let Some(Reverse(i)) = queue.pop() {
        results.push(i);
        for &j in &a2b[i] {
            b2a[j].remove(&i);
            if b2a[j].is_empty() {
                queue.push(Reverse(j));
            }
        }
        a2b[i].clear();
    }

    if results.len() == n {
        let result = results.iter().map(|x| x + 1).join(" ");
        println!("{}", result);
    } else {
        println!("{}", -1);
    }
}
