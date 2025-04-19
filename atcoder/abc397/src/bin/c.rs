use std::collections::HashSet;

use itertools::Itertools;
use proconio::input;

fn f(a: &[usize]) -> Vec<usize> {
    let mut set = HashSet::new();
    let mut result = Vec::with_capacity(a.len() + 1);
    result.push(0);
    for x in a {
        set.insert(x);
        result.push(set.len());
    }
    result
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let v0 = f(&a);
    let v1 = f(&a.iter().map(|&x| x).rev().collect_vec());
    let Some(result) = (0..=n).map(|i| v0[i] + v1[n - i]).max() else {
        unreachable!()
    };
    println!("{result}");
}
