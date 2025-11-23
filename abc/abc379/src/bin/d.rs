use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        q: usize,
    }

    let mut queue = VecDeque::<i64>::new();
    let mut cur = 0i64; // 現在の日付

    for _ in 0..q {
        input! {
            k: usize,
        }
        match k {
            1 => {
                queue.push_back(-cur);
            }
            2 => {
                input! {
                    t: i64,
                }
                cur += t;
            }
            3 => {
                input! {
                    h: i64,
                }
                let len = queue.len();
                while let Some(&x) = queue.front() {
                    if cur + x >= h {
                        queue.pop_front();
                    } else {
                        break;
                    }
                }
                let result = len - queue.len();
                println!("{result}");
            }
            _ => unreachable!(),
        }
    }
}
