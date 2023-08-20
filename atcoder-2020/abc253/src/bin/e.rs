// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
    }

    if k == 0 {
        let mut result = 1;
        for _ in 0..n {
            result *= m as i64;
            result %= 998_244_353;
        }
        println!("{}", result);
        return;
    }

    let mut dp = vec![vec![0i64; m + 1]; n + 1];
    for i in 1..=m {
        dp[1][i] = 1;
    }

    for i in 1..n {
        let sum: i64 = dp[i].iter().sum();
        let mut sum2: i64 = dp[i][0..k].iter().sum();
        for j in 1..=m {
            if j >= k {
                sum2 -= dp[i][j - k];
            }
            if j + k - 1 <= m {
                sum2 += dp[i][j + k - 1];
            }
            dp[i + 1][j] = (sum - sum2) % 998_244_353;
        }
    }

    let sum = dp[n].iter().sum::<i64>() % 998_244_353;
    println!("{}", sum);
}
