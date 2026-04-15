use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        lrc: [(Usize1, Usize1, isize); q],
    }
    let mut imos = vec![0; n + 1];
    for (l, r, c) in lrc {
        imos[l] += c;
        imos[r + 1] -= c;
    }
    let mut results = vec![0; n];
    results[0] = imos[0];
    for i in 1..n {
        results[i] = results[i - 1] + imos[i];
    }
    println!("{}", results.iter().join(" "));
}
