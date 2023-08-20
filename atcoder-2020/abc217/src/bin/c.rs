use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
    }
    let mut v = vec![0; n];
    for (i, &x) in p.iter().enumerate() {
        v[x - 1] = i + 1;
    }
    println!("{}", v.iter().join(" "));
}
