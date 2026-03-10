use std::collections::{BinaryHeap, HashMap};

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

fn f(d: &[Vec<usize>]) -> usize {
    let n = d.len();
    let mut map = HashMap::new();
    let mut queue = BinaryHeap::new();
    queue.push((0usize, 0usize, 1usize));
    let mut result = usize::MAX;
    while let Some((i, score, bits)) = queue.pop() {
        // eprintln!("{}, {}, {:?}", i, score, &map);
        if let Some(&score0) = map.get(&(i, bits)) {
            if score >= score0 {
                continue;
            }
        }
        map.insert((i, bits), score);
        if bits == (1 << n) - 1 {
            result = result.min(score + d[i][0]);
        }

        for j in 0..n {
            if bits.bit_test(j) {
                continue;
            }
            let score = score + d[i][j];
            if let Some(&score0) = map.get(&(j, bits)) {
                if score >= score0 {
                    continue;
                }
            }
            let bits = bits | 1 << j;
            queue.push((j, score, bits));
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
