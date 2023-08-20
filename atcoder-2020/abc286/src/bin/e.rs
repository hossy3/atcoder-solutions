use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        s: [Chars; n],
        q: usize,
        uv: [(Usize1, Usize1); q],
    }

    let mut results = vec![vec![(0, 0); n]; n];
    for u in 0..n {
        let mut visited = vec![false; n];
        let mut heap = BinaryHeap::new();
        heap.push((Reverse(0), 0, u)); // step, value, pos
        while let Some((Reverse(step), value, v)) = heap.pop() {
            if visited[v] {
                continue;
            }
            let value = value + a[v];
            results[u][v] = (step, value);
            visited[v] = true;
            for (i, &c) in s[v].iter().enumerate() {
                if c == 'Y' && !visited[i] {
                    heap.push((Reverse(step + 1), value, i));
                }
            }
        }
    }

    for &(u, v) in &uv {
        let (step, value) = results[u][v];
        if value == 0 {
            println!("Impossible");
        } else {
            println!("{} {}", step, value);
        }
    }
}
