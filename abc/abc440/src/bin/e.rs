use std::collections::{BinaryHeap, HashSet};

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        x: usize,
        mut a: [isize; n],
    }
    a.sort();
    a.reverse();
    let a = a;

    let mut set = HashSet::new();
    let mut heap = BinaryHeap::new();
    let mut results = vec![];
    let mut v0 = vec![0; n];
    v0[0] = k;
    heap.push((a[0] * k as isize, v0.clone()));
    set.insert(v0.clone());

    while let Some((score, v)) = heap.pop() {
        results.push(score);
        if results.len() == x {
            break;
        }

        let mut v = v.clone();
        for i in 0..(n - 1) {
            if v[i] > 0 {
                v[i] -= 1;
                v[i + 1] += 1;
                if !set.contains(&v) {
                    let score = score - a[i] + a[i + 1];
                    heap.push((score, v.clone()));
                    set.insert(v.clone());
                }
                v[i + 1] -= 1;
                v[i] += 1;
            }
        }
    }

    for result in results {
        println!("{result}");
    }
}
