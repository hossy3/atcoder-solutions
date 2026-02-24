use std::collections::BinaryHeap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        fd: [(usize, usize); n],
    }
    let mut queue = BinaryHeap::new();
    for (f, d) in fd {
        queue.push((f, d));
    }
    let mut result = 0;
    for _ in 0..m {
        let (f, d) = queue.pop().unwrap();
        result += f;
        queue.push((f.saturating_sub(d), d));
    }
    println!("{result}");
}
