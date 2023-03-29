use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    }
    let mut uf = UnionFind::new(n);
    for &(a, b) in &ab {
        uf.union(a, b);
    }
    let yes = (1..n).all(|x| uf.equiv(0, x));
    println!(
        "{}",
        if yes {
            "The graph is connected."
        } else {
            "The graph is not connected."
        }
    );
}
