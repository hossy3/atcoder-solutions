use proconio::input;

type Mint = ac_library::ModInt998244353;

fn main() {
    input! {
        (n, k0): (usize, usize),
    }

    let mint0 = Mint::new(0);
    let mint1 = Mint::new(1);

    let mut dp = vec![vec![]; k0 + 1]; // dp[h][m]: 長さ m の、条件を満たし、かつすべて a[i] >= h を満たすものの個数
    dp[k0] = vec![mint0, mint1]; // 条件を満たし、かつすべて a[i] >= k0 を満たすもの: 長さ 1 は a = [k0] の 1通り
    for i in (0..k0).rev() {
        let j_max = if i == 0 { n } else { n.min(k0 / i) }; // 面積が h を超えない範囲
        dp[i].resize(j_max + 1, mint0);
        for j in 1..=j_max {
            let mut x = mint0;
            let k_max = (j + 1).min(dp[i + 1].len());
            for k in 1..=k_max {
                let x0 = if k > 1 { dp[i + 1][k - 1] } else { mint1 };
                let x1 = if k < j { dp[i][j - k] } else { mint1 };
                x += x0 * x1;
            }
            dp[i][j] = x;
        }
    }

    let result = dp[0][n];
    println!("{result}");
}
