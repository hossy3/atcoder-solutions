use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        k: usize,
    }

    let mut count = 0;
    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0usize), Reverse(0usize)));
    for i in 1..=9 {
        heap.push((Reverse(0usize), Reverse(i)));
    }

    while let Some((Reverse(k0), Reverse(x))) = heap.pop() {
        let k1 = 10usize.pow(k0 as u32);
        let x1 = x / k1;
        if x1 > 0 {
            count += 1;
            if count == k {
                println!("{x}");
                break;
            }
        }

        heap.push((Reverse(k0 + 1), Reverse((x1 * 10) * k1 + x)));
        if x1 > 0 {
            heap.push((Reverse(k0 + 1), Reverse(((x1 - 1) * 10) * k1 + x)));
        }
        if x1 < 9 {
            heap.push((Reverse(k0 + 1), Reverse(((x1 + 1) * 10) * k1 + x)));
        }
    }
}
