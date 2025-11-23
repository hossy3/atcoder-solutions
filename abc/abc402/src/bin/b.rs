use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        q: usize,
    }
    let mut queue = VecDeque::new();
    for _ in 0..q {
        input! {
            k: u8,
        }
        match k {
            1 => {
                input! {
                    x: usize,
                }
                queue.push_back(x);
            }
            2 => {
                let Some(x) = queue.pop_front() else {
                    unreachable!()
                };
                println!("{x}");
            }
            _ => {
                unreachable!()
            }
        }
    }
}
