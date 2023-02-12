use std::{cmp::Reverse, collections::BinaryHeap};

use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(Usize1, Usize1, usize); m],
    }

    let mut graph = vec![vec![]; n];
    for &(a, b, c) in &abc {
        graph[a].push((b, c));
        graph[b].push((a, c));
    }

    let mut prev = vec![None; n];
    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), 0, 0));
    while let Some((Reverse(step), i, j)) = heap.pop() {
        if prev[j] != None {
            continue;
        }
        prev[j] = Some(i);
        for &(k, c) in &graph[j] {
            if prev[k] != None {
                continue;
            }
            heap.push((Reverse(step + c), j, k));
        }
    }

    let mut v = vec![n - 1];
    while let Some(&i) = v.last() {
        if i == 0 {
            break;
        }
        if let Some(j) = prev[i] {
            v.push(j);
        }
    }
    let result = v.iter().rev().map(|&x| x + 1).join(" ");
    println!("{}", result);
}
