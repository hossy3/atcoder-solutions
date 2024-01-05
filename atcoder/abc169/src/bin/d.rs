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
    let max = (n as f64).sqrt() as usize + 1;
    let primes = build_primes(max);

    let mut count = 0;
    let mut rest = n;
    for &x in &primes {
        if rest < x {
            break;
        }
        for i in 1u32.. {
            let y = x.pow(i);
            if rest % y != 0 {
                break;
            }
            count += 1;
            rest /= y;
        }
    }

    if let Some(&x) = primes.last() {
        if rest > x {
            count += 1; // rest is prime
        }
    }

    println!("{count}");
}
