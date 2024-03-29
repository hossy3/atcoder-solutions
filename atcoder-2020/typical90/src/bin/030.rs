use proconio::input;

fn build_prime_nums(n: usize) -> Vec<usize> {
    let mut primes = vec![0; n + 1];
    for i in 2..=n {
        if primes[i] > 0 {
            continue;
        }
        let m = n / i;
        for j in 1..=m {
            primes[i * j] += 1;
        }
    }
    primes
}

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let prime_nums = build_prime_nums(n);
    let count = prime_nums.iter().filter(|&&x| x >= k).count();
    println!("{}", count);
}
