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

fn main() {
    input! {
        n: usize,
    }

    let m = (n as f64).sqrt() as usize;
    let primes = build_primes(m); // 最大 m まで素数を探す

    let mut result = 0usize;
    for (i0, &x0) in primes.iter().enumerate() {
        for &x1 in &primes[..i0] {
            if x0 * x0 * x1 * x1 > n {
                break;
            }
            result += 1;
        }
    }
    for &x0 in &primes {
        if x0.pow(8) > n {
            break;
        }
        result += 1;
    }
    println!("{result}");
}
