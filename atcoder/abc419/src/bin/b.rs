use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        q: usize,
    }
    let mut heap = BinaryHeap::new();
    for _ in 0..q {
        input! {
            t: u8,
        }
        match t {
            1 => {
                input! {
                    x: usize,
                }
                heap.push(Reverse(x));
            }
            2 => {
                let Some(Reverse(result)) = heap.pop() else {
                    unreachable!()
                };
                println!("{result}");
            }
            _ => unreachable!(),
        }
    }
}
