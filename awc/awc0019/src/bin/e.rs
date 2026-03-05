use std::collections::BinaryHeap;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        wd: [(usize, usize); n],
    }

    let xw = wd
        .iter()
        .map(|&(w, d)| (w + d, w))
        .sorted()
        .collect::<Vec<_>>();

    let mut heap = BinaryHeap::new();
    let mut sum_w = 0usize;
    for (x, w) in xw {
        sum_w += w;
        heap.push(w);
        let Some(&w0) = heap.iter().next() else {
            unreachable!()
        };
        if sum_w > x {
            sum_w -= w0;
            heap.pop();
        }
    }

    let result = heap.len();
    println!("{result}");
}
