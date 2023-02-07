use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    }
    let mut uf = UnionFind::new(n);
    let mut count = 0;
    for (a, b) in ab {
        if uf.equiv(a, b) {
            count += 1;
        } else {
            uf.union(a, b);
        }
    }
    println!("{}", count);
}
