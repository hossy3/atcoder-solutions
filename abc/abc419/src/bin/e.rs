use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        l: usize,
        a: [usize; n],
    }

    let mut v = vec![vec![0; m]; l]; // 長さL 何個足すとその数にできるか
    for i in 0..n {
        let a = a[i] % m;
        for j in 0..m {
            v[i % l][j] += (m - a + j) % m;
        }
    }

    let mut dp = vec![vec![usize::MAX / 4; m]; l + 1];
    dp[0][0] = 0;
    for i in 0..l {
        for j in 0..m {
            for k in 0..m {
                dp[i + 1][(j + k) % m] = dp[i + 1][(j + k) % m].min(dp[i][j] + v[i][k]);
            }
        }
    }

    let result = dp[l][0];
    println!("{result}");
}
