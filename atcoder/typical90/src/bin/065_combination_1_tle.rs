use proconio::input;

type Mint = ac_library::ModInt998244353;

fn combination(n: usize, r: usize, fact: &[Mint], fact_inv: &[Mint]) -> Mint {
    fact[n] * fact_inv[r] * fact_inv[n - r]
}

fn f((r, g, b, k): (usize, usize, usize, usize), (x, y, z): (usize, usize, usize)) -> u32 {
    let b0 = k - x; // 青色のボールを b0 個以上選ぶ
    let r0 = k - y;
    let g0 = k - z;
    if r0 > r || g0 > g || b0 > b || r0 + g0 + b0 > k {
        return 0;
    }

    let Some(&n) = [r, g, b, k].iter().max() else { unreachable!() };

    let mut fact = vec![Mint::new(1); n + 1];
    for i in 2..=n {
        fact[i] = fact[i - 1] * i;
    }

    let mut fact_inv = vec![Mint::new(1); n + 1];
    for i in 0..=n {
        fact_inv[i] = fact[i].inv();
    }

    let mut result = Mint::new(0);
    for r1 in r0..=r.min(k - (g0 + b0)) {
        let x = combination(r, r1, &fact, &fact_inv);
        for g1 in g0..=g.min(k - (r1 + b0)) {
            let b1 = k - (r1 + g1);
            if b1 <= b {
                result +=
                    x * combination(g, g1, &fact, &fact_inv) * combination(b, b1, &fact, &fact_inv);
            }
        }
    }

    result.val()
}

fn main() {
    input! {
        (r, g, b, k): (usize, usize, usize, usize),
        (x, y, z): (usize, usize, usize),
    }
    let result = f((r, g, b, k), (x, y, z));
    println!("{result}");
}
