use proconio::{input, marker::Usize1};

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

fn f(d: &[Vec<usize>]) -> usize {
    let n = d.len();
    let mut dp = vec![vec![usize::MAX; n]; 1 << n];
    dp[0][1] = 0;
    for bits in 0..(1 << n) {
        for i in 0..n {
            if dp[bits][i] == usize::MAX {
                continue;
            }
            for j in 0..n {
                let score = dp[bits][i] + d[i][j];
                let bits = bits | 1 << j;
                dp[bits][j] = dp[bits][j].min(score);
            }
        }
    }

    let result = (0..n).map(|i| dp[(1 << n) - 1][i] + d[i][1]).min().unwrap();
    result
}

fn main() {
    input! {
        n: usize,
        m: usize,
        uvw: [(Usize1, Usize1, usize); m],
    }

    let mut d = vec![vec![usize::MAX; n]; n];
    for i in 0..n {
        d[i][i] = 0;
    }
    for &(u, v, w) in &uvw {
        d[u][v] = w;
        d[v][u] = w;
    }
    let d = floyd_warshall(d);
    if d.iter().any(|d| d.iter().any(|&x| x == usize::MAX)) {
        println!("-1");
        return;
    }

    let result = f(&d);
    println!("{result}");
}
