use std::iter;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        a: [Chars; n],
        b: [Chars; n],
    }
    let i = iter::zip(&a, &b).position(|(a, b)| a != b).unwrap();
    let j = iter::zip(&a[i], &b[i]).position(|(a, b)| a != b).unwrap();
    println!("{} {}", i + 1, j + 1);
}
