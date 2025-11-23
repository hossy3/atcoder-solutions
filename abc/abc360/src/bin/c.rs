use std::iter;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
        w: [usize; n],
    }

    let mut v = vec![vec![]; n];
    for (a, w) in iter::zip(a, w).into_iter() {
        v[a].push(w);
    }

    let mut result = 0usize;
    for v in v.iter_mut() {
        v.sort();
        v.pop();
        result += v.iter().sum::<usize>();
    }
    println!("{result}");
}
