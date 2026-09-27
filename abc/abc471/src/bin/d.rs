use std::collections::BinaryHeap;

use proconio::input;

fn main() {
    input! {
        q: usize,
        v: isize,
    }

    let mut heap = BinaryHeap::new(); // 時刻0から充電開始していたとして、時刻0のバッテリー容量
    for _ in 0..q {
        input! {
            k: u8,
            t: isize,
        }

        match k {
            1 => {
                input! {
                    w: isize,
                }
                heap.push(w - t);
            }
            2 => {
                if let Some(x) = heap.pop() {
                    let result = (x + t).clamp(0, v);
                    println!("{result}");
                } else {
                    println!("-1");
                }
            }
            _ => unreachable!(),
        }
    }
}
