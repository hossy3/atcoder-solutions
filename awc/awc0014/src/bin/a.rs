use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        r: usize,
        t: usize,
        p: [usize; n],
    }

    let mut results = vec![];
    for p in p {
        results.push((t / p).min(r));
    }
    println!("{}", results.iter().join(" "));
}
