use std::collections::BinaryHeap;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        ldkcab: [(usize, usize, usize, usize, Usize1, Usize1); m],
    }

    // 逆方向のグラフを組み立てる
    let mut graph = vec![vec![]; n];
    for &(l, d, k, c, a, b) in &ldkcab {
        graph[b].push((l, d, k, c, a));
    }

    let mut v = vec![None; n];
    let mut heap = BinaryHeap::<(usize, usize)>::new();
    heap.push((usize::MAX, n - 1));

    while let Some((t, i)) = heap.pop() {
        if let Some(t0) = v[i] {
            if t0 >= t {
                continue;
            }
        }
        v[i] = Some(t);
        for &(l, d, k, c, a) in &graph[i] {
            if t < l + c {
                continue; // 始発前
            }
            let t0 = l + ((t - (l + c)) / d).clamp(0, k - 1) * d;
            if let Some(t1) = v[a] {
                if t1 >= t0 {
                    continue;
                }
            }
            heap.push((t0, a));
        }
    }

    for x in &v[..(n - 1)] {
        if let Some(x) = x {
            println!("{x}");
        } else {
            println!("Unreachable");
        }
    }
}
