use std::eprintln;

use proconio::input;

type Mint = ac_library::ModInt998244353;

fn combination(n: usize, r: usize, fact: &[Mint], fact_inv: &[Mint]) -> Mint {
    eprintln!("{n} {r}");
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
        k: usize,
        a: [usize; n],
    }

    let fact = build_fact(n);
    let fact_inv = build_fact_inv(&fact);

    let sum = a.iter().map(|&x| Mint::new(x)).sum::<Mint>();
    let sum2 = a.iter().map(|&x| Mint::new(x).pow(2)).sum::<Mint>();
    let result = sum2 * combination(n - 1, k - 1, &fact, &fact_inv)
        + if k > 1 {
            (sum.pow(2) - sum2) * combination(n - 2, k - 2, &fact, &fact_inv)
        } else {
            Mint::new(0)
        };
    // eprintln!("{}", sum2 * combination(n - 1, k - 1, &fact, &fact_inv));
    // eprintln!(
    //     "{}",
    //     (sum.pow(2) - sum2) * combination(n - 2, k - 2, &fact, &fact_inv)
    // );

    println!("{result}");
}
