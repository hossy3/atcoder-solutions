use std::{cmp::Reverse, collections::BinaryHeap};

use num::integer::Roots;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: i64,
    }

    let mut v = Vec::new();
    let max = m.sqrt();
    for i in 0..=max {
        let j = (m - i * i).sqrt();
        if i * i + j * j == m {
            v.push((i, j));
            if i != 0 {
                v.push((-i, j));
            }
            if j != 0 {
                v.push((i, -j));
            }
            if i != 0 && j != 0 {
                v.push((-i, -j));
            }
        }
    }

    let mut a = vec![vec![-1; n]; n];
    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), 0_i64, 0_i64));
    while let Some((Reverse(k), i, j)) = heap.pop() {
        if a[i as usize][j as usize] >= 0 {
            continue;
        }
        a[i as usize][j as usize] = k;
        for &(i0, j0) in &v {
            let i = i + i0;
            let j = j + j0;
            if i < 0 || i >= n as i64 {
                continue;
            }
            if j < 0 || j >= n as i64 {
                continue;
            }
            if a[i as usize][j as usize] >= 0 {
                continue;
            }
            heap.push((Reverse(k + 1), i, j));
        }
    }

    for a0 in &a {
        for (i, x) in a0.iter().enumerate() {
            let s = if i == 0 { "" } else { " " };
            print!("{}{}", s, x);
        }
        println!();
    }
}
