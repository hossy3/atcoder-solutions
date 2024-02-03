use pathfinding::undirected::kruskal::kruskal;
use proconio::{input, marker::Usize1};

fn f(n: usize, clr: &[(i64, usize, usize)]) -> i64 {
    let edges: Vec<_> = clr.iter().map(|&(c, l, r)| (l, r + 1, c)).collect();
    let v: Vec<_> = kruskal(&edges).collect();
    if v.len() == n {
        v.iter().map(|(_, _, c)| c).sum::<i64>()
    } else {
        -1
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        clr: [(i64, Usize1, Usize1); m],
    }
    let result = f(n, &clr);
    println!("{result}");
}
