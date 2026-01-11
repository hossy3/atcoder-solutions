use proconio::{input, marker::Chars};

type Mint = ac_library::ModInt998244353;

fn combination(n: usize, r: usize, fact: &[Mint], fact_inv: &[Mint]) -> Mint {
    fact[n] * fact_inv[r] * fact_inv[n - r]
}

fn ctoi(c: char) -> usize {
    (c as u8 - '0' as u8) as usize
}

fn f(s: &[char]) -> u32 {
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
    let mut x0;
    let mut x1;
    for (i, &c) in s.iter().enumerate() {
        let j = ctoi(c);
        if j == 9 {
            continue;
        }

        x0 = cum[j][i + 1];
        x1 = cum[j + 1][n] - cum[j + 1][i + 1];
        if x1 > 0 {
            result += combination(x0 - 1 + x1, x1 - 1, &fact, &fact_inv);
        }
    }

    result.val()
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn test_name() {
        assert_eq!(f(&"1122".chars().collect_vec()), 5);
        assert_eq!(f(&"2025".chars().collect_vec()), 0);
        assert_eq!(f(&"0777468889971".chars().collect_vec()), 30);
    }
}

fn main() {
    input! {
        s: Chars,
    }
    let result = f(&s);
    println!("{result}");
}
