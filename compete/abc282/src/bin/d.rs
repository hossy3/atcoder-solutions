use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(Usize1, Usize1); m],
    }
    let mut uf = UnionFind::new(n * 2);
    for &(u, v) in &uv {
        uf.union(u, v + n);
        uf.union(u + n, v);
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

    let mut map = HashMap::<usize, Vec<_>>::new();
    for i in 0..n {
        let j = a[i];
        if let Some(x) = map.get_mut(&j) {
            x.push(i);
        } else {
            map.insert(j, vec![i]);
        }
    }

    let mut a0 = Vec::new();
    for (_, v) in &map {
        a0.push(v.len());
    }
    let mut count = n * (n - 1) / 2 - m;
    for &x in &a0 {
        count -= x * (x - 1) / 2;
    }
    println!("{}", count);
}
