use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        se: [(usize, usize); n],
    }

    let mut queue = BinaryHeap::new();
    for (s, e) in se {
        queue.push((Reverse(s), Reverse(e)));
    }
    let mut result = 0usize;
    while queue.len() > 0 {
        result += 1;
        let mut queue0 = BinaryHeap::new(); // 次のキュー
        let mut cur = 0;
        while let Some((Reverse(s), Reverse(e))) = queue.pop() {
            if s >= cur {
                cur = e;
            } else {
                queue0.push((Reverse(s), Reverse(e)));
            }
        }
        queue = queue0;
    }

    println!("{result}");
}
