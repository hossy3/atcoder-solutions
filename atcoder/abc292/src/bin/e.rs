use std::collections::{BinaryHeap, HashSet};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(Usize1, Usize1); m],
    }

    let mut graph = vec![vec![]; n];
    let mut exists = vec![vec![false; n]; n];
    for &(u, v) in &uv {
        graph[u].push(v);
        exists[u][v] = true;
    }

    let mut count = 0;
    for start in 0..n {
        let mut set = HashSet::new();
        let mut heap = BinaryHeap::new();

        for &i in &graph[start] {
            heap.push(i);
        }
        while let Some(i) = heap.pop() {
            if set.contains(&i) || start == i {
                continue;
            }
            set.insert(i);
            if !exists[start][i] {
                exists[start][i] = true;
                count += 1;
            }
            for &i in &graph[i] {
                heap.push(i);
            }
        }
    }

    println!("{}", count);
}
