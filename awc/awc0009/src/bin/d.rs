use std::collections::VecDeque;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        lr: [(usize, usize); m],
    }

    let mut queue: VecDeque<_> = lr.iter().sorted().collect();
    let mut cur = 0;
    let mut result = 0;
    while let Some(&(l, r)) = queue.pop_front() {
        if l <= cur {
            cur = cur.max(r);
        } else {
            result += l - cur - 1;
            if result >= n {
                println!("{}", (l - 1) - (result - n));
                return;
            }
            cur = r;
        }
    }

    println!("{}", cur + (n - result));
}
