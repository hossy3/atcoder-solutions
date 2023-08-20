use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        k: usize,
        ab: [(Usize1, Usize1); n],
    }
    let m = 5000;
    let mut v = vec![vec![0; m]; m];
    for &(a, b) in &ab {
        v[a][b] += 1;
    }
    let mut cum = vec![vec![0; m + 1]; m + 1];
    for i in 0..m {
        for j in 0..m {
            cum[i + 1][j + 1] = cum[i + 1][j] + cum[i][j + 1] - cum[i][j] + v[i][j];
        }
    }

    let mut count = 0;
    for i in 0..(m - k) {
        for j in 0..(m - k) {
            let i0 = i + k + 1;
            let j0 = j + k + 1;
            count = count.max(cum[i0][j0] + cum[i][j] - cum[i0][j] - cum[i][j0]);
        }
    }
    println!("{}", count);
}
