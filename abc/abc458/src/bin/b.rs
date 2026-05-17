use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
    }

    let mut results = vec![vec![0usize; w]; h];
    for i in 0..h {
        for j in 0..w {
            let mut x = 0;
            if i > 0 {
                x += 1;
            }
            if i + 1 < h {
                x += 1;
            }
            if j > 0 {
                x += 1;
            }
            if j + 1 < w {
                x += 1;
            }
            results[i][j] = x;
        }
    }

    for results in results {
        println!("{}", results.iter().join(" "));
    }
}
