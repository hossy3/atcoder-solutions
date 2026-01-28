use proconio::input;

type Mint = ac_library::ModInt998244353;

fn main() {
    input! {
        n: usize,
        s: usize,
        a: [usize; n],
    }

    // knapsack DP を変わった感じで実装する
    let mut dp = vec![vec![None; s + 1]; n + 1];
    dp[0][0] = Some(Mint::new(1));
    for (i, &a) in a.iter().enumerate() {
        dp[i + 1] = dp[i].clone();
        dp[i + 1][0] = Some(Mint::new(i + 2)); // l は自身より左のどこを選んでも良い
        if a == s {
            if let Some(x) = dp[i][0] {
                if let Some(y) = dp[i + 1][s] {
                    dp[i + 1][s] = Some(x * (n - i) + y);
                } else {
                    dp[i + 1][s] = Some(x * (n - i));
                }
            }
        } else if a < s {
            for j in 0..=(s - a) {
                if let Some(x) = dp[i][j] {
                    let j = j + a;
                    let k = if j == s { n - i } else { 1 }; // r は自身より右のどこを選んでも良い
                    if let Some(y) = dp[i + 1][j] {
                        dp[i + 1][j] = Some(x * k + y);
                    } else {
                        dp[i + 1][j] = Some(x * k);
                    }
                }
            }
        }
    }

    let result = dp[n][s].unwrap_or(Mint::new(0));
    println!("{result}");
}
