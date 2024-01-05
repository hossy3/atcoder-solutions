use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(Usize1, Usize1); m],
    }
    let mut set = HashSet::new();
    let mut uf = UnionFind::new(n * 2);
    for &(u, v) in &uv {
        uf.union(u, v + n);
        uf.union(u + n, v);
        set.insert(u.min(v) * n + u.max(v));
    }
    let mut a = vec![0; 2 * n];
    for i in 0..(2 * n) {
        a[i] = uf.find(i);
    }
    for i in 0..n {
        if a[i] == a[i + n] {
            println!("{}", 0);
            return;
        }
    }

    let mut count = 0;
    for i in 0..(n - 1) {
        for j in (i + 1)..n {
            if set.contains(&(i * n + j)) {
                continue;
            }
            if a[i] == a[j] {
                continue;
            }
            count += 1;
        }
    }
    println!("{}", count);
}
