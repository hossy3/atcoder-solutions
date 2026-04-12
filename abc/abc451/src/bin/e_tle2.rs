use std::collections::BTreeSet;

use ac_library::Dsu;
use proconio::input;

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

fn f(a: &[Vec<usize>]) -> bool {
    let n = a.len() + 1;

    let mut set = BTreeSet::new();
    for i in 0..(n - 1) {
        for j in 0..(n - i - 1) {
            set.insert((a[i][j], i, i + j + 1));
        }
    }

    let mut d = vec![vec![usize::MAX; n]; n];
    let mut dsu = Dsu::new(n);

    for (x, i, j) in set {
        if !dsu.same(i, j) {
            d[i][j] = x;
            d[j][i] = x;
            dsu.merge(i, j);
        }
    }

    let d = floyd_warshall(d);
    for i in 0..(n - 1) {
        for j in 0..(n - i - 1) {
            if a[i][j] != d[i][i + j + 1] {
                return false;
            }
        }
    }

    true
}

fn main() {
    input! {
        n: usize,
    }

    let mut a = vec![];
    for i in 1..n {
        input! {
            a0: [usize; n - i],
        }
        a.push(a0);
    }

    let yes = f(&a);
    println!("{}", if yes { "Yes" } else { "No" });
}
