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
    let primes = build_primes(1_000_000);
    let mut result = 0;
    for (i, &a) in primes.iter().enumerate() {
        if a.pow(5) > n {
            break;
        }
        for (j, &b) in primes[(i + 1)..].iter().enumerate() {
            if a.pow(2) * b.pow(3) > n {
                break;
            }
            for &c in &primes[(i + j + 2)..] {
                if a.pow(2) * b * c.pow(2) > n {
                    break;
                }
                result += 1;
            }
        }
    }
    println!("{}", result);
}
