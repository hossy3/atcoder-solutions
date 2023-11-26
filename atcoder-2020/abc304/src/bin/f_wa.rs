use itertools::Itertools;
use proconio::{input, marker::Chars};

fn prime_division(mut n: usize) -> Vec<(usize, usize)> {
    let mut result = vec![];
    let mut k = 2;
    while k * k <= n {
        let mut count = 0;
        while n % k == 0 {
            count += 1;
            n /= k;
        }
        if count > 0 {
            result.push((k, count));
        }
        k += if k == 2 { 1 } else { 2 };
    }
    if n > 1 {
        result.push((n, 1));
    }
    result
}

fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    const MOD: usize = 998244353;
    let primes = prime_division(n);
    let mut result = 0usize;

    if primes.iter().any(|&x| x.1 > 1) {
        result = 1;
        for i in 0..primes.len() {
            if primes[i].1 == 1 {
                continue;
            }
            let mut x = 1;
            let m = n / primes[i].0;
            let j = n / m;
            for i0 in 0..m {
                if (0..j).all(|j| s[m * j + i0] == '#') {
                    x = (x * 2) % MOD;
                }
            }
            result = (result + x - 1) % MOD;

            let mut x = 1;
            let m = primes[i].0.pow(primes[i].1 as u32);
            let j = n / m;
            for i0 in 0..m {
                if (0..j).all(|j| s[m * j + i0] == '#') {
                    x = (x * 2) % MOD;
                }
            }
            result = (result + x - 1) % MOD;
        }

        println!("{}", result);
        return;
    }

    for i in 0..primes.len() {
        let mut x = 1;
        for v in (0..primes.len()).combinations(i + 1) {
            let m = v
                .iter()
                .map(|&j| primes[j].0.pow(primes[j].1 as u32))
                .fold(1, |acc, x| acc * x);
            if m == n {
                continue;
            }
            let j = n / m;
            for i0 in 0..m {
                if (0..j).all(|j| s[m * j + i0] == '#') {
                    x = (x * 2) % MOD;
                }
            }
        }
        if i % 2 == 0 {
            result = (result + x) % MOD;
        } else {
            result = (result + MOD - x) % MOD;
        }
    }

    println!("{}", result);
}
