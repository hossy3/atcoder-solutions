use proconio::{input, marker::Usize1};

fn build_ungraph(n: usize, uv: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut graph = vec![vec![]; n]; // node, edge
    for &(u, v) in uv {
        graph[u].push(v);
        graph[v].push(u);
    }
    graph
}

fn main() {
    input! {
        (n, m): (usize, usize),
        ab: [(Usize1, Usize1); m],
        q: usize,
        xy: [(Usize1, usize); q],
    }

    const N: usize = 1000; // この数以上のものは遅延適用する

    let graph = build_ungraph(n, &ab);
    let mut v = vec![(1usize, 0usize); n]; // (value, changed time)

    let mut v0 = vec![(1usize, 0usize); n]; // (value, changed time) for 遅延適用
    let mut lazy = vec![vec![]; n];
    for (i, v) in graph.iter().enumerate() {
        if v.len() >= N {
            for &j in v {
                lazy[j].push(i);
            }
        }
    }

    for (t, &(x, y)) in xy.iter().enumerate() {
        for &x0 in &lazy[x] {
            if v[x].1 < v0[x0].1 {
                v[x] = v0[x0];
            }
        }

        println!("{}", v[x].0);

        v[x] = (y, t + 1);
        if graph[x].len() < N {
            for &i in &graph[x] {
                v[i] = v[x];
            }
        } else {
            v0[x] = v[x];
        }
    }
}
