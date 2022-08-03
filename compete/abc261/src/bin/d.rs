// -*- coding:utf-8-unix -*-

use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        x: [usize; n],
        cy: [(usize, usize); m]
    }

    let mut h = HashMap::<usize, usize>::new(); // cy
    for (c, y) in cy {
        h.insert(c, y);
    }

    let mut dp = vec![vec![0_usize; n + 1]; n + 1];
    dp[0][0] = 0;
    dp[0][1] = x[0];
    if let Some(y) = h.get(&1) {
        dp[0][1] += y;
    }
    for i in 1..n {
        for j in 0..=i {
            dp[i][0] = dp[i][0].max(dp[i - 1][j]);
        }
        for j in 1..=(i + 1) {
            dp[i][j] = dp[i - 1][j - 1] + x[i];
            if let Some(y) = h.get(&j) {
                dp[i][j] += y;
            }
        }
    }

    let mut score = 0;
    for i in 0..=n {
        score = score.max(dp[n - 1][i]);
    }
    println!("{}", score);
}
