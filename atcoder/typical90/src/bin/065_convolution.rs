use ac_library::convolution;
use proconio::input;

type Mint = ac_library::ModInt998244353;

fn combination(n: usize, r: usize, fact: &[Mint], fact_inv: &[Mint]) -> Mint {
    fact[n] * fact_inv[r] * fact_inv[n - r]
}

fn build_fact(n: usize) -> Vec<Mint> {
    let mut fact = vec![Mint::new(1); n + 1];
    for i in 2..=n {
        fact[i] = fact[i - 1] * i;
    }
    fact
}

fn build_fact_inv(fact: &[Mint]) -> Vec<Mint> {
    let n = fact.len() - 1;
    let mut fact_inv = vec![Mint::new(1); n + 1];
    for i in 0..=n {
        fact_inv[i] = fact[i].inv();
    }
    fact_inv
}

fn f((r, g, b, k): (usize, usize, usize, usize), (x, y, z): (usize, usize, usize)) -> u32 {
    let r0 = k - y; // 赤色のボールを r0 個以上選ぶ
    let g0 = k - z;
    let b0 = k - x;
    if r0 > r || g0 > g || b0 > b || r0 + g0 + b0 > k {
        return 0;
    }

    let r1 = r.min(k - (g0 + b0)); // 赤色のボールを r1 個以下選ぶ
    let g1 = g.min(k - (b0 + r0));
    let b1 = b.min(k - (r0 + g0));

    let Some(&n) = [r, g, b, k].iter().max() else { unreachable!() };
    let fact = build_fact(n);
    let fact_inv = build_fact_inv(&fact);

    let mut rs = vec![Mint::new(0); r1 - r0 + 1];
    let mut gs = vec![Mint::new(0); g1 - g0 + 1];
    let mut bs = vec![Mint::new(0); b1 - b0 + 1];

    for i in 0..=(r1 - r0) {
        rs[i] = combination(r, r0 + i, &fact, &fact_inv);
    }
    for i in 0..=(g1 - g0) {
        gs[i] = combination(g, g0 + i, &fact, &fact_inv);
    }
    for i in 0..=(b1 - b0) {
        bs[i] = combination(b, b0 + i, &fact, &fact_inv);
    }

    let rgs = convolution(&rs, &gs);

    let mut result = Mint::new(0);
    for ib in 0..=(b1 - b0) {
        let i = k - (r0 + g0 + b0 + ib);
        result += rgs[i] * bs[ib];
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
