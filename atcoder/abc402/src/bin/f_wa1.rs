use std::collections::{BTreeSet, HashSet};

use proconio::input;

fn pow10(k: usize, m: u128) -> u128 {
    let mut x = 1u128;
    for _ in 0..k {
        x *= 10;
        x %= m;
    }
    x
}

fn main() {
    input! {
        n: usize,
        m: u128,
        a: [[u128; n]; n],
    }

    // 左上から中間の対角線までのパターン
    let mut dp0 = vec![vec![HashSet::new(); n]; n];
    dp0[0][0].insert(a[0][0]);
    for i in 0..(n - 1) {
        for j in 0..=i {
            let i = i - j;
            let dp = dp0[i][j].clone();
            for x in dp {
                dp0[i + 1][j].insert((x * 10 + a[i + 1][j]) % m);
                dp0[i][j + 1].insert((x * 10 + a[i][j + 1]) % m);
            }
        }
    }

    // 右下から中間の対角線までのパターン
    let mut dp1 = vec![vec![HashSet::new(); n]; n];
    dp1[n - 1][n - 1].insert(0u128);
    for i in (1..n).rev() {
        for j in (i..n).rev() {
            let i = n + i - j - 1;
            let k = pow10(2 * n - i - j - 2, m);
            let dp = dp1[i][j].clone();
            for x in dp {
                dp1[i - 1][j].insert((x + a[i][j] * k) % m);
                dp1[i][j - 1].insert((x + a[i][j] * k) % m);
            }
        }
    }

    // BTreeMap にする
    let mut dp2 = vec![BTreeSet::new(); n];
    for i in 0..n {
        let j = n - i - 1;
        for &x in &dp1[i][j] {
            dp2[j].insert(x);
        }
    }

    let mut result = 0u128;
    let k = pow10(n - 1, m);
    for i in 0..n {
        let j = n - i - 1;
        for &x in &dp0[i][j] {
            let x = (k * x) % m;
            if let Some(&y) = dp2[j].range(..=(m - x)).last() {
                result = result.max((x + y) % m);
            } else if let Some(&y) = dp2[j].last() {
                result = result.max((x + y) % m);
            }
        }
    }
    println!("{result}");
}
