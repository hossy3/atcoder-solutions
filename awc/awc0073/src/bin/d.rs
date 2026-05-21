use proconio::{input, marker::Usize1};

/// ワーシャルフロイド法 (floyd_warshall) で、重み付きグラフの全ペアの最短経路を解く
/// * `d`: 隣接行列 (d[u][v] が `u` から `v` への重み。 移動不可能は `usize::MAX` で示す)
fn floyd_warshall(mut d: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let n = d.len();
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                d[i][j] = d[i][j].min(d[i][k].saturating_add(d[k][j]));
            }
        }
    }
    d
}

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        uvw: [(Usize1, Usize1, usize); k],
        st: [(Usize1, Usize1); n],
    }

    let mut d = vec![vec![usize::MAX; m]; m];
    for i in 0..m {
        d[i][i] = 0;
    }
    for &(u, v, w) in &uvw {
        d[u][v] = d[u][v].min(w);
        d[v][u] = d[u][v];
    }
    let d = floyd_warshall(d);

    let mut result = 0;
    for &(s, t) in &st {
        result += d[s][t];
    }
    println!("{result}");
}
