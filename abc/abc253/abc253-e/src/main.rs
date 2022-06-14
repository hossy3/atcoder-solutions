// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: i64,
        m: i64,
        k: i64,
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

    let mut dp = vec![vec![0i64; (m + 1) as usize]; (n + 1) as usize];
    for i in 1..=m {
        dp[1][i as usize] = 1;
    }

    for i in 1..n {
        let sum = dp[i as usize].iter().fold(0, |sum, x| sum + x);
        let mut sum2 = dp[i as usize][0..(k as usize)]
            .iter()
            .fold(0, |sum, x| sum + x);
        for j in 1..=m {
            if (j - k) >= 0 {
                sum2 -= dp[i as usize][(j - k) as usize];
            }
            if (j + k - 1) <= m {
                sum2 += dp[i as usize][(j + k - 1) as usize];
            }
            dp[(i + 1) as usize][j as usize] = (sum - sum2) % 998_244_353;
        }
    }

    let sum = dp[n as usize].iter().fold(0, |sum, x| sum + x) % 998_244_353;
    println!("{}", sum);
}
