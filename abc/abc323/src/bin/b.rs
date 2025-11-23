use std::cmp::Reverse;

use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }
    let mut v = vec![];
    for (i, s) in s.iter().enumerate() {
        let count = s.iter().filter(|&&c| c == 'o').count();
        v.push((Reverse(count), i + 1));
    }
    v.sort();
    let result = v.iter().map(|(_, i)| i).join(" ");
    println!("{result}");
}
