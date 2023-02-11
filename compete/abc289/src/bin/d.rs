use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        m: usize,
        b: [usize; m],
        x: usize,
    }

    let mut s = HashSet::new();
    for i in b {
        s.insert(i);
    }

    let mut dp = vec![false; x + 1];
    dp[0] = true;
    for i in 0..x {
        if !dp[i] || s.contains(&i) {
            continue;
        }
        for &j in &a {
            let j = i + j;
            if j <= x {
                dp[j] = true;
            }
        }
    }

    let yes = dp[x];
    println!("{}", if yes { "Yes" } else { "No" });
}
