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

    let mut g = Vec::with_capacity(n);
    for _ in 0..n {
        input! {
            a: usize,
            s: [Usize1; a],
        }
        let set: HashSet<usize> = s.into_iter().collect();
        g.push(set);
    }

    let mut v = vec![false; m];
    let mut queue = BinaryHeap::new();
    for i in 0..n {
        if g[i].contains(&0) {
            queue.push((Reverse(0), i));
            v[i] = true;
            if g[i].contains(&(m - 1)) {
                println!("{}", 0);
                return;
            }
        }
    }
    while let Some((Reverse(step), i)) = queue.pop() {
        for j in 0..n {
            if v[j] {
                continue;
            }
            if g[i].is_disjoint(&g[j]) {
                continue;
            }
            queue.push((Reverse(step + 1), j));
            v[j] = true;
            if g[j].contains(&(m - 1)) {
                println!("{}", step + 1);
                return;
            }
        }
    }

    println!("{}", -1);
}
