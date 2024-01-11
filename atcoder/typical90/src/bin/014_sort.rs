use std::iter;

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
        mut b: [usize; n],
    }
    a.sort();
    b.sort();
    let result: usize = iter::zip(a, b).map(|(a, b)| a.abs_diff(b)).sum();
    println!("{result}");
}
