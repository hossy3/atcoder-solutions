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
        n: usize,
        m: usize,
        k: usize,
        s: Usize1,
        t: Usize1,
        x: Usize1,
        uv: [(Usize1, Usize1); m],
    }

    const MOD: usize = 998244353;

    let graph = build_ungraph(n, &uv);
    let mut v = vec![vec![0usize; n]; 2];
    v[0][s] = 1;

    for _ in 0..k {
        let mut v0 = vec![vec![0usize; n]; 2];
        for i in 0..2 {
            for j in 0..n {
                for &j0 in &graph[j] {
                    let i0 = if j0 == x { 1 - i } else { i };
                    v0[i0][j0] += v[i][j];
                }
            }
        }
        for i in 0..2 {
            for j in 0..n {
                v0[i][j] %= MOD;
            }
        }
        v = v0;
    }

    let result = v[0][t];
    println!("{}", result);
}
