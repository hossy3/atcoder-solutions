use std::collections::BinaryHeap;

// toposort

use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        xy: [(Usize1, Usize1); m],
    }

    let mut edges = vec![vec![]; n];
    let mut degs = vec![0; n];
    for &(x, y) in &xy {
        edges[x].push(y);
        degs[y] += 1;
    }

    let mut start = n;
    for i in 0..n {
        if degs[i] == 0 {
            if start == n {
                start = i;
            } else {
                println!("No");
                return;
            }
        }
    }

    let mut v = vec![0; n];
    let mut heap = BinaryHeap::new();
    heap.push((1usize, start));
    while let Some((step, i)) = heap.pop() {
        v[i] = step;
        for &x in &edges[i] {
            degs[x] -= 1;
            if degs[x] == 0 {
                heap.push((step + 1, x));
            }
        }
        if heap.len() > 1 {
            println!("No");
            return;
        }
    }
    if v.iter().max() != Some(&n) {
        println!("No");
        return;
    }

    let result = v.iter().join(" ");
    println!("Yes");
    println!("{}", result);
}
