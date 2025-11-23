use itertools::Itertools;
use proconio::{input, marker::Usize1};

trait BitTest {
    fn bit_test(&self, i: usize) -> bool;
}

impl BitTest for usize {
    fn bit_test(&self, i: usize) -> bool {
        self & (1 << i) != 0
    }
}

fn floyd_warshall(mut d: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let n = d.len();
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                d[i][j] = d[i][j].min(d[i][k] + d[k][j]);
            }
        }
    }
    d
}

fn main() {
    input! {
        n: usize,
        m: usize,
        uvt: [(Usize1, Usize1, usize); m],
        q: usize,
    }

    let mut d = vec![vec![usize::MAX / n; n]; n];
    for u in 0..n {
        d[u][u] = 0;
    }
    for &(u, v, t) in &uvt {
        d[u][v] = d[u][v].min(t);
        d[v][u] = d[v][u].min(t);
    }
    d = floyd_warshall(d);

    for _ in 0..q {
        input! {
            k: usize,
            b: [Usize1; k],
        }
        let mut result = usize::MAX;
        for b in b.iter().permutations(b.len()) {
            for i in 0usize..(2 << k) {
                let mut cur = 0usize;
                let mut result0 = 0usize;
                for (j, &&b) in b.iter().enumerate() {
                    let (u, v, t) = uvt[b];
                    if i.bit_test(j) {
                        result0 += d[cur][u] + t;
                        cur = v;
                    } else {
                        result0 += d[cur][v] + t;
                        cur = u;
                    }
                }
                result0 += d[cur][n - 1];
                result = result.min(result0);
            }
        }
        println!("{result}");
    }
}
