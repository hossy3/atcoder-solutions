use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    const MOD: usize = 998244353;

    let mut primes = vec![];
    {
        let mut n = n;
        let mut k = 2;
        while k * k <= n {
            if n % k == 0 {
                primes.push(k);
                while n % k == 0 {
                    n /= k;
                }
            }
            k += if k == 2 { 1 } else { 2 };
        }
        if n > 1 {
            primes.push(n);
        }
    }

    let mut result = 0usize;
    for k in 0..primes.len() {
        for v in primes.iter().combinations(k + 1) {
            let i = v.iter().fold(n, |acc, &&x| acc / x);
            let mut x = 1;
            let j = n / i;
            for i0 in 0..i {
                if (0..j).all(|j0| s[i * j0 + i0] == '#') {
                    x = (x * 2) % MOD;
                }
            }

            if k % 2 == 0 {
                result = (result + x) % MOD;
            } else {
                result = (result + MOD - x) % MOD;
            }
        }
    }

    println!("{}", result);
}
