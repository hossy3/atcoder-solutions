use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        ab: [(usize, usize); n],
    }

    let mut dp = vec![vec![vec![]; s + 1]; n + 1];
    dp[1][ab[0].0] = vec!['A'];
    dp[1][ab[0].1] = vec!['B'];
    for i in 1..n {
        for j in 0..s {
            if dp[i][j].len() == 0 {
                continue;
            };
            let (a, b) = ab[i];
            if j + a <= s {
                let mut path = dp[i][j].clone();
                path.push('A');
                dp[i + 1][j + a] = path;
            }
            if j + b <= s {
                let mut path = dp[i][j].clone();
                path.push('B');
                dp[i + 1][j + b] = path;
            }
        }
    }

    if dp[n][s].len() > 0 {
        println!("{}", dp[n][s].iter().join(""));
    } else {
        println!("Impossible");
    }
}
