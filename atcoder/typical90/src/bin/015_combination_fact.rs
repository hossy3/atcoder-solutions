use proconio::input;

type Mint = ac_library::ModInt1000000007;

fn combination(n: usize, r: usize, fact: &[Mint]) -> Mint {
    fact[n] / (fact[r] * fact[n - r])
}

fn main() {
    input! {
        n: usize,
    }

    let mut fact = vec![Mint::new(1); n + 1];
    for i in 2..=n {
        fact[i] = fact[i - 1] * i;
    }

    for k in 1..=n {
        let result = (1..=(1 + (n - 1) / k)).fold(Mint::new(0), |acc, i| {
            acc + combination(n - (k - 1) * (i - 1), i, &fact)
        });
        println!("{result}");
    }
}
