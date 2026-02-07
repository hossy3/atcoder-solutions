use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        t: usize,
        a: [usize; n],
    }

    let mut queue = BinaryHeap::new();
    queue.push((Reverse(0), true)); // true: 開く候補
    for x in a {
        queue.push((Reverse(x), false)); // false: 閉じる候補
    }
    queue.push((Reverse(t), false)); // false: 開く候補

    let mut result = 0;
    let mut cur = None; // 現在開いていれば Some(開き始めた時間)
    while let Some((Reverse(x), b)) = queue.pop() {
        if x > t {
            break;
        }
        if let Some(x0) = cur {
            if !b {
                cur = None;
                result += x - x0;
                queue.push((Reverse(x + 100), true));
            }
        } else {
            if b {
                cur = Some(x);
            }
        }
    }

    println!("{result}");
}
