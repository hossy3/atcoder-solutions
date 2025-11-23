use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [[usize; n]; n],
    }

    let mut v = vec![1usize; 2 * n - 1];
    for i in (0..(2 * n - 2)).rev() {
        v[i] = (v[i + 1] * 10) % m;
    }
    for i in 0..n {
        for j in 0..n {
            a[i][j] = (a[i][j] * v[i + j]) % m;
        }
    }

    // 左上から中間の対角線までのパターン
    let mut dp0 = vec![vec![vec![]; n]; n];
    dp0[0][0].push(a[0][0]);
    for i in 0..(n - 1) {
        for j in 0..=i {
            let i = i - j;
            let dp = dp0[i][j].clone();
            for x in dp {
                dp0[i + 1][j].push((x + a[i + 1][j]) % m);
                dp0[i][j + 1].push((x + a[i][j + 1]) % m);
            }
        }
    }

    // 右下から中間の対角線までのパターン
    let mut dp1 = vec![vec![vec![]; n]; n];
    dp1[n - 1][n - 1].push(0usize);
    for i in (1..n).rev() {
        for j in (i..n).rev() {
            let i = n + i - j - 1;
            eprintln!("1: i: {i}, j: {j}");
            let dp = dp1[i][j].clone();
            for x in dp {
                dp1[i - 1][j].push((x + a[i][j]) % m);
                dp1[i][j - 1].push((x + a[i][j]) % m);
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

    let mut result = 0usize;
    for i in 0..n {
        let j = n - i - 1;
        for &x in &dp0[i][j] {
            if let Some(&y) = dp2[j].range(..(m - x)).last() {
                result = result.max((x + y) % m);
            } else if let Some(&y) = dp2[j].last() {
                result = result.max((x + y) % m);
            }
        }
    }
    println!("{result}");
}
