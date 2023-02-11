use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

use proconio::{input, marker::Usize1};

fn f() -> i64 {
    input! {
        n: usize,
        m: usize,
        c: [usize; n],
        uv: [(Usize1, Usize1); m],
    }

    if c[0] == c[n - 1] {
        return -1;
    }

    let mut graph = vec![vec![]; n];
    for (u, v) in uv {
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut set = HashSet::new();
    let mut heap = BinaryHeap::new();

    set.insert((0, n - 1));
    heap.push((Reverse(0i64), 0, n - 1));

    while let Some((Reverse(step), i, j)) = heap.pop() {
        for &i0 in &graph[i] {
            for &j0 in &graph[j] {
                if c[i0] != c[j0] && !set.contains(&(i0, j0)) {
                    set.insert((i0, j0));
                    heap.push((Reverse(step + 1), i0, j0));
                    if i0 == n - 1 && j0 == 0 {
                        return step + 1;
                    }
                }
            }
        }
    }

    return -1;
}

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        let result = f();
        println!("{}", result);
    }
}
