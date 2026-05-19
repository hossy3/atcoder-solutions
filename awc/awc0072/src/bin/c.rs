use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        s: [isize; n],
        lr: [(Usize1, Usize1); q],
    }

    let mut imos = vec![0isize; n + 1];
    imos[0] = s[0];
    for i in 0..(n - 1) {
        imos[i + 1] = s[i + 1] - s[i];
    }
    for &(l, r) in &lr {
        imos[l] -= 1;
        imos[r + 1] += 1;
    }
    for i in 0..n {
        imos[i + 1] += imos[i];
    }

    let results = imos[..n].iter().map(|&x| x.max(0)).collect::<Vec<_>>();
    println!("{}", results.iter().join(" "));
}
