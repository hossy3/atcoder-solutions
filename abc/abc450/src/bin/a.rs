use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    println!("{}", (1..=n).rev().join(","));
}
