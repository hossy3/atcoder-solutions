use std::collections::HashMap;

use itertools::Itertools;
use proconio::input;

type Mint = ac_library::ModInt998244353;

fn build_primes(n: usize) -> Vec<usize> {
    let mut primes = vec![true; n + 1];
    primes[0] = false;
    primes[1] = false;
    for i in 2..=n {
        if !primes[i] {
            continue;
        }
        let m = n / i;
        for j in 2..=m {
            primes[i * j] = false;
        }
    }

    let m = primes.iter().filter(|&&b| b).count();
    let mut result = Vec::with_capacity(m);
    for (i, &b) in primes.iter().enumerate() {
        if b {
            result.push(i);
        }
    }
    result
}

fn prime_division(primes: &[usize], mut n: usize) -> Vec<(usize, usize)> {
    let mut result = vec![];
    for &prime in primes {
        let mut count = 0;
        while n % prime == 0 {
            count += 1;
            n /= prime;
        }
        if count > 0 {
            result.push((prime, count));
        }
        if n == 1 {
            break;
        }
    }
    if n > 1 {
        result.push((n, 1));
    }
    result
}

fn main() {
    input! {
        t: usize,
    }

    let primes = build_primes(10_000_000_usize.isqrt());

    for _ in 0..t {
        input! {
            n: usize,
            a: [usize; n],
        }
        let mut v = Vec::with_capacity(n);
        for &x in &a {
            v.push(prime_division(&primes, x));
        }

        let mut map = HashMap::new();
        for primes in &v {
            for &(prime, count) in primes {
                map.entry(prime).or_insert(vec![]).push(count);
            }
        }

        let mut lcm = Mint::new(1);
        for (prime, v) in &mut map {
            v.sort();
            lcm *= Mint::new(*prime).pow(v[v.len() - 1] as u64);
        }

        let mut results = vec![];
        for primes in &v {
            let mut result = lcm;
            for (prime, count) in primes {
                let v0 = map.get(&prime).unwrap();
                let len = v0.len();
                if v0[len - 1] == *count {
                    if v0.len() > 1 {
                        result /= Mint::new(*prime).pow((v0[len - 1] - v0[len - 2]) as u64);
                    } else {
                        result /= Mint::new(*prime).pow(v0[len - 1] as u64);
                    }
                }
            }
            results.push(result.val());
        }

        println!("{}", results.iter().join(" "));
    }
}
