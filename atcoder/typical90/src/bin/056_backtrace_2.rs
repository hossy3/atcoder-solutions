use std::collections::VecDeque;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        ab: [(usize, usize); n],
    }

    let mut dp = vec![vec![None; s + 1]; n + 1];
    for i in 0..n {
        for j in 0..s {
            if dp[i][j].is_none() && !(i == 0 && j == 0) {
                continue;
            }
            let (a, b) = ab[i];
            if j + a <= s {
                dp[i + 1][j + a] = Some(('A', (i, j)));
            }
            if j + b <= s {
                dp[i + 1][j + b] = Some(('B', (i, j)));
            }
        }
    }

    if dp[n][s].is_none() {
        println!("Impossible");
        return;
    }

    let mut result = VecDeque::new();
    let mut i = n;
    let mut j = s;
    while let Some((c, (i0, j0))) = dp[i][j] {
        result.push_front(c);
        (i, j) = (i0, j0);
    }
    println!("{}", result.iter().join(""));
}
