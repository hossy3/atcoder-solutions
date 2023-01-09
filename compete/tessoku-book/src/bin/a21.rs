use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        pa: [(Usize1, usize); n],
    }

    let mut dp = vec![vec![0; n + 1]; n + 1]; // dp[#left][#right]
    for l in 0..n {
        for r in 0..(n - l) {
            // remove left
            let k = l;
            if l < pa[k].0 && pa[k].0 <= n - r - 1 {
                dp[l + 1][r] = dp[l + 1][r].max(dp[l][r] + pa[k].1);
            } else {
                dp[l + 1][r] = dp[l + 1][r].max(dp[l][r]);
            }

            // remove right
            let k = n - r - 1;
            if l <= pa[k].0 && pa[k].0 < n - r - 1 {
                dp[l][r + 1] = dp[l][r + 1].max(dp[l][r] + pa[k].1);
            } else {
                dp[l][r + 1] = dp[l][r + 1].max(dp[l][r]);
            }
        }
    }

    let mut result = 0;
    for i in 0..=n {
        result = result.max(dp[i][n - i]);
    }
    println!("{}", result);
}
