use std::collections::BTreeSet;

use proconio::input;

// DP 復元

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); n],
    }

    let mut dp = vec![vec![0usize; m + 1]];
    for (i, &(a, b)) in ab.iter().enumerate() {
        dp.push(dp[i].clone());
        for w in (0..=(m - a)).rev() {
            dp[i + 1][w + a] = dp[i + 1][w + a].max(dp[i][w] + b);
        }
    }

    let mut cand = BTreeSet::new();
    for w in (0..=m).rev() {
        if dp[n][w] > 0 {
            cand.insert((n, w));
            break;
        }
    }

    let mut results = BTreeSet::new();
    while let Some((i, w)) = cand.pop_last() {
        if i > 0 && dp[i - 1][w] == dp[i][w] {
            cand.insert((i - 1, w));
        }
        for j in 0..i {
            let (a, b) = ab[j];
            if w >= a && dp[i][w] == dp[j][w - a] + b {
                results.insert(j);
                cand.insert((j, w - a));
            }
        }
    }

    for w in 0..n {
        let yes = results.contains(&w);
        println!("{}", if yes { "Yes" } else { "No" });
    }
}
