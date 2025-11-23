use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [[usize; n]; n],
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
    dp1[n - 1][n - 1].insert(0usize);
    for i in (1..n).rev() {
        for j in (i..n).rev() {
            let i = n + i - j - 1;
            let k = 10usize.pow((n * 2 - i - j - 2) as u32);
            let dp = dp1[i][j].clone();
            for x in dp {
                dp1[i - 1][j].insert((x + a[i][j] * k) % m);
                dp1[i][j - 1].insert((x + a[i][j] * k) % m);
            }
        }
    }

    let mut result = 0usize;
    let k = 10usize.pow((n - 1) as u32);
    for i in 0..n {
        let j = n - i - 1;
        for &x in &dp0[i][j] {
            let x = (k * x) % m;
            for &y in &dp1[i][j] {
                result = result.max((x + y) % m);
            }
        }
    }
    println!("{result}");
}
