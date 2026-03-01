use fixedbitset::FixedBitSet;
use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize; w]; h],
        b: [[usize; w]; h],
    }

    // a, b がそれぞれ 80 以下なので、各マスの状態数は 80*距離 以内
    // そのため "80*(h+w)*2+1" 状態の DPを解けばよい
    let mid = 80 * (h + w);
    let mut dp = vec![vec![FixedBitSet::with_capacity(mid * 2 + 1); w]; h];
    dp[0][0].insert(mid + a[0][0] - b[0][0]);
    dp[0][0].insert(mid - a[0][0] + b[0][0]);

    for i in 0..h {
        for j in 0..w {
            if i + 1 < h {
                for k in 0..=(mid * 2) {
                    if dp[i][j].contains(k) {
                        dp[i + 1][j].insert(k + a[i + 1][j] - b[i + 1][j]);
                        dp[i + 1][j].insert(k - a[i + 1][j] + b[i + 1][j]);
                    }
                }
            }
            if j + 1 < w {
                for k in 0..=(mid * 2) {
                    if dp[i][j].contains(k) {
                        dp[i][j + 1].insert(k + a[i][j + 1] - b[i][j + 1]);
                        dp[i][j + 1].insert(k - a[i][j + 1] + b[i][j + 1]);
                    }
                }
            }
        }
    }

    let result = (0..=(mid * 2))
        .filter(|&i| dp[h - 1][w - 1].contains(i))
        .map(|i| i.abs_diff(mid))
        .min()
        .unwrap();
    println!("{result}");
}
