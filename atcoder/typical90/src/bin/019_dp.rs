use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; 2 * n],
    }

    let mut dp = vec![vec![0; 2 * n + 1]; 2 * n]; // dp[l][r] 半開区間;
    for i in 1..=n {
        for l in 0..(2 * n) {
            let r = l + 2 * i;
            if r > 2 * n {
                continue;
            }
            let mut x = dp[l + 1][r - 1] + a[l].abs_diff(a[r - 1]);
            for k in 1..i {
                let m = l + k * 2; // 分割箇所
                x = x.min(dp[l][m] + dp[m][r]);
            }
            dp[l][r] = x;
        }
    }

    let result = dp[0][2 * n];
    println!("{result}");
}
