use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        lr: [(Usize1, Usize1); m],
    }

    let mut imos = vec![0i32; n + 1];
    for (l, r) in lr {
        imos[l] += 1;
        imos[r + 1] -= 1;
    }

    let mut v = vec![0; n];
    v[0] = imos[0];
    for i in 1..n {
        v[i] = v[i - 1] + imos[i];
    }
    println!("{}", v.iter().join(" "));
}
