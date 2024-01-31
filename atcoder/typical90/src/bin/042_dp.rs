use proconio::input;

type Mint = ac_library::ModInt1000000007;

fn f(k: usize) -> u32 {
    if k % 9 != 0 {
        return 0;
    }
    let mut dp = vec![Mint::new(0); k + 1];
    dp[0] = Mint::new(1);
    for i in 0..k {
        for j in (i + 1)..=(i + 9).min(k) {
            let x = dp[i];
            dp[j] += x;
        }
    }
    dp[k].val()
}

fn main() {
    input! {
        k: usize,
    }
    let result = f(k);
    println!("{result}");
}
