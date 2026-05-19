use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn build_ungraph(n: usize, uv: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut graph = vec![vec![]; n]; // node, edge
    for &(u, v) in uv {
        graph[u].push(v);
        graph[v].push(u);
    }
    for i in 0..n {
        graph[i].sort_unstable();
    }
    graph
}

fn main() {
    input! {
        n: usize,
        m: usize,
        r: [usize; n],
        uv: [(Usize1, Usize1); m],
    }

    let graph = build_ungraph(n, &uv);
    let mut used = vec![false; n];
    let v = (0..n)
        .sorted_unstable_by_key(|&i| r[i])
        .rev()
        .collect::<Vec<_>>();
    for i in v {
        if used[i] {
            continue;
        }
        used[i] = true;
        let j = *graph[i].iter().find(|&&j| !used[j]).unwrap();
        used[j] = true;
        if i == 0 {
            println!("{}", j + 1);
            return;
        } else if j == 0 {
            println!("{}", i + 1);
            return;
        }
    }
}
