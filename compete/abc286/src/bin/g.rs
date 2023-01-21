use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(Usize1, Usize1); m],
        k: usize,
        x: [Usize1; k],
    }

    let mut a = vec![false; m];
    for &x in &x {
        a[x] = true;
    }
    let mut uf = UnionFind::new(n);
    for (i, &(u, v)) in uv.iter().enumerate() {
        if !a[i] {
            uf.union(u, v);
        }
    }

    let mut edges = vec![0; n];
    for &x in &x {
        let (u, v) = (uf.find(uv[x].0), uf.find(uv[x].1));
        edges[u] += 1;
        edges[v] += 1;
    }
    let yes = edges.iter().filter(|&&x| x % 2 == 1).count() <= 2;
    println!("{}", if yes { "Yes" } else { "No" });
}
