use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut dp = vec![(0usize, 0usize, 0usize); n + 2]; // (はみ出しなし, 上はみ出し, 下はみ出し)
    dp[0] = (1, 0, 0);
    for i in 0..n {
        dp[i + 1].0 += dp[i].0; // 縦置き
        dp[i + 1].1 += dp[i].0; // L
        dp[i + 1].2 += dp[i].0; // L
        dp[i + 2].0 += dp[i].0; // 横置き2個

        dp[i + 1].2 += dp[i].1; // 横置き
        dp[i + 2].0 += dp[i].1; // L

        dp[i + 1].1 += dp[i].2; // 横置き
        dp[i + 2].0 += dp[i].2; // L
    }

    let result = dp[n].0;
    println!("{result}");
}
