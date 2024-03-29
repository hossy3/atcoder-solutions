use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        tuv: [(usize, Usize1, Usize1)],
    }

    let mut uf = UnionFind::new(n);
    for &(t, u, v) in &tuv {
        if t == 1 {
            uf.union(u, v);
        } else {
            let yes = uf.equiv(u, v);
            println!("{}", if yes { "Yes" } else { "No" });
        }
    }
}
