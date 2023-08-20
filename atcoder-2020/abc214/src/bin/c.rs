use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [usize; n],
        t: [usize; n],
    }

    let mut queue = BinaryHeap::new();
    for (i, &x) in t.iter().enumerate() {
        queue.push((Reverse(x), i));
    }

    let mut v = vec![std::usize::MAX; n];
    while let Some((Reverse(x), i)) = queue.pop() {
        if v[i] < std::usize::MAX {
            continue;
        }
        v[i] = x;
        let x = x + s[i];
        let i = (i + 1) % n;
        if v[i] > x {
            queue.push((Reverse(x), i));
        }
    }

    for x in v {
        println!("{}", x);
    }
}
