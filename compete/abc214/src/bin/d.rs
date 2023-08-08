use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        mut uvw: [(Usize1, Usize1, usize); n - 1],
    }
    uvw.sort_by_key(|k| (k.2, k.0, k.1));

    let mut result = 0;
    let mut uf = UnionFind::new(n);
    let mut a = vec![1; n];
    for &(u, v, w) in &uvw {
        let u = uf.find(u);
        let v = uf.find(v);
        result += a[u] * a[v] * w;
        uf.union(u, v);
        let x = uf.find(u);
        a[x] = a[u] + a[v];
    }
    println!("{}", result);
}
