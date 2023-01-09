use proconio::input;

// bit DP

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [[usize; n]; m],
    }

    const MAX: i64 = 1_000_000_000;

    let mut v = Vec::with_capacity(m);
    for a in &a {
        let mut x = 0;
        for (i, a) in a.iter().enumerate() {
            if *a == 1 {
                x += 1 << i;
            }
        }
        v.push(x);
    }

    let mut dp = Vec::with_capacity(m + 1);
    let w = 1 << n;
    dp.push(vec![MAX; w]);
    dp[0][0] = 0;
    for (i, x) in v.iter().enumerate() {
        dp.push(dp[i].clone());
        for j in 0..w {
            dp[i + 1][j | x] = dp[i + 1][j | x].min(dp[i][j] + 1);
        }
    }

    let mut result = *dp[m].last().unwrap();
    if result == MAX {
        result = -1;
    }
    println!("{}", result);
}
