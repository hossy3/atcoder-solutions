use std::collections::VecDeque;

use itertools::Itertools;
use proconio::input;

fn f(i: usize, j: usize, dp: &[(usize, i64)], h: &[i64]) -> i64 {
    dp[i].1 + (h[j] - h[i]).abs()
}

fn main() {
    input! {
        n: usize,
        h: [i64; n],
    }

    let mut dp = vec![(0, 0); n];
    dp[1] = (0, f(0, 1, &dp, &h));

    for i in 2..n {
        let a = f(i - 2, i, &dp, &h);
        let b = f(i - 1, i, &dp, &h);
        dp[i] = if a < b { (i - 2, a) } else { (i - 1, b) };
    }

    let mut v = VecDeque::with_capacity(n);
    let mut i = n - 1;
    v.push_front(i + 1);
    while i > 0 {
        i = dp[i].0;
        v.push_front(i + 1);
    }

    let result = v.iter().join(" ");
    println!("{}", v.len());
    println!("{}", result);
}
