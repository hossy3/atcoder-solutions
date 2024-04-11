use ac_library::convolution;
use proconio::input;

type Mint = ac_library::ModInt998244353;

// c[0] + c[1] * x + ... + c[n - 1] * x^(n - 1) * p(x) == 1 (mod x^L) を満たす、l - 1 次以下の多項式を求める
// c[0] == 1
fn polynomial_inverse(c: &[Mint], l: usize) -> Vec<Mint> {
    let n = c.len();
    let mut v = vec![Mint::new(1), Mint::new(0)];
    let k_max = (l * 2 - 1).ilog2();
    for k in 0..k_max {
        let k0 = 1 << k;
        v.resize(2 * k0, Mint::new(0));
        let p = convolution(&v, &c[..(n.min(2 * k0))]);
        let mut q = vec![Mint::new(0); 2 * k0];
        q[0] = Mint::new(1);
        for j in k0..(2 * k0) {
            q[j] = -p[j];
        }
        v = convolution(&v, &q);
    }

    v.resize(l, Mint::new(0));
    v
}

fn main() {
    input! {
        (n, k): (usize, usize),
    }

    let mut dp = vec![vec![]; k + 1]; // 条件を満たし、かつすべて a[i] >= k0 を満たすもの: 長さ 1 は a = [k0] の 1通り
    dp[k] = vec![Mint::new(1); 3];
    for i in (0..k).rev() {
        let k_max = if i == 0 { n } else { n.min(k / i) }; // 面積が h を超えない範囲
        let j_max = dp[i + 1].len();
        let mut c = vec![Mint::new(0); j_max];
        c[0] = Mint::new(1);
        for j in 1..j_max {
            c[j] = -dp[i + 1][j];
        }
        dp[i] = polynomial_inverse(&c, k_max + 2);
    }

    let result = dp[0][n + 1];
    println!("{result}");
}
