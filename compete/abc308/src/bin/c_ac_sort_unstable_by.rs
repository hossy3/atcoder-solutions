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
    v.sort_unstable_by(|l, r| {
        let (li, la, lb) = l;
        let (ri, ra, rb) = r;
        ((ra * (la + lb)), li).cmp(&((la * (ra + rb)), ri))
    });
    let result = v.iter().map(|(i, _, _)| i).join(" ");
    println!("{}", result);
}
