use proconio::input;

fn build_prime_factor_nums(n: usize) -> Vec<u8> {
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
        k: u8,
    }

    let prime_factor_nums = build_prime_factor_nums(n);
    let count = prime_factor_nums.iter().filter(|&&x| x >= k).count();
    println!("{count}");
}
