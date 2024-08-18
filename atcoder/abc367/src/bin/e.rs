use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn build_doubling(k: usize, x: &[usize]) -> Vec<Vec<usize>> {
    let n = x.len();
    let nbits = k.ilog2() as usize + 1;
    let mut m = vec![vec![0; nbits]; n];
    for (i, &x) in x.iter().enumerate() {
        m[i][0] = x;
    }
    for i in 0..(nbits - 1) {
        for j in 0..n {
            m[j][i + 1] = m[m[j][i]][i];
        }
    }
    m
}

fn main() {
    input! {
        n: usize,
        k: usize,
        x: [Usize1; n],
        a: [usize; n],
    }

    if k == 0 {
        println!("{}", a.iter().join(" "));
        return;
    }

    let m = build_doubling(k, &x);
    let mut v: Vec<_> = (0..n).collect();
    for i in 0..(m[0].len()) {
        if k & (1 << i) != 0 {
            for j in 0..n {
                v[j] = m[v[j]][i];
            }
        }
    }

    let results: Vec<_> = (0..n).map(|i| a[v[i]]).collect();
    println!("{}", results.iter().join(" "));
}
