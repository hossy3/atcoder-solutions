use proconio::input;

type Mint = ac_library::ModInt1000000007;

fn combination(n: usize, r: usize, fact: &[Mint], fact_inv: &[Mint]) -> Mint {
    fact[n] * fact_inv[r] * fact_inv[n - r]
}

fn main() {
    input! {
        n: usize,
    }

    let mut fact = vec![Mint::new(1); n + 1];
    for i in 2..=n {
        fact[i] = fact[i - 1] * i;
    }

    let mut fact_inv = vec![Mint::new(1); n + 1];
    for i in 0..=n {
        fact_inv[i] = fact[i].inv();
    }

    for k in 1..=n {
        let result = (1..=(1 + (n - 1) / k)).fold(Mint::new(0), |acc, i| {
            acc + combination(n - (k - 1) * (i - 1), i, &fact, &fact_inv)
        });
        println!("{result}");
    }
}
