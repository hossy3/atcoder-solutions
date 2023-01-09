use std::{collections::BinaryHeap, cmp::Reverse};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [Usize1; n - 1],
        b: [Usize1; n - 1],
    }

    let mut heap = BinaryHeap::new();
    heap.push((Reverse(a[0]), 100));
    heap.push((Reverse(b[0]), 150));

    let mut prev = 0;
    while let Some((Reverse(i), score)) = heap.pop() {
        if i == n - 1 {
            println!("{}", score);
            return;
        }
        if prev != i {
            heap.push((Reverse(a[i]), score + 100));
            heap.push((Reverse(b[i]), score + 150));
            prev = i;
        }
    }
}
