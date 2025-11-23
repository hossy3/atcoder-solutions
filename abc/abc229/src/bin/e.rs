use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        mut ab: [(Usize1, Usize1); m],
    }
    ab.sort();
    let mut v = vec![vec![]; n + 1];
    for &(a, b) in &ab {
        v[a].push((a, b));
    }

    let mut uf = UnionFind::new(n);
    let mut results = vec![0; n];
    for i in (1..n).rev() {
        let mut result = results[i] + 1;
        for &(a, b) in &v[i] {
            if uf.union(a, b) {
                result -= 1;
            }
        }
        results[i - 1] = result;
    }

    for &result in &results {
        println!("{}", result);
    }
}
