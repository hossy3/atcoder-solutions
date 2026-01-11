use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        t: [usize; n],
    }
    let v: Vec<_> = t
        .iter()
        .enumerate()
        .map(|(i, &t)| (t, i + 1))
        .sorted()
        .collect();
    println!("{}", v[..3].iter().map(|(_, i)| i).join(" "));
}
