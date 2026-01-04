use std::cmp::Reverse;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
    }

    // 並び替える
    let ab: Vec<_> = ab
        .iter()
        .sorted_by_key(|(a, b)| (a, Reverse(b)))
        .cloned()
        .collect();

    // DP
    let mut state = vec![]; // 長さ0
    for &(_, b) in &ab {
        let i = state.partition_point(|&x| x < b);
        if i == state.len() {
            state.push(b);
        } else {
            state[i] = b;
        }
    }

    let result = state.len();
    println!("{result}");
}
