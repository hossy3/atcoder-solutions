use num_integer::Roots;
use proconio::input;

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

fn calc_max_prime(mut n: usize) -> usize {
    let primes = build_primes(n.sqrt());
    for i in 0..primes.len() {
        let prime = primes[i];
        while n > 0 && n % prime == 0 {
            n /= prime;
        }
        if n == 1 {
            return prime;
        }
    }
    n
}

fn gcd(a: usize, b: usize) -> usize {
    if a % b == 0 {
        b
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    input! {
        k: usize,
    }
    let last_prime = calc_max_prime(k);
    let mut x = k;
    for i in 2..=k {
        let y = gcd(x, i);
        if x == y {
            println!("{}", i);
            return;
        }
        x /= y;
        if x == last_prime && i < last_prime {
            println!("{}", x);
            break;
        }
    }
}
