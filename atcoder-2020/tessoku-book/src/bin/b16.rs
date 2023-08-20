use proconio::input;

fn f(i: usize, j: usize, dp: &[i64], h: &[i64]) -> i64 {
    dp[i] + (h[j] - h[i]).abs()
}

fn main() {
    input! {
        n: usize,
        h: [i64; n],
    }

    let mut dp = vec![0; n];
    dp[1] = f(0, 1, &dp, &h);

    for i in 2..n {
        dp[i] = f(i - 2, i, &dp, &h).min(f(i - 1, i, &dp, &h));
    }
    let result = dp[n - 1];
    println!("{}", result);
}
