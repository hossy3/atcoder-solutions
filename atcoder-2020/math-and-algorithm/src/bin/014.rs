use itertools::Itertools;
use proconio::input;

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

#[test]
fn test_prime_division() {
    assert_eq!(prime_division(2), [(2, 1)]);
    assert_eq!(prime_division(3), [(3, 1)]);
    assert_eq!(prime_division(4), [(2, 2)]);
    assert_eq!(prime_division(5), [(5, 1)]);
    assert_eq!(prime_division(6), [(2, 1), (3, 1)]);
    assert_eq!(prime_division(7), [(7, 1)]);
    assert_eq!(prime_division(8), [(2, 3)]);
    assert_eq!(prime_division(9), [(3, 2)]);
}

fn prime_factorization(n: usize) -> Vec<usize> {
    let a = prime_division(n);
    let mut v = vec![];
    for &(k, count) in &a {
        for _ in 0..count {
            v.push(k);
        }
    }
    v
}

#[test]
fn test_prime_factorization() {
    assert_eq!(prime_factorization(2), [2]);
    assert_eq!(prime_factorization(3), [3]);
    assert_eq!(prime_factorization(4), [2, 2]);
    assert_eq!(prime_factorization(5), [5]);
    assert_eq!(prime_factorization(6), [2, 3]);
    assert_eq!(prime_factorization(7), [7]);
    assert_eq!(prime_factorization(8), [2, 2, 2]);
    assert_eq!(prime_factorization(9), [3, 3]);
}

fn main() {
    input! {
        n: usize,
    }
    let a = prime_factorization(n);
    let result = a.iter().join(" ");
    println!("{}", result);
}
