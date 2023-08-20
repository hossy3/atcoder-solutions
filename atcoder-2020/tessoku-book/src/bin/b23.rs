use proconio::input;

// bit DP

fn main() {
    input! {
        n: usize,
        xy: [(f64, f64); n],
    }

    const MAX: f64 = 1e9;

    let w = 1 << n;
    let mut dp = vec![vec![MAX; n]; w]; // [set][pos]
    dp[0][0] = 0.0;
    for i in 0..w {
        for j in 0..n {
            if dp[i][j] == MAX {
                continue;
            }
            for j0 in 0..n {
                let i0 = i | (1 << j0);
                if i0 == i {
                    continue; // visited
                }
                let (x, y) = xy[j];
                let (x0, y0) = xy[j0];
                let len = ((x0 - x).powi(2) + (y0 - y).powi(2)).sqrt();
                dp[i0][j0] = dp[i0][j0].min(dp[i][j] + len);
            }
        }
    }

    let result = dp[w - 1][0];
    println!("{}", result);
}
