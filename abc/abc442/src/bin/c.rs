use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    }
    let mut v = vec![0isize; n]; // 利害関係にある人数
    for (a, b) in ab {
        v[a] += 1;
        v[b] += 1;
    }
    let mut results = vec![];
    for x in v {
        let k = n as isize - x - 1; // 共著者候補の数
        let result = k * (k - 1) * (k - 2) / 6;
        results.push(result);
    }
    println!("{}", results.iter().join(" "));
}
