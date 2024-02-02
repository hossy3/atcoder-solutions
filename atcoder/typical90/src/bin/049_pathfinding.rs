use ac_library::Dsu;
use pathfinding::undirected::kruskal::kruskal;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        clr: [(i64, Usize1, Usize1); m],
    }

    let mut uf = Dsu::new(n + 1);
    for &(_, l, r) in clr.iter() {
        if !uf.same(l, r + 1) {
            uf.merge(l, r + 1);
        }
    }
    if (0..n).any(|i| !uf.same(i, n)) {
        println!("-1");
        return;
    }

    let edges: Vec<_> = clr.iter().map(|&(c, l, r)| (l, r + 1, c)).collect();
    let v = kruskal(&edges);
    let result: i64 = v.map(|(_, _, c)| c).sum();
    println!("{result}");
}
