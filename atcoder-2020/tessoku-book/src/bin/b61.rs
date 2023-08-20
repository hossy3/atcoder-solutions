use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    }
    let mut v = vec![0usize; n];
    for (a, b) in ab {
        v[a] += 1;
        v[b] += 1;
    }
    let result = v.iter().position_max().unwrap() + 1;
    println!("{}", result);
}
