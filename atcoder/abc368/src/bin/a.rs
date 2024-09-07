use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    let i = n - k;
    let mut v0: Vec<_> = a[i..].iter().collect();
    let mut v1: Vec<_> = a[0..i].iter().collect();
    v0.append(&mut v1);
    let result = v0.iter().join(" ");
    println!("{result}");
}
