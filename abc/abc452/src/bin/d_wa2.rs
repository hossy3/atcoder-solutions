use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let mut dp = vec![vec![0usize; t.len() + 1]; s.len() + 1];
    for (i, &s0) in s.iter().enumerate() {
        // 既存文字あり: 採用しない場合
        dp[i + 1] = dp[i].clone();

        // 既存文字あり: 採用する場合
        for (j, &t0) in t.iter().enumerate() {
            if s0 == t0 {
                dp[i + 1][j + 1] += dp[i][j];
            } else {
                dp[i + 1][j] += dp[i][j];
            }
        }

        // 既存文字なし: 採用する場合
        if s0 == s[0] {
            dp[i + 1][1] += 1;
        } else {
            dp[i + 1][0] += 1;
        }
    }
    eprintln!("{:?}", &dp);

    let result = dp[s.len()][..t.len()].iter().sum::<usize>();
    println!("{result}");
}
