use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(f64, f64); n],
    }
    let mut v = Vec::with_capacity(n);
    for (i, &(a, b)) in ab.iter().enumerate() {
        v.push((i + 1, a / (a + b)));
    }
    v.sort_by(|(_, l), (_, r)| r.partial_cmp(l).unwrap());
    let result = v.iter().map(|(i, _)| i).join(" ");
    println!("{}", result);
}
