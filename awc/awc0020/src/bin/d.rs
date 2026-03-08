use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        l: isize,
        xr: [(isize, isize); n],
    }

    let mut queue = BinaryHeap::new();
    for &(x, r) in &xr {
        let l0 = (x - r).max(0);
        let r0 = (x + r).min(l);
        queue.push(Reverse((l0, r0)));
    }

    let mut cur = 0;
    while let Some(Reverse((l, r))) = queue.pop() {
        if cur < l {
            println!("No");
            return;
        }
        cur = cur.max(r);
    }

    let yes = cur == l;
    println!("{}", if yes { "Yes" } else { "No" });
}
