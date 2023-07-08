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
    v.sort_by(|l, r| {
        let (_, la, lb) = l;
        let (_, ra, rb) = r;
        (ra * (la + lb)).cmp(&(la * (ra + rb)))
    });
    let result = v.iter().map(|(i, _, _)| i).join(" ");
    println!("{}", result);
}
