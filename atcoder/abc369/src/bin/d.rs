use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let mut dp = vec![(0i64, i64::MIN); n + 1]; // 偶数回倒した, 奇数回倒した
    for (i, &x) in a.iter().enumerate() {
        let (x0, x1) = dp[i];
        dp[i + 1] = (x0.max(x1 + x * 2), x1.max(x0 + x));
    }
    let (x0, x1) = dp[n];
    let result = x0.max(x1);
    println!("{result}");
}
