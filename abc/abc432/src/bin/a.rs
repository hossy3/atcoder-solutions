use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }
    let mut v = vec![a, b, c];
    v.sort();
    v.reverse();
    let result = v.iter().join("");
    println!("{result}");
}
