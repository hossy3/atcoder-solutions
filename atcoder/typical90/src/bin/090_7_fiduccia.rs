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

    v.truncate(l);
    v
}

// dp[1] を求める
fn f(n: usize, k: usize) -> Vec<Mint> {
    let mut dp = vec![vec![]; k + 1]; // 条件を満たし、かつすべて a[i] >= k0 を満たすもの: 長さ 1 は a = [k0] の 1通り
    dp[k] = vec![Mint::new(1); 3];
    for i in (1..k).rev() {
        let k_max = n.min(k / i); // 面積が h を超えない範囲
        let j_max = dp[i + 1].len();
        let mut c = vec![Mint::new(0); j_max];
        c[0] = Mint::new(1);
        for j in 1..j_max {
            c[j] = -dp[i + 1][j];
        }
        dp[i] = polynomial_inverse(&c, k_max + 2);
    }

    dp[1].clone()
}

// dp[1] から dp[0][n + 1] を高速に求める (a.k.a. kitamasa)
fn fiduccia(n: usize, k: usize, dp1: &[Mint]) -> Mint {
    let k = k.min(n);
    let mut v = vec![n + k + 1];
    while let Some(&x) = v.last() {
        if x < k + 1 {
            break;
        }
        v.push(x / 2);
    }
    v.reverse();
    let ref v = v;

    let mut cl = vec![Mint::new(0); k + 2];
    cl[0] = Mint::new(1);
    for i in 1..(k + 2) {
        cl[i] = -dp1[i];
    }

    let gl = polynomial_inverse(&cl, k + 2);
    cl.reverse();
    let ref cl = cl;

    let mut poly = vec![Mint::new(0); k + 1];
    poly[v[0]] = Mint::new(1); // v[0] < k + 1
    for &x in &v[1..] {
        poly = convolution(&poly, &poly);
        if x % 2 == 1 {
            poly.insert(0, Mint::new(0));
        } else {
            poly.push(Mint::new(0));
        }

        let mut p = poly[(k + 1)..].to_vec();
        poly.truncate(k + 1);
        assert_eq!(p.len(), k + 1);
        p.reverse();

        p = convolution(&p, &gl);
        p.truncate(k + 1);
        p.reverse();

        p = convolution(&p, &cl);
        p.truncate(k + 1);

        for i in 0..(k + 1) {
            poly[i] -= p[i];
        }
    }

    poly[k]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let n = 2;
        let k = 2;
        let dp1 = [1, 1, 2, 3].map(|x| Mint::new(x)).to_vec();
        let expect = Mint::new(8);

        assert_eq!(f(n, k), dp1);
        assert_eq!(fiduccia(n, k, &dp1), expect);
    }

    #[test]
    fn test_4() {
        let n = 28593;
        let k = 1;
        let dp1 = vec![Mint::new(1); 3];
        let expect = Mint::new(365728740);

        assert_eq!(f(n, k), dp1);
        assert_eq!(fiduccia(n, k, &dp1), expect);
    }
}

fn main() {
    input! {
        (n, k): (usize, usize),
    }

    let dp1 = f(n, k);
    let result = fiduccia(n, k, &dp1);
    println!("{result}");
}
