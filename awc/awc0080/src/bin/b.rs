use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
        t: [usize; q],
    }

    let mut heap = BinaryHeap::new();
    for &a in &a {
        heap.push(Reverse(a));
    }

    for &t in &t {
        let mut count = 0;
        while let Some(&Reverse(x)) = heap.iter().next() {
            if x >= t {
                break;
            }
            heap.pop();
            count += 1;
        }
        println!("{count}");
    }
}
