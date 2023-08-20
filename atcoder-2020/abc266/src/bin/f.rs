use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};
use smallvec::SmallVec;

fn detect_loop(uv: &[(usize, usize)]) -> Vec<bool> {
    let mut edges = vec![SmallVec::<[_; 2]>::new(); uv.len()];
    for &(u, v) in uv {
        edges[u].push(v);
        edges[v].push(u);
    }

    let mut stack = Vec::new();
    for (i, edge) in edges.iter().enumerate() {
        if edge.len() == 1 {
            stack.push(i);
        }
    }

    let mut a = vec![true; uv.len()];
    while let Some(i) = stack.pop() {
        let neighbors: SmallVec::<[_; 2]> = edges[i].iter().filter(|&&i| a[i]).collect();
        if neighbors.len() == 1 {
            a[i] = false; // i is branch
            stack.push(*neighbors[0]);
        }
    }
    
    a
}

fn main() {
    input! {
        uv: [(Usize1, Usize1)],
        xy: [(Usize1, Usize1)],
    }

    let a = detect_loop(&uv);
    let mut uf = UnionFind::new(uv.len());
    for &(u, v) in &uv {
        if !a[u] || !a[v] {
            uf.union(u, v);
        }
    }

    for &(x, y) in &xy {
        let yes = uf.equiv(x, y);
        println!("{}", if yes { "Yes" } else { "No" });
    }
}
