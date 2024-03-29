use itertools::Itertools;
use proconio::{input, marker::Usize1};

// DP on trees

fn main() {
    input! {
        n: usize,
        a: [Usize1; n - 1],
    }

    let mut dp = vec![0; n];
    for i in (1..n).rev() {
        dp[a[i - 1]] += dp[i] + 1;
    }
    let result = dp.iter().join(" ");
    println!("{}", result);
}
