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

// 約数を求める
fn build_factors(primes: &[usize], mut n: usize) -> Vec<usize> {
    let mut v = vec![];
    for &prime in primes {
        let mut count = 0usize;
        while n % prime == 0 {
            n /= prime;
            count += 1;
        }
        if count > 0 {
            v.push((prime, count));
        }
    }
    if n > 1 {
        v.push((n, 1));
    }

    let mut state = vec![1usize];
    for (prime, count) in v {
        let mut new_state = vec![];
        for &s in &state {
            for i in 0..=count {
                new_state.push(s * prime.pow(i as u32));
            }
        }
        state = new_state;
    }

    state.sort_unstable();
    state
}

fn main() {
    input! {
        n: usize,
    }

    let primes = build_primes(n.isqrt());
    let mut result = build_factors(&primes, n - 1).len() - 1; // (n - 1) の 1以外の約数は OK

    // n の 1以外の約数についてシミュレーションする
    let factors = build_factors(&primes, n);
    for &k in &factors[1..] {
        let mut n = n;
        while n % k == 0 {
            n /= k;
        }
        if n % k == 1 {
            result += 1;
        }
    }

    println!("{result}");
}
