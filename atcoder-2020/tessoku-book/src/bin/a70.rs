use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::{input, marker::Usize1};

fn f(a: &[usize]) -> usize {
    let mut n = 0;
    for i in 0..a.len() {
        if a[i] == 1 {
            n += 1 << i;
        }
    }
    n
}

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        xyz: [(Usize1, Usize1, Usize1); m],
    }

    let mut v = vec![-1; 1 << n];
    let start = f(&a);

    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), start));
    while let Some((Reverse(step), i)) = heap.pop() {
        if v[i] < 0 {
            v[i] = step;
            for &(x, y, z) in &xyz {
                let j = i ^ (1 << x) ^ (1 << y) ^ (1 << z);
                heap.push((Reverse(step + 1), j));
            }
        }
    }

    let result = v[(1 << n) - 1];
    println!("{}", result);
}
