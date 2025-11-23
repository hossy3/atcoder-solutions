use proconio::{input, marker::Chars};

type Mint = ac_library::ModInt998244353;

fn combination(n: usize, r: usize, fact: &[Mint], fact_inv: &[Mint]) -> Mint {
    fact[n] * fact_inv[r] * fact_inv[n - r]
}

fn ctoi(c: char) -> usize {
    (c as u8 - '0' as u8) as usize
}

fn main() {
    input! {
        s: Chars,
    }

    let n = s.len();

    let mut fact = vec![Mint::new(1); n + 1];
    for i in 2..=n {
        fact[i] = fact[i - 1] * i;
    }

    let mut fact_inv = vec![Mint::new(1); n + 1];
    for i in 0..=n {
        fact_inv[i] = fact[i].inv();
    }

    let mut cum = vec![vec![0usize; n + 1]; 10]; // 最大10桁
    for (i, &c) in s.iter().enumerate() {
        let j = ctoi(c);
        cum[j][i + 1] = 1;
    }
    for j in 0..10 {
        for i in 0..n {
            cum[j][i + 1] += cum[j][i];
        }
    }

    let mut result = Mint::new(0);
    for (i, &c) in s.iter().enumerate() {
        let j = ctoi(c);
        if j == 9 || cum[j][i] == cum[j][i + 1] {
            continue;
        }

        let x0 = cum[j][i + 1];
        let x1 = cum[j + 1][n] - cum[j + 1][i + 1];
        for r in 1..=x0.min(x1) {
            result +=
                combination(x0 - 1, r - 1, &fact, &fact_inv) * combination(x1, r, &fact, &fact_inv);
        }
    }

    println!("{result}");
}
