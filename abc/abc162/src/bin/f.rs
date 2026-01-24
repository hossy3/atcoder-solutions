use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [isize; n],
    }

    let k = if n % 2 == 0 { 1 } else { 2 }; // 選べるときにも選ばなくて良い残り数
    let mut dp = vec![vec![vec![isize::MIN; 2]; k + 1]; n + 1]; // [場所][k][次選べる 0, 選べない 1]
    dp[0][k][0] = 0;
    for i in 0..n {
        for k in 0..=k {
            if dp[i][k][0] != isize::MIN {
                dp[i + 1][k][1] = dp[i + 1][k][1].max(dp[i][k][0] + a[i]);
                if k > 0 {
                    dp[i + 1][k - 1][0] = dp[i + 1][k - 1][0].max(dp[i][k][0]);
                }
            }
            if dp[i][k][1] != isize::MIN {
                dp[i + 1][k][0] = dp[i + 1][k][0].max(dp[i][k][1]);
            }
        }
    }
    let result = dp[n][0][0].max(dp[n][0][1]).max(dp[n][1][0]); // xoxox パターンで [1][0] を使う
    println!("{result}");
}
