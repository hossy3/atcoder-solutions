use proconio::input;

fn main() {
    input! {
        n: usize,
        mut dcs: [(usize, usize, usize); n],
    }
    dcs.sort();

    const D: usize = 5000;
    let mut dp = vec![vec![0usize; D + 1]; n + 1];
    dp[0][0] = 0;

    for (i, &(d, c, s)) in dcs.iter().enumerate() {
        for j in 0..=D {
            dp[i + 1][j] = dp[i + 1][j].max(dp[i][j]);
            let j0 = j + c;
            if j0 <= d {
                dp[i + 1][j0] = dp[i + 1][j0].max(dp[i][j] + s);
            }
        }
    }

    let result = dp[n].iter().max().unwrap();
    println!("{result}");
}
