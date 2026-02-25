use std::collections::HashSet;

use proconio::{input, marker::Usize1};

fn build_digraph(n: usize, uv: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut graph = vec![vec![]; n]; // node, edge
    for &(u, v) in uv {
        graph[u].push(v);
    }
    graph
}

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(Usize1, Usize1); m],
    }

    let graph = build_digraph(n, &uv);
    let mut results = vec![];

    let mut visited = HashSet::new();
    let mut deleted = HashSet::new(); // 辿る方法があり削除していたノード
    deleted.insert(0);

    for u in 0..n {
        if !deleted.remove(&u) {
            results.push(-1); // 現時点ではたどり着けない
            continue;
        }
        visited.insert(u);

        let mut stack = graph[u].clone();
        while let Some(v) = stack.pop() {
            if v > u {
                deleted.insert(v); // いったん削除して辿れなくする
            } else if visited.insert(v) {
                stack.append(&mut graph[v].clone());
            }
        }

        if visited.len() == u + 1 {
            results.push(deleted.len() as isize);
        } else {
            results.push(-1); // たどり着けないノードがある
        }
    }

    for result in results {
        println!("{result}");
    }
}
