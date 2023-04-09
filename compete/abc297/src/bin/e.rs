use std::{
    cmp::Reverse,
    collections::{BTreeSet, BinaryHeap},
};

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [usize; n],
    }

    a.sort();
    let mut queue = BinaryHeap::new();
    let mut set = BTreeSet::new();
    for (i, &x) in a.iter().enumerate() {
        queue.push((Reverse(x), i));
    }
    while let Some((Reverse(x), i)) = queue.pop() {
        if set.contains(&x) {
            continue;
        }
        set.insert(x);
        if set.len() == k {
            println!("{}", x);
            return;
        }
        for (i, &y) in a[i..].iter().enumerate() {
            queue.push((Reverse(x + y), i));
        }
    }
}
