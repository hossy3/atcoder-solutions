use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        ab: [(Usize1, Usize1); n - 1],
        cd: [(Usize1, Usize1); q],
    }
    let mut uf = UnionFind::new(n * 2);
    for &(a, b) in &ab {
        uf.union(a + n, b);
        uf.union(a, b + n);
    }
    for &(c, d) in &cd {
        let yes = uf.equiv(c, d);
        println!("{}", if yes { "Town" } else { "Road" });
    }
}
