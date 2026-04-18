use std::collections::BinaryHeap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut k: usize,
        ab: [(usize, usize); n],
    }

    let mut heap = BinaryHeap::new();
    for (a, b) in ab {
        if a >= b {
            heap.push((b, a / b));
        }
        if a % b > 0 {
            heap.push((a % b, 1));
        }
    }

    let mut result = 0;
    while let Some((x0, k0)) = heap.pop() {
        let k0 = k0.min(k);
        result += x0 * k0;
        k -= k0;
        if k == 0 {
            break;
        }
    }
    println!("{result}");
}
