use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut m = vec![vec![0; n]; n];
    let mut i = 0;
    let mut j = (n - 1) / 2;
    for k in 1..=(n * n) {
        m[i][j] = k;
        if k % n == 0 {
            i = (i + 1) % n;
        } else {
            i = (i + n - 1) % n;
            j = (j + 1) % n;
        }
    }

    for i in 0..n {
        println!("{}", m[i].iter().join(" "));
    }
}
