use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};

fn is_bipartite_graph(n: usize, ab: &[(usize, usize)]) -> bool {
    let mut uf = UnionFind::new(n * 2);
    for &(a, b) in ab {
        uf.union(a, b + n);
        uf.union(b, a + n);
    }
    for i in 0..n {
        if uf.equiv(i, i + n) {
            return false;
        }
    }
    true
}

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    }
    let yes = is_bipartite_graph(n, &ab);
    println!("{}", if yes { "Yes" } else { "No" });
}
