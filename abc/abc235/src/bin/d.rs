use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn log10(n: usize) -> u32 {
    (n as f64).log10().floor() as u32
}

fn main() {
    input! {
        a: usize,
        n: usize,
    }

    const MAX: usize = 1_000_000;
    let mut v = vec![std::usize::MAX; MAX];
    let mut heap = BinaryHeap::new();
    v[1] = 0;
    heap.push((Reverse(0), 1, 1));

    while let Some((Reverse(step), i, k)) = heap.pop() {
        if i == n {
            println!("{}", step);
            return;
        }

        let j = i * a;
        if j < MAX && v[j] == std::usize::MAX {
            let k = 10usize.pow(log10(j));
            heap.push((Reverse(step + 1), j, k));
            v[j] = step + 1;
        }

        if i >= 10 && i % 10 > 0 {
            let j = (i % 10) * k + (i / 10);
            if j < MAX && v[j] == std::usize::MAX {
                heap.push((Reverse(step + 1), j, k));
                v[j] = step + 1;
            }
        }
    }

    println!("{}", -1);
}
