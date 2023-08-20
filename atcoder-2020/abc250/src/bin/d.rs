// -*- coding:utf-8-unix -*-

use proconio::input;

fn build_primes(n: usize) -> Vec<usize> {
    let mut primes = vec![2usize];
    'outer: for i in 3.. {
        if 2 * (i as usize).pow(3) > n {
            break;
        }
        for prime in &primes {
            if i % prime == 0 {
                continue 'outer;
            }
            if prime * prime > i {
                primes.push(i);
                continue 'outer;
            }
        }
        panic!("unreachable");
    }
    primes
}

fn main() {
    input! {
        n: usize,
    }
    if n < 18 {
        println!("0"); // min: 2 * 3 * 3 * 3 = 18
        return;
    }
    let primes = build_primes(n);
    let mut count = 0;
    for prime0 in &primes {
        if prime0.pow(4) > n {
            break;
        }
        for prime1 in &primes {
            if prime1 <= prime0 {
                continue;
            }
            if prime0 * prime1.pow(3) > n {
                break;
            }
            count += 1;
        }
    }
    println!("{}", count);
}
