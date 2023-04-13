use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize],
    }

    let mut dp = vec![0usize; n + 1];
    dp[1] = 1;
    for n in 2..=n {
        dp[n] = a
            .iter()
            .map(|&x| if n >= x { n - dp[n - x] } else { 0 })
            .max()
            .unwrap();
    }
    println!("{}", dp[n]);
}
