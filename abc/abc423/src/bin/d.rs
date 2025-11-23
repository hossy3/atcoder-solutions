use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        abc: [(usize, usize, usize); n],
    }

    let mut time = 0; // 現在時刻
    let mut count = 0; // 現在のお客さんの数
    let mut queue = BinaryHeap::new(); // お客さんの退店予定時刻
    let mut results = vec![];
    for (a, b, c) in abc {
        while count + c > k {
            let Some((Reverse(a), c)) = queue.pop() else {
                unreachable!()
            };
            count -= c;
            time = a;
        }
        let a = a.max(time);
        count += c;
        results.push(a);
        queue.push((Reverse(a + b), c));
    }
    for result in results {
        println!("{result}");
    }
}
