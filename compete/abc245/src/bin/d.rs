use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i64; n + 1],
        mut c: [i64; n + m + 1],
    }
    let mut b = vec![0; m + 1];
    for i in (0..=m).rev() {
        b[i] = c[n + i] / a[n];
        for j in 0..=n {
            c[j + i] -= a[j] * b[i];
        }
    }
    let result = b.iter().join(" ");
    println!("{}", result);
}
