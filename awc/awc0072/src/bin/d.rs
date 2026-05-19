use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        p: [usize; n],
        uv: [(Usize1, Usize1); m],
    }

    let mut graph = vec![vec![]; n]; // node, edge
    for &(u, v) in &uv {
        if p[u] < p[v] {
            graph[u].push(v);
        }
    }

    let mut v = vec![None; n];
    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), 0, 0));
    v[0] = Some(0);

    while let Some((_, step, i)) = heap.pop() {
        if step < v[i].unwrap_or(0) {
            continue;
        }

        for &j in &graph[i] {
            if p[i] >= p[j] {
                continue;
            }
            let w = p[j];
            let step = step + 1;
            let step0 = v[j].unwrap_or(0);
            if step > step0 {
                v[j] = Some(step);
                heap.push((Reverse(w), step, j));
            }
        }
    }

    let result = v.iter().map(|&x| x.unwrap_or(0)).max().unwrap_or(0) + 1;
    println!("{result}");
}
