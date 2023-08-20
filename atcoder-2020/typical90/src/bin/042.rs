use proconio::input;

const MOD: usize = 1_000_000_007;

fn main() {
    input! {
        k: usize,
    }
    if k % 9 != 0 {
        println!("{}", 0);
        return;
    }
    let mut dp = vec![0usize; k + 1 + 8];
    dp[0] = 1;
    for i in 0..k {
        for j in 1..=9 {
            dp[i + j] = (dp[i + j] + dp[i]) % MOD;
        }
    }
    println!("{}", dp[k]);
}
