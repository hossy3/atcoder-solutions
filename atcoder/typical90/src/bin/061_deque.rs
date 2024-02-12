use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        q: usize,
        tx: [(u8, usize); q]
    }

    let mut v = VecDeque::new();
    for (t, x) in tx {
        match t {
            1 => {
                v.push_front(x);
            }
            2 => {
                v.push_back(x);
            }
            3 => {
                println!("{}", v[x - 1]);
            }
            _ => unreachable!(),
        }
    }
}
