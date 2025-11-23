use std::collections::VecDeque;

use ac_library::{Max, Segtree};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        d: usize,
        r: usize,
        h: [Usize1; n],
    }

    let mut segtree: Segtree<Max<i64>> = vec![-1_i64; n].into(); // 移動可能な segtree
    let mut queue = VecDeque::new();
    let mut results = vec![0i64; n];

    let mut v = vec![]; // 低い順にソート
    for i in 0..n {
        v.push((h[i], i));
    }
    v.sort();

    for &(h, i) in &v {
        queue.push_back((h, i));
        while let Some(&(h0, i0)) = queue.front() {
            if h0 + d > h {
                break;
            }
            queue.pop_front();
            segtree.set(i0, results[i0]);
        }

        let l0 = i.saturating_sub(r);
        let r0 = (i + r).min(n - 1);
        let x = segtree.prod(l0..=r0) + 1;
        results[i] = x;
    }

    let result = *results.iter().max().unwrap();
    println!("{result}");
}
