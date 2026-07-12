use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        cs: [(Usize1, isize); n],
    }
    let mut results = vec![-1; m];
    for &(c, s) in &cs {
        results[c] = results[c].max(s);
    }
    println!("{}", results.iter().join(" "));
}
