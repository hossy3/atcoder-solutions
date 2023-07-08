use itertools::Itertools;
use num_rational::Ratio;
use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(i64, i64); n],
    }
    let mut v = Vec::with_capacity(n);
    for (i, &(a, b)) in ab.iter().enumerate() {
        v.push((i + 1, Ratio::new(a, a + b)));
    }
    v.sort_by(|(_, l), (_, r)| r.cmp(l));
    let result = v.iter().map(|(i, _)| i).join(" ");
    println!("{}", result);
}
