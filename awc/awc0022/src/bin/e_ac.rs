use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

use proconio::{input, marker::Usize1};

fn floyd_warshall(mut d: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let n = d.len();
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                d[i][j] = d[i][j].min(d[i][k].saturating_add(d[k][j]));
            }
        }
    }
    d
}

trait BitTest {
    fn bit_test(&self, i: usize) -> bool;
}

impl BitTest for usize {
    fn bit_test(&self, i: usize) -> bool {
        self & (1 << i) != 0
    }
}

// 正解だけれど遅い。 bits は常に大きくなる方向に変わるので、このようなキューは必要ない

fn f(d: &[Vec<usize>]) -> usize {
    let n = d.len();
    let mut set = HashSet::new();
    let mut queue = BinaryHeap::new();
    queue.push((Reverse(0usize), 0usize, 1usize));
    let mut result = usize::MAX;
    while let Some((Reverse(score), i, bits)) = queue.pop() {
        // eprintln!("{}, {}, {:?}", i, score, &map);
        if set.contains(&(i, bits)) {
            continue;
        }
        set.insert((i, bits));
        if bits == (1 << n) - 1 {
            result = result.min(score + d[i][0]);
        }

        for j in 0..n {
            if bits.bit_test(j) {
                continue;
            }
            let score = score + d[i][j];
            if set.contains(&(j, bits)) {
                continue;
            }
            let bits = bits | 1 << j;
            queue.push((Reverse(score), j, bits));
        }
    }

    result
}

fn main() {
    input! {
        n: usize,
        m: usize,
        uvw: [(Usize1, Usize1, usize); m],
    }

    let mut d = vec![vec![usize::MAX; n]; n];
    for i in 0..n {
        d[i][i] = 0;
    }
    for &(u, v, w) in &uvw {
        d[u][v] = w;
        d[v][u] = w;
    }
    let d = floyd_warshall(d);
    if d.iter().any(|d| d.iter().any(|&x| x == usize::MAX)) {
        println!("-1");
        return;
    }

    let result = f(&d);
    println!("{result}");
}
