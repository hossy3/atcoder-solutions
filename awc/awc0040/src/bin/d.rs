use std::collections::BinaryHeap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        g: usize,
        mut f: usize,
        mut pr: [(usize, usize); n],
    }

    pr.push((0, 0));
    pr.push((g, 0));
    pr.sort_unstable();

    let mut result = 0usize;
    let mut heap: BinaryHeap<usize> = BinaryHeap::new();

    for i in 0..=n {
        heap.push(pr[i].1);
        if pr[i].0 + f < pr[i + 1].0 {
            while let Some(x) = heap.pop() {
                result += 1;
                f += x;
                if !(pr[i].0 + f < pr[i + 1].0) {
                    break;
                }
            }
            if pr[i].0 + f < pr[i + 1].0 {
                println!("-1");
                return;
            }
        }
        f -= pr[i + 1].0 - pr[i].0;
    }

    println!("{result}");
}
