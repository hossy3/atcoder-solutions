use std::collections::BinaryHeap;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        p: [Usize1; n - 1],
        xy: [(Usize1, usize); m],
    }

    let mut v = vec![vec![]; n]; // children
    for (i, &x) in p.iter().enumerate() {
        v[x].push(i + 1);
    }

    let mut covered = vec![false; n];
    let mut queue = BinaryHeap::new();
    for &(x, y) in &xy {
        queue.push((y, x));
    }

    while let Some((y, x)) = queue.pop() {
        if covered[x] {
            continue;
        }
        covered[x] = true;
        if y == 0 {
            continue;
        }
        let y = y - 1;
        for &x in &v[x] {
            if !covered[x] {
                queue.push((y, x));
            }
        }
    }

    let result = covered.iter().filter(|&&x| x).count();
    println!("{}", result);
}
