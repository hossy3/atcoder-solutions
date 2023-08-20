use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut g0 = Vec::with_capacity(n);
    for _ in 0..n {
        input! {
            a: usize,
            s: [Usize1; a],
        }
        let set: HashSet<usize> = s.into_iter().collect();
        g0.push(set);
    }

    let mut g = vec![vec![]; n];
    for (i, set) in g0.iter().enumerate() {
        for j in (i + 1)..n {
            if !set.is_disjoint(&g0[j]) {
                g[i].push(j);
                g[j].push(i);
            }
        }
    }

    let mut v = vec![false; m];
    let mut queue = BinaryHeap::new();
    for i in 0..n {
        if g0[i].contains(&0) {
            queue.push((Reverse(0), i));
            v[i] = true;
        }
    }
    while let Some((Reverse(step), i)) = queue.pop() {
        if g0[i].contains(&(m - 1)) {
            println!("{}", step);
            return;
        }
        for &j in &g[i] {
            if v[j] {
                continue;
            }
            queue.push((Reverse(step + 1), j));
            v[j] = true;
        }
    }

    println!("{}", -1);
}
