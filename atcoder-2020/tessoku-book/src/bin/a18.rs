use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        a: [usize; n],
    }

    let mut dp = Vec::with_capacity(n + 1);
    dp.push(vec![false; s + 1]);
    dp[0][0] = true;
    for i in 0..n {
        dp.push(dp.last().unwrap().clone());

        let a = a[i];
        if s < a {
            continue;
        }
        for j in 0..=(s - a) {
            if dp[i][j] {
                dp[i + 1][j + a] = true;
            }
        }
    }

    let yes = dp[n][s];
    println!("{}", if yes { "Yes" } else { "No" });
}
