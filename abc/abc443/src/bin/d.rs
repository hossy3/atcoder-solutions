use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: usize,
            mut r: [usize; n],
        }

        let mut queue = BinaryHeap::new();
        for (i, &x) in r.iter().enumerate() {
            queue.push((Reverse(x), i));
        }

        let mut result = 0usize;
        while let Some((Reverse(x), i)) = queue.pop() {
            if r[i] < x {
                continue; // すでに自分自身が移動している
            }
            let x0 = r[i] + 1; // 隣の移動候補
            if i > 0 && r[i - 1] > x0 {
                result += r[i - 1] - x0;
                r[i - 1] = x0;
                queue.push((Reverse(x0), i - 1));
            }
            if i < n - 1 && r[i + 1] > x0 {
                result += r[i + 1] - x0;
                r[i + 1] = x0;
                queue.push((Reverse(x0), i + 1));
            }
        }

        println!("{result}");
    }
}
