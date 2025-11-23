use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut results = vec![-1; n];
    for i in 0..n {
        for j in (0..i).rev() {
            if a[j] > a[i] {
                results[i] = (j + 1) as isize;
                break;
            }
        }
    }

    println!("{}", results.iter().join("\n"));
}
