use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [isize; n],
        lr: [(Usize1, Usize1); m],
    }

    let mut imos = vec![0isize; n + 1];
    for (l, r) in lr {
        imos[l] += 1;
        imos[r + 1] -= 1;
    }
    for i in 1..=n {
        imos[i] += imos[i - 1];
    }
    let results = (0..n).map(|i| a[i] * imos[i]).collect::<Vec<_>>();
    println!("{}", results.iter().join(" "));
}
