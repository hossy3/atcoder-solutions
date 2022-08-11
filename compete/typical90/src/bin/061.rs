use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        q: usize,
        tx: [(usize, usize); q]
    }
    let mut v = VecDeque::new();
    for (t, x) in tx {
        match t {
            1 => {
                v.push_front(x);
            },
            2 => {
                v.push_back(x);
            },
            _ => {
                println!("{}", v[x - 1]);
            }
        }
    }
}
