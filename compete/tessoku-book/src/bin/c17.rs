use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        q: usize,
    }

    let mut v0 = VecDeque::new();
    let mut v1 = VecDeque::new();

    for _ in 0..q {
        input! {
            t: char,
        }
        match t {
            'A' => {
                input! {
                    x: String,
                }
                v1.push_back(x);
            }
            'B' => {
                input! {
                    x: String,
                }
                v1.push_front(x);
            }
            'C' => {
                v0.pop_front();
            }
            _ => {
                let x = v0.front().unwrap();
                println!("{}", x);
            }
        }
        if v0.len() + 1 == v1.len() {
            v0.push_back(v1.pop_front().unwrap());
        }
    }
}
