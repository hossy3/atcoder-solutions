use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut v = vec![0usize; 1_000_002];
    for &x in &a {
        v[x] += x;
    }
    for i in 0..=1_000_000 {
        v[i + 1] += v[i];
    }
    let mut results = Vec::with_capacity(n);
    for &x in &a {
        results.push(v[1_000_001] - v[x]);
    }
    let result = results.iter().join(" ");
    println!("{result}");
}
