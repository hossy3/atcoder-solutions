use ac_library::Dsu;
use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn g(y: usize, graph: &[Vec<usize>], path: &mut Vec<usize>, v: &mut [bool]) -> bool {
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
            return true;
        }
        if g(y, graph, path, v) {
            return true;
        }
        path.pop();
        v[j] = false;
    }
    false
}

fn build_ungraph(n: usize, x: usize, y: usize, uv: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut dsu = Dsu::new(n);
    for &(u, v) in uv {
        if u != x || v != x {
            dsu.merge(u, v);
        }
    }

    let mut graph = vec![vec![]; n]; // node, edge
    for &(u, v) in uv {
        if dsu.same(u, y) || dsu.same(v, y) {
            graph[u].push(v);
            graph[v].push(u);
        }
    }
    for i in 0..n {
        graph[i].sort_unstable();
    }
    graph
}

fn f(n: usize, x: usize, y: usize, uv: &[(usize, usize)]) -> Vec<usize> {
    let graph = build_ungraph(n, x, y, uv);
    let mut v = vec![false; n];
    v[x] = true;
    let mut path = vec![x];
    if !g(y, &graph, &mut path, &mut v) {
        unreachable!()
    };
    path
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
