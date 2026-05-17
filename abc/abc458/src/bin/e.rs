use proconio::input;

type Mint = ac_library::ModInt998244353;

fn combination(n: usize, r: usize, fact: &[Mint], fact_inv: &[Mint]) -> Mint {
    if n < r {
        return Mint::new(0);
    }
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
        x1: usize,
        x2: usize,
        x3: usize,
    }

    let fact = build_fact(x1 + x2 + x3);
    let fact_inv = build_fact_inv(&fact);

    let mut result = Mint::new(0);

    // 数列を "2" で区切る。 "1" と "3" を配置できる隙間は (x2 + 1) 個。
    for k in 1..=x2 {
        // "2" の配置方法: (x2 + 1) C k
        let count2 = combination(x2 + 1, k, &fact, &fact_inv);

        // "1" の配置方法:
        // k 個の隙間に 1 を最低1個配置すると (x1 - k) 個残る
        // (x1 - k) 個の間にしきりを (k - 1) 個配置することと同じ
        // つまり ((x1 - k) + (k - 1)) C (k - 1)
        let count1 = combination(x1 - 1, k - 1, &fact, &fact_inv);

        // "3" の配置方法:
        // (x2 + 1) 個の隙間から "1" に k 個使ったので、残りは (x2 + 1) - k 個
        // これをしきりと思い、 x3 個の間にしきりを (x2 - k) 個配置することと同じ
        // つまり (x3 x2 - k) C (x2 - k)
        let count3 = combination(x3 + x2 - k, x2 - k, &fact, &fact_inv);

        result += count1 * count2 * count3;
    }

    println!("{result}");
}
