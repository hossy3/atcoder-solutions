use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut count = 0;
    let mut dp = vec![vec![0usize; n]; 3];
    for l in 0..(n - 1) {
        dp[1][l] = if a[l] == a[l + 1] { 0 } else { 1 };
        count += dp[1][l];
    }
    for w in 2..n {
        dp[2] = vec![0usize; n - w];
        for l in 0..(n - w) {
            dp[2][l] = dp[0][l + 1] + if a[l] == a[l + w] { 0 } else { 1 };
            count += dp[2][l];
        }
        dp.swap(0, 1);
        dp.swap(1, 2);
    }
    println!("{}", count);
}
