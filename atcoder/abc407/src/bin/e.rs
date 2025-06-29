use std::collections::BinaryHeap;

use proconio::input;

fn f(a: &[usize]) -> usize {
    let n = a.len() / 2;
    let mut queue = BinaryHeap::new();
    let mut result = a[0];
    for i in 0..(n - 1) {
        queue.push(a[i * 2 + 1]);
        queue.push(a[i * 2 + 2]);
        result += queue.pop().unwrap();
    }
    result
}

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: usize,
            a: [usize; n * 2],
        }
        let result = f(&a);
        println!("{result}");
    }
}
