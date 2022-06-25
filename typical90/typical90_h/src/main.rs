// -*- coding:utf-8-unix -*-

use proconio::{input, marker::Chars};

// 008 - AtCounter（★4）
// https://atcoder.jp/contests/typical90/tasks/typical90_h

fn main() {
    input! {
        n: usize,
        s: Chars,
    };
    let m = 1_000_000_007;
    let mut dp = vec![vec![0i64; n]; 7];

    for j in 0..n {
        if s[j] == 'a' {
            dp[0][j] = 1;
        }
    }

    let chars = ['a', 't', 'c', 'o', 'd', 'e', 'r'];
    for i in 1..7 {
        let mut sum = 0;
        let c = chars[i];
        for j in 1..n {
            sum += dp[i - 1][j - 1];
            sum %= m;
            if s[j] == c {
                dp[i][j] = sum;
            }
        }
    }

    let score = dp[6].iter().fold(0, |sum, x| (sum + x) % m);
    println!("{}", score);
}
