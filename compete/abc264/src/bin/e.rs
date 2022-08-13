use std::collections::HashSet;

use petgraph::unionfind::UnionFind;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        e: usize,
        uv: [(usize, usize); e],
        q: usize,
        x: [usize; q],
    }

    let mut s = HashSet::new();
    for x in &x {
        s.insert(x - 1);
    }

    let mut uf = UnionFind::new(n + m);
    for i in 0..(uv.len()) {
        if s.contains(&i) {
            continue;
        }
        let (u, v) = uv[i];
        uf.union(u - 1, v - 1);
    }
    for i in (n + 1)..(n + m) {
        uf.union(n, i);
    }

    let mut counts = vec![0usize; n + m];
    for i in 0..(n + m) {
        counts[uf.find(i)] += 1;
    }
    let mut results = vec![0usize; q];
    let mut count = counts[uf.find(n)] - m;
    for i in (0..q).rev() {
        results[i] = count;
        let (u, v) = uv[x[i] - 1];
        let n0 = uf.find(n);
        let u0 = uf.find(u - 1);
        let v0 = uf.find(v - 1);
        if u0 == n0 && u0 != v0 {
            uf.union(u0, v0);
            count += counts[v0];
        }
        if v0 == n0 && u0 != v0 {
            uf.union(u0, v0);
            count += counts[u0];
        }
        if u0 != n0 && v0 != n0 && u0 != v0 {
            uf.union(u0, v0);
            counts[u0] += counts[v0];
            counts[v0] = counts[u0];
        }
    }

    for r in results {
        println!("{}", r);
    }
}
