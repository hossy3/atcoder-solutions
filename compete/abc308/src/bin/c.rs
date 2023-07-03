use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
    }
    let mut v = Vec::with_capacity(n);
    for (i, &(a, b)) in ab.iter().enumerate() {
        v.push((i + 1, a, b));
    }
    v.sort_by(|a, b| (b.1 * (a.1 + a.2)).cmp(&(a.1 * (b.1 + b.2))));
    let result = v.iter().map(|x| x.0).join(" ");
    println!("{}", result);
}
