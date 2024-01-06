use proconio::input;

type Mint = ac_library::ModInt998244353;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut dp = vec![Mint::new(0); n];
    dp[0] = Mint::new(1);

    const A: usize = 100;
    let mut acc = vec![vec![Mint::new(0); A]; A];
    for (i, &x) in a.iter().enumerate() {
        for j in 1..A {
            dp[i] += acc[j][i % j];
        }
        let cur = dp[i];
        if cur.val() == 0 {
            continue;
        }
        if x < A {
            acc[x][i % x] += cur;
        } else {
            for j in ((i + x)..n).step_by(x) {
                dp[j] += cur;
            }
        }
    }

    let result: Mint = dp.iter().sum();
    println!("{result}");
}
