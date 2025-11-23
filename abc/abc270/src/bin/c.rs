use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        x: Usize1,
        y: Usize1,
        uv: [(Usize1, Usize1); n - 1],
    }
    let mut g = vec![Vec::new(); n];
    for &(u, v) in &uv {
        g[u].push(v);
        g[v].push(u);
    }

    // y to x
    let mut a = vec![-1i64; n];
    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), y));
    while let Some((Reverse(i), z)) = heap.pop() {
        if a[z] > -1 {
            continue;
        }
        a[z] = i;
        for &j in &g[z] {
            heap.push((Reverse(i + 1), j));
        }
    }

    // x to y
    let mut z = x;
    while z != y {
        print!("{} ", z + 1);
        z = *g[z].iter().find(|&&i| a[i] + 1 == a[z]).unwrap();
    }
    println!("{}", y + 1);
}
