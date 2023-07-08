use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(i128, i128); n],
    }
    const FACTOR: i128 = (2 * 10i128.pow(9)).pow(2);
    let mut v = Vec::with_capacity(n);
    for (i, &(a, b)) in ab.iter().enumerate() {
        v.push((i + 1, FACTOR * a / (a + b)));
    }
    v.sort_by_key(|(_, x)| -x);
    let result = v.iter().map(|(i, _)| i).join(" ");
    println!("{}", result);
}
