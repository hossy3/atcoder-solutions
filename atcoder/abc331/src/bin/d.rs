use proconio::{input, marker::Chars};

fn f(m: &[Vec<usize>], i: usize, j: usize) -> usize {
    let n = m.len() - 1;
    let (i0, i1) = (i / n, i % n);
    let (j0, j1) = (j / n, j % n);
    i0 * j0 * m[n][n] + j0 * m[i1][n] + i0 * m[n][j1] + m[i1][j1]
}

fn main() {
    input! {
        n: usize,
        q: usize,
        p: [Chars; n],
        abcd: [(usize, usize, usize, usize); q],
    }

    let mut m = vec![vec![0usize; n + 1]; n + 1];
    for i in 0..n {
        for j in 0..n {
            m[i + 1][j + 1] = if p[i][j] == 'B' { 1 } else { 0 };
        }
    }
    for i in 0..=n {
        for j in 0..n {
            m[i][j + 1] += m[i][j];
        }
    }
    for i in 0..n {
        for j in 0..=n {
            m[i + 1][j] += m[i][j];
        }
    }

    for &(a, b, c, d) in &abcd {
        let result = f(&m, c + 1, d + 1) + f(&m, a, b) - f(&m, a, d + 1) - f(&m, c + 1, b);
        println!("{result}");
    }
}
