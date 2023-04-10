use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        _: usize,
        a: [[u8; n]; n],
        q: usize,
        st: [(Usize1, Usize1); q],
    }

    'outer: for &(x, t) in &st {
        let t = t % n;
        let mut v = vec![false; n];
        let mut queue = BinaryHeap::new();
        for i in 0..n {
            let x = x % n;
            if a[x][i] == 1 {
                queue.push((Reverse(1), i));
            }
        }
        while let Some((Reverse(step), x)) = queue.pop() {
            let x = x % n;
            if v[x] {
                continue;
            }
            v[x] = true;
            if x == t {
                println!("{}", step);
                continue 'outer;
            }
            for i in 0..n {
                if a[x][i] == 1 && !v[i] {
                    queue.push((Reverse(step + 1), i));
                }
            }
        }
        println!("{}", -1);
    }
}
