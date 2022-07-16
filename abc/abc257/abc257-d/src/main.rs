// -*- coding:utf-8-unix -*-

use std::{
    cmp::Reverse,
    collections::{BTreeSet, BinaryHeap},
};

use proconio::input;

fn main() {
    input! {
        n: usize,
        xyp: [(i64, i64, i64); n],
    }
    let mut dict = vec![Vec::<(i64, usize)>::new(); n]; // dict[from] = (weight, to)[]
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            let s = ((xyp[j].0 - xyp[i].0).abs() + (xyp[j].1 - xyp[i].1).abs() + xyp[i].2 - 1)
                / xyp[i].2;
            dict[i].push((s, j));
        }
    }

    let mut min_score = 1i64 << 62;
    'outer: for i in 0..n {
        let mut heap = BinaryHeap::<(Reverse<i64>, usize)>::new();
        for (weight, j) in &dict[i] {
            heap.push((Reverse(*weight), *j));
        }
        let mut reached = BTreeSet::<usize>::new();
        reached.insert(i);
        let mut score = 0i64;
        while let Some((weight, j)) = heap.pop() {
            score = score.max(weight.0);
            if reached.contains(&j) {
                continue;
            }
            reached.insert(j);
            if reached.len() == n {
                min_score = min_score.min(score);
                continue 'outer;
            }
            for (weight, j) in &dict[j] {
                heap.push((Reverse(*weight), *j));
            }
        }
    }

    println!("{}", min_score);
}
