use proconio::{input, marker::Usize1};

fn main() {
    input! {
        h: usize,
        w: usize,
        x: [[usize; w]; h],
        q: usize,
        abcd: [(Usize1, Usize1, Usize1, Usize1); q],
    }

    let mut cum = vec![vec![0; w + 1]; h + 1];
    for i in 0..h {
        for j in 0..w {
            cum[i + 1][j + 1] = x[i][j] + cum[i][j + 1] + cum[i + 1][j] - cum[i][j];
        }
    }

    for &(a, b, c, d) in &abcd {
        let result = cum[c + 1][d + 1] + cum[a][b] - cum[a][d + 1] - cum[c + 1][b];
        println!("{}", result);
    }
}
