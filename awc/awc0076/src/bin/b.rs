use std::cmp::Reverse;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
    }
    let results = ab
        .iter()
        .enumerate()
        .sorted_unstable_by_key(|&(i, &(a, b))| (Reverse(b), Reverse(a), i))
        .map(|(i, _)| i + 1)
        .collect::<Vec<_>>();
    for result in results {
        println!("{result}");
    }
}
