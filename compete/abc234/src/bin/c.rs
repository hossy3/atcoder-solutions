use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        mut k: usize,
    }
    let mut v = vec![];
    while k > 0 {
        v.push((k % 2) * 2);
        k /= 2;
    }
    let result = v.iter().rev().join("");
    println!("{}", result);
}
