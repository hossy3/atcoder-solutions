use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }

    let mut dp = vec![vec![usize::MAX; n + 1]; n + 1]; // 上から順に 白が左から n 個詰まっているときの操作最小値
    dp[0].fill(0);
    for (i, s) in s.iter().enumerate() {
        let mut count = s.iter().filter(|&&c| c == '#').count(); // 操作数 まずすべて白にする
        dp[i + 1][n] = dp[i][n] + count;
        for j in (0..n).rev() {
            if s[j] == '#' {
                count -= 1; // もともと黒だったので操作しなくてよくなる
            } else {
                count += 1;
            }
            for k in j..=n {
                // if dp[i + 1][j] != 5 && dp[i + 1][j].min(dp[i][k] + count) == 5 {
                //     eprintln!("{} {} {} {} {}", i, j, k, dp[i][k], count);
                // }
                dp[i + 1][j] = dp[i + 1][j].min(dp[i][k] + count);
            }
        }
    }

    let result = *dp[n].iter().min().unwrap();
    println!("{result}");
}
