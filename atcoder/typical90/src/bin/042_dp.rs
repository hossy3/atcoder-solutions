use proconio::input;

type Mint = ac_library::ModInt1000000007;

fn f(k: usize) -> Mint {
    if k % 9 != 0 {
        return Mint::new(0);
    }
    let mut dp = vec![Mint::new(0); k + 1];
    dp[0] = Mint::new(1);
    for i in 0..k {
        let x = dp[i];
        for j in (i + 1)..=(i + 9).min(k) {
            dp[j] += x;
        }
    }
    dp[k]
}

fn main() {
    input! {
        k: usize,
    }
    let result = f(k);
    println!("{result}");
}
