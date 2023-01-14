use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        q: usize,
    }

    let mut heap = BinaryHeap::new();

    for _ in 0..q {
        input! {
            k: usize,
        }

        match k {
            1 => {
                input! {
                    x: usize,
                }
                heap.push(Reverse(x));
            }
            2 => {
                if let Some(Reverse(x)) = heap.peek() {
                    println!("{}", x);
                }
            }
            _ => {
                heap.pop();
            }
        }
    }
}
