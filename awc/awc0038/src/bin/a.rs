use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [usize; n],
    }

    let mut results = vec![1];
    for i in 1..n {
        if h[i] > h[results[results.len() - 1] - 1] {
            results.push(i + 1);
        }
    }
    println!("{}", results.iter().join(" "));
}
