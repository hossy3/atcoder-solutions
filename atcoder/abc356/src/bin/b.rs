use std::iter::zip;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m],
        x: [[usize; m]; n],
    }

    let a0: Vec<_> = (0..m)
        .map(|i| (0..n).map(|j| x[j][i]).sum::<usize>())
        .collect();
    let yes = zip(a, a0).all(|t| t.0 <= t.1);
    println!("{}", if yes { "Yes" } else { "No" });
}
