use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        ab: [(Usize1, Usize1); k],
    }
    let mut v = vec![vec![]; n];
    let mut results = vec![];
    for (a, b) in ab {
        v[a].push(b);
        if v[a].len() == m {
            results.push(a + 1);
        }
    }
    if !results.is_empty() {
        println!("{}", results.iter().join(" "));
    }
}
