use proconio::input;

fn main() {
    input! {
        n: usize,
        mut td: [(usize, usize); n],
    }

    const MAX: usize = 1_000_000;

    td.sort_by_key(|&x| (x.1, x.0));
    let mut dp = Vec::with_capacity(n + 1);
    dp.push(vec![MAX; n + 1]); // time
    dp[0][0] = 0;

    for (i, &(t, d)) in td.iter().enumerate() {
        dp.push(dp[i].clone());
        for j in 0..=i {
            let t = t + dp[i][j];
            if t <= d {
                dp[i + 1][j + 1] = dp[i + 1][j + 1].min(t);
            }
        }
    }

    let result = (0..=n).position(|i| dp[n][i] == MAX).unwrap_or(n + 1) - 1;
    println!("{}", result);
}
