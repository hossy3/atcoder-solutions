use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        abcd: [(Usize1, Usize1, Usize1, Usize1); n],
    }

    let mut imos = vec![vec![0i64; w + 1]; h + 1];
    for &(a, b, c, d) in &abcd {
        imos[a][b] += 1;
        imos[c + 1][b] -= 1;
        imos[a][d + 1] -= 1;
        imos[c + 1][d + 1] += 1;
    }

    for j in 0..=w {
        for i in 0..h {
            imos[i + 1][j] += imos[i][j];
        }
    }
    for i in 0..=h {
        for j in 0..w {
            imos[i][j + 1] += imos[i][j];
        }
    }

    for i in 0..h {
        let line = imos[i][0..w].iter().join(" ");
        println!("{}", line);
    }
}
