use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        q: usize,
    }

    let mut v0 = VecDeque::new();
    let mut v1 = VecDeque::new();
    let f = |v0: &mut VecDeque<_>, v1: &mut VecDeque<_>| {
        if v0.len() + 1 == v1.len() {
            v0.push_back(v1.pop_front().unwrap());
        }
    };

    for _ in 0..q {
        input! {
            t: char,
        }
        match t {
            'A' => {
                input! {
                    x: String,
                }
                f(&mut v0, &mut v1);
                v1.push_back(x);
            }
            'B' => {
                input! {
                    x: String,
                }
                f(&mut v0, &mut v1);
                v1.push_front(x);
            }
            'C' => {
                f(&mut v0, &mut v1);
                v0.pop_front();
            }
            _ => {
                let x = v0.front().unwrap_or(&v1[0]);
                println!("{}", x);
            }
        }
    }
}
