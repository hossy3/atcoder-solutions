use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::{input, marker::Usize1};

fn f(i: usize, n: usize, edges: &[Vec<(usize, usize)>]) -> Vec<usize> {
    const MAX: usize = 1usize << 60;
    let mut s = vec![MAX; n];
    let mut heap = BinaryHeap::<(Reverse<usize>, usize)>::new();
    s[i] = 0;
    for &(a, c) in &edges[i] {
        heap.push((Reverse(c), a));
    }
    while let Some((Reverse(c), a)) = heap.pop() {
        if s[a] < MAX {
            continue;
        }
        s[a] = c;
        for &(a, c0) in &edges[a] {
            heap.push((Reverse(c + c0), a));
        }
    }
    s
}

fn main() {
    input! {
        n: usize,
        abc: [(Usize1, Usize1, usize)],
    }
    let mut edges = vec![vec![(0, 0); 0]; n];
    for &(a, b, c) in &abc {
        edges[a].push((b, c));
        edges[b].push((a, c));
    }

    let s0 = f(0, n, &edges);
    let sn = f(n - 1, n, &edges);

    for i in 0..n {
        println!("{}", s0[i] + sn[i]);
    }
}
