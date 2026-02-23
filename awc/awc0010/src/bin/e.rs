use std::collections::{HashMap, VecDeque};

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        c: [[usize; n]; n],
    }

    // サイクル数を求める
    let mut queue = VecDeque::new();
    queue.push_back(((0..n).collect::<Vec<_>>(), 0));
    let mut cycle = HashMap::new();
    while let Some((v, k0)) = queue.pop_front() {
        cycle.insert(v.clone(), k0);
        if k0 == k {
            continue;
        }
        for i in 0..(n - 1) {
            for j in (i + 1)..n {
                let mut v = v.clone();
                v.swap(i, j);
                if !cycle.contains_key(&v) {
                    cycle.insert(v.clone(), k0 + 1);
                    queue.push_back((v, k0 + 1));
                }
            }
        }
    }

    let mut result = 0usize;
    for (v, _) in cycle {
        let x = (0..n).map(|i| c[v[i]][v[(i + 1) % n]]).sum::<usize>();
        result = result.max(x);
    }

    println!("{result}");
}
