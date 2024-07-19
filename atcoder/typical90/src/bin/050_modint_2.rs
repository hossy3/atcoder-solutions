use proconio::input;

type Mint = ac_library::ModInt1000000007;

fn main() {
    input! {
        n: usize,
        l: usize,
    }
    let mut dp = vec![Mint::new(0); n + 1];
    dp[0] = Mint::new(1);
    for i in 1..=n {
        if i < l {
            dp[i] = dp[i - 1];
        } else {
            dp[i] = dp[i - 1] + dp[i - l];
        }
    }
    let result = dp[n];
    println!("{result}");
}
