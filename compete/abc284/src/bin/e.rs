use std::collections::HashSet;

use proconio::{input, marker::Usize1};

fn f(i: usize, mut count: usize, g: &[Vec<usize>], s: &mut HashSet<usize>) -> usize {
    const MAX: usize = 1000000;

    if s.contains(&i) || count >= MAX {
        return count.min(MAX);
    }

    s.insert(i);
    count += 1;
    for &i in &g[i] {
        count = f(i, count, g, s);
    }

    s.remove(&i);
    count.min(MAX)
}

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(Usize1, Usize1); m],
    }

    let mut g = vec![vec![]; n];
    for &(u, v) in &uv {
        g[u].push(v);
        g[v].push(u);
    }

    let mut s = HashSet::new();
    let count = f(0, 0, &g, &mut s);
    println!("{}", count);
}
