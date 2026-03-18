use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        hs: [(usize, usize); n],
        mut p: [usize; m],
    }

    let mut queue = BinaryHeap::new();
    for &(h, s) in &hs {
        queue.push((Reverse(h), s));
    }
    p.sort();

    let mut result = 0;
    let mut queue0 = BinaryHeap::new();
    for &x in &p {
        while let Some(&(Reverse(h), s)) = queue.peek() {
            if h > x {
                break;
            }
            queue0.push(s);
            queue.pop();
        }

        if let Some(x) = queue0.pop() {
            result += x;
        } else {
            println!("-1");
            return;
        }
    }

    println!("{result}");
}
