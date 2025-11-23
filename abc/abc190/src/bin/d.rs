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

fn main() {
    input! {
        n: usize,
    }
    let primes = prime_division(2 * n);
    let result = primes
        .iter()
        .filter(|&(k, _)| k % 2 == 1)
        .fold(1, |acc, &(_, n)| acc * (n + 1))
        * 2;
    println!("{result}");
}
