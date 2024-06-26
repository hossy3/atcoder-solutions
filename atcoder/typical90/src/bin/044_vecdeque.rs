use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
        txy: [(u8, usize, usize); q],
    }
    let mut v = VecDeque::from(a);
    for (t, x, y) in txy {
        match t {
            1 => {
                v.swap(x - 1, y - 1);
            }
            2 => {
                let x = v.pop_back().unwrap();
                v.push_front(x);
            }
            3 => {
                println!("{}", v[x - 1]);
            }
            _ => unreachable!(),
        }
    }
}
