use std::collections::BinaryHeap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }
    a.sort();
    a.reverse();
    let mut heap = BinaryHeap::new();
    heap.push(a[0]);
    let mut result = 0;
    for &x in &a[1..] {
        let y = heap.pop().unwrap();
        result += y;
        heap.push(x);
        heap.push(x);
    }
    println!("{result}");
}
