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

fn f(primes: &[usize], k: usize, i: usize, rest: usize) -> usize {
    let mut count = 0;
    for (j, &prime) in primes.iter().enumerate() {
        let mut rest = rest / prime;
        if rest == 0 {
            break;
        }
        while rest > 0 {
            if i + 1 >= k {
                count += 1;
            }
            count += f(&primes[(j + 1)..], k, i + 1, rest);
            rest /= prime;
        }
    }
    count
}

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let primes = build_primes(n);
    let count = f(&primes, k, 0, n);
    println!("{}", count);
}
