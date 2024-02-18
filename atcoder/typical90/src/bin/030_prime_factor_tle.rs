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

fn f(n: usize, k: usize) -> usize {
    if k == 1 {
        return n - 1; // 1 以外はすべて条件を満たす
    }

    let primes = build_primes(n);

    let mut products = vec![1usize; k.min(primes.len()) + 1];
    for i in 0..(products.len() - 1) {
        products[i + 1] = products[i] * primes[i];
    }

    let mut count = 0usize;
    let mut stack: Vec<(_, _, &[usize])> = vec![(1usize, 0usize, &primes)];
    while let Some((n0, k0, primes0)) = stack.pop() {
        let Some(&prime) = primes0.last() else { unreachable!(); };

        if k0 + primes0.len() - 1 >= k {
            if n0 * products[k - k0] <= n && primes0.len() > 1 {
                stack.push((n0, k0, &primes0[..(primes0.len() - 1)]));
            }
        }

        if k0 + primes0.len() >= k {
            let mut n0 = n0 * prime;
            let k0 = (k0 + 1).min(k);
            while n0 * products[k - k0] <= n {
                if k0 == k {
                    count += 1;
                }
                if primes0.len() > 1 {
                    stack.push((n0, k0, &primes0[..(primes0.len() - 1)]));
                }
                n0 *= prime;
            }
        }
    }

    count
}

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let count = f(n, k);
    println!("{count}");
}
