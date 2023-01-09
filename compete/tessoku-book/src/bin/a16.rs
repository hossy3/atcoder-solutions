use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n - 1],
        b: [usize; n - 2],
    }

    let mut dp = vec![0; n];
    dp[1] = a[0];
    for i in 0..(n - 2) {
        dp[i + 2] = (dp[i] + b[i]).min(dp[i + 1] + a[i + 1]);
    }
    let result = dp[n - 1];
    println!("{}", result);
}
