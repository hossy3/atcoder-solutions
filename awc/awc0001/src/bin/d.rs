use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        ab: [(usize, usize); n],
    }

    let mut dp = vec![vec![0usize; m + 1]; n]; // dp[マス][滞在費] = 利益
    for (i, &(a, b)) in ab.iter().enumerate() {
        if b > m {
            continue;
        }
        dp[i][b] = dp[i][b].max(a);
        for j in i.saturating_sub(k)..i {
            for k in b..=m {
                dp[i][k] = dp[i][k].max(dp[j][k - b] + a);
            }
        }
    }
    // eprintln!("{:?}", &dp);

    let result = dp.iter().map(|v| v.iter().max().unwrap()).max().unwrap();
    println!("{result}");
}
