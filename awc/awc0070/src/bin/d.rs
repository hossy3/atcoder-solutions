use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
        b: [[usize; m]; n],
        rcrc: [(Usize1, Usize1, usize, usize); q],
    }

    let mut a = vec![vec![0usize; m + 1]; n + 1];
    for &(r1, c1, r2, c2) in &rcrc {
        a[r1][c1] += 1;
        a[r2][c2] += 1;
        a[r1][c2] += 1;
        a[r2][c1] += 1;
    }
    let sum = {
        let mut sum = vec![vec![0; a[0].len() + 1]; a.len() + 1];
        for (i, a) in a.iter().enumerate() {
            for (j, &a) in a.iter().enumerate() {
                sum[i + 1][j + 1] = sum[i][j + 1] + sum[i + 1][j] - sum[i][j] + a;
            }
        }
        sum
    };

    let mut result = 0;
    for r in 0..n {
        for c in 0..m {
            result += b[r][c] * if sum[r + 1][c + 1] % 2 == 0 { 1 } else { 2 };
        }
    }
    println!("{result}");
}
