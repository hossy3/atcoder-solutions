use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        q: usize,
    }

    let mut queue = VecDeque::new();

    for _ in 0..q {
        input! {
            k: usize,
        }

        match k {
            1 => {
                input! {
                    name: String,
                }
                queue.push_back(name);
            }
            2 => {
                if let Some(name) = queue.front() {
                    println!("{}", name);
                }
            }
            _ => {
                queue.pop_front();
            }
        }
    }
}
