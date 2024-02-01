use std::collections::VecDeque;

use proconio::input;

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
                let Some(x) = v.pop_back() else { unreachable!(); };
                v.push_front(x);
            }
            3 => {
                println!("{}", v[x - 1]);
            }
            _ => unreachable!(),
        }
    }
}
