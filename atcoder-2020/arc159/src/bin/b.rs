use std::mem::swap;

use proconio::input;

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

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
        mut a: usize,
        mut b: usize,
    }
    if a == b {
        println!("{}", 1);
        return;
    }

    let mut count = 0;
    while a > 0 && b > 0 {
        let x = gcd(a, b);
        if x > 1 {
            a /= x;
            b /= x;
        }
        if a < b {
            swap(&mut a, &mut b);
        }
        let n = a - b;
        let primes = prime_division(n);
        let x = primes.iter().map(|&x| b % x.0).min().unwrap_or(b);
        count += x;
        a -= x;
        b -= x;
    }
    println!("{}", count);
}
