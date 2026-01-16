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

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
    }

    let fact = build_fact(n);
    let fact_inv = build_fact_inv(&fact);
    let mut result = Mint::new(0);

    for k in 0..=k {
        result += Mint::new(m)
            * Mint::new(m - 1).pow((n - 1 - k) as u64)
            * combination(n - 1, k, &fact, &fact_inv);
    }
    println!("{result}");
}
