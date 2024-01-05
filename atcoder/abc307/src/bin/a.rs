use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; 7 * n],
    }
    let mut v = vec![0; n];
    for i in 0..n {
        for j in 0..7 {
            v[i] += a[i * 7 + j];
        }
    }
    println!("{}", v.iter().join(" "));
}
