use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        p: [Usize1; n - 1],
        xd: [(Usize1, usize); q],
    }

    let mut v = vec![0; n];
    for (x, d) in xd {
        v[x] += d;
    }

    for (i, &p) in p.iter().enumerate() {
        v[i + 1] += v[p];
    }
    println!("{}", v.iter().join(" "));
}
