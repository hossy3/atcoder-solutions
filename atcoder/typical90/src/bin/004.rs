use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize; w]; h],
    }

    let mut sum_c = vec![0; w];
    let mut sum_r = vec![0; h];
    for r in 0..h {
        for c in 0..w {
            sum_c[c] += a[r][c];
            sum_r[r] += a[r][c];
        }
    }

    for r in 0..h {
        let result = (0..w).map(|c| sum_c[c] + sum_r[r] - a[r][c]).join(" ");
        println!("{result}");
    }
}
