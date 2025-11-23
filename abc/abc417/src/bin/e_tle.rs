use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn g(y: usize, graph: &[Vec<usize>], path: &mut Vec<usize>, v: &mut [bool]) -> Option<Vec<usize>> {
    let Some(&i) = path.last() else {
        unreachable!();
    };
    for &j in &graph[i] {
        if v[j] {
            continue;
        }
        path.push(j);
        v[j] = true;
        if j == y {
            return Some(path.clone());
        }
        let result = g(y, graph, path, v);
        if result.is_some() {
            return result;
        }
        path.pop();
        v[j] = false;
    }
    None
}

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

fn f(n: usize, x: usize, y: usize, uv: &[(usize, usize)]) -> Vec<usize> {
    let graph = build_ungraph(n, uv);
    let mut v = vec![false; n];
    v[x] = true;
    let mut path = vec![x];
    let Some(result) = g(y, &graph, &mut path, &mut v) else {
        unreachable!()
    };
    result
}

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            m: usize,
            x: Usize1,
            y: Usize1,
            uv: [(Usize1, Usize1); m],
        }

        let results = f(n, x, y, &uv);
        println!("{}", results.iter().map(|i| i + 1).join(" "));
    }
}
