use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        a: [Chars; n],
    }
    let mut b = vec![vec!['.'; n]; n];
    for i in 0..(n / 2) {
        let i0 = n - i - 1;
        for j in 0..(n / 2) {
            let j0 = n - j - 1;
            match i.min(j) % 4 {
                0 => {
                    b[i][j] = a[j0][i];
                    b[j0][i] = a[i0][j0];
                    b[i0][j0] = a[j][i0];
                    b[j][i0] = a[i][j];
                }
                1 => {
                    b[i][j] = a[i0][j0];
                    b[j0][i] = a[j][i0];
                    b[i0][j0] = a[i][j];
                    b[j][i0] = a[j0][i];
                }
                2 => {
                    b[i][j] = a[j][i0];
                    b[j0][i] = a[i][j];
                    b[i0][j0] = a[j0][i];
                    b[j][i0] = a[i0][j0];
                }
                3 => {
                    b[i][j] = a[i][j];
                    b[j0][i] = a[j0][i];
                    b[i0][j0] = a[i0][j0];
                    b[j][i0] = a[j][i0];
                }
                _ => {
                    unreachable!()
                }
            }
        }
    }

    for b in b {
        println!("{}", b.iter().join(""));
    }
}
