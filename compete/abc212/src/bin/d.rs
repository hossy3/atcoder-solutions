use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        q: usize,
    }
    let mut base = 0;
    let mut queue = BinaryHeap::new();
    for _ in 0..q {
        input! {
            p: usize,
        }
        match p {
            1 => {
                input! {
                    x: i64,
                }
                queue.push(Reverse(x - base));
            }
            2 => {
                input! {
                    x: i64,
                }
                base += x;
            }
            _ => {
                if let Some(Reverse(x)) = queue.pop() {
                    println!("{}", x + base);
                }
            }
        }
    }
}
